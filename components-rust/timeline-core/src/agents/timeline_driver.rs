use golem_rust::{agent_definition, agent_implementation};

use crate::agents::helpers::*;
use crate::agents::event_processor::EventProcessorClient;
use crate::agents::timeline_processor::TimelineProcessorClient;
use crate::agents::aggregator::AggregatorClient;
use crate::conversions::predicate_to_api;
use crate::types::*;

#[agent_definition]
pub trait TimelineDriver {
    fn new(name: String) -> Self;
    async fn initialize_timeline(
        &self,
        timeline: TimelineOpGraph,
        aggregation: Option<AggregationConfig>,
    ) -> Result<String, String>;
}

struct TimelineDriverImpl {
    name: String,
}

#[agent_implementation]
impl TimelineDriver for TimelineDriverImpl {
    fn new(name: String) -> Self {
        Self { name }
    }

    async fn initialize_timeline(
        &self,
        timeline: TimelineOpGraph,
        aggregation: Option<AggregationConfig>,
    ) -> Result<String, String> {
        let recursive_op = timeline.to_recursive();
        let (result, _) = self.setup_node(&recursive_op, &mut 0, &None).await?;

        // Wire root node to aggregator if aggregation config is provided
        if let Some(agg_config) = aggregation {
            let aggregator_name = format!("aggregator-{}", agg_config.group_by_value);
            let mut agg_client = AggregatorClient::get(aggregator_name.clone());
            agg_client
                .initialize_aggregator(agg_config.aggregations)
                .await;
            agg_client.register_session().await;

            let agg_ref = AggregatorRef {
                worker_name: aggregator_name,
            };

            if result.is_leaf {
                // Root is a leaf — a single-node expression like TlLatestEventToState.
                // Aggregation on raw leaf values is unusual; skip for now.
            } else {
                let mut client = TimelineProcessorClient::get(result.worker_name.clone());
                client.set_aggregator(agg_ref).await;
            }
        }

        Ok(format!(
            "Timeline initialized. Result worker: {}",
            result.worker_name
        ))
    }
}

impl TimelineDriverImpl {
    /// Recursively set up agents for each node in the timeline expression tree.
    /// `parent_ref` is passed to leaf nodes that are direct children of derived nodes
    /// that don't need a separate TimelineProcessor (not used currently — parent wiring
    /// happens after creation via `set_child_parent`).
    fn setup_node<'a>(
        &'a self,
        op: &'a common_lib::TimeLineOp,
        counter: &'a mut u64,
        _parent_ref: &'a Option<ParentRef>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(SetupResult, bool), String>> + 'a>,
    > {
        Box::pin(async move {
            *counter += 1;
            let worker_name = format!("{}-node-{}", self.name, counter);

            match op {
                common_lib::TimeLineOp::TlLatestEventToState(col) => {
                    let mut ep = EventProcessorClient::get(worker_name.clone());
                    ep.initialize_leaf(LeafOperation::LatestEventToState(col.0.clone()))
                        .await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: true,
                        },
                        true,
                    ))
                }
                common_lib::TimeLineOp::TlHasExisted(pred) => {
                    let mut ep = EventProcessorClient::get(worker_name.clone());
                    ep.initialize_leaf(LeafOperation::TlHasExisted(predicate_to_api(pred)))
                        .await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: true,
                        },
                        true,
                    ))
                }
                common_lib::TimeLineOp::TlHasExistedWithin(pred, dur) => {
                    let mut ep = EventProcessorClient::get(worker_name.clone());
                    ep.initialize_leaf(LeafOperation::TlHasExistedWithin(
                        predicate_to_api(pred),
                        *dur,
                    ))
                    .await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: true,
                        },
                        true,
                    ))
                }
                common_lib::TimeLineOp::EqualTo(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::EqualTo,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildWorkerRef {
                            worker_name: child_result.worker_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::GreaterThan(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::GreaterThan,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildWorkerRef {
                            worker_name: child_result.worker_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::GreaterThanOrEqual(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::GreaterThanOrEqual,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildWorkerRef {
                            worker_name: child_result.worker_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::LessThan(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::LessThan,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildWorkerRef {
                            worker_name: child_result.worker_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::LessThanOrEqual(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::LessThanOrEqual,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildWorkerRef {
                            worker_name: child_result.worker_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::Not(child) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Negation,
                        vec![ChildWorkerRef {
                            worker_name: child_result.worker_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::And(left, right) => {
                    let (left_result, _) = self.setup_node(left, counter, _parent_ref).await?;
                    let (right_result, _) = self.setup_node(right, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::And,
                        vec![
                            ChildWorkerRef {
                                worker_name: left_result.worker_name.clone(),
                                is_leaf: left_result.is_leaf,
                            },
                            ChildWorkerRef {
                                worker_name: right_result.worker_name.clone(),
                                is_leaf: right_result.is_leaf,
                            },
                        ],
                    )
                    .await;
                    set_child_parent(&left_result, &worker_name, 0).await;
                    set_child_parent(&right_result, &worker_name, 1).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::Or(left, right) => {
                    let (left_result, _) = self.setup_node(left, counter, _parent_ref).await?;
                    let (right_result, _) = self.setup_node(right, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Or,
                        vec![
                            ChildWorkerRef {
                                worker_name: left_result.worker_name.clone(),
                                is_leaf: left_result.is_leaf,
                            },
                            ChildWorkerRef {
                                worker_name: right_result.worker_name.clone(),
                                is_leaf: right_result.is_leaf,
                            },
                        ],
                    )
                    .await;
                    set_child_parent(&left_result, &worker_name, 0).await;
                    set_child_parent(&right_result, &worker_name, 1).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::TlDurationWhere(child) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::DurationWhere,
                        vec![ChildWorkerRef {
                            worker_name: child_result.worker_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::TlDurationInCurState(child) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::DurationInCurState,
                        vec![ChildWorkerRef {
                            worker_name: child_result.worker_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
            }
        })
    }
}
