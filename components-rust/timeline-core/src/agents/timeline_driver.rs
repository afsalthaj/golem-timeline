// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
use golem_rust::{agent_definition, agent_implementation};

use crate::agents::helpers::*;
use crate::agents::event_processor::EventProcessorClient;
use crate::agents::timeline_processor::TimelineProcessorClient;
use crate::conversions::predicate_to_api;
use crate::types::*;

/// Orchestrator agent. Walks a timeline expression tree, spawns all leaf and
/// derived agents, and wires them together. One driver per session.
///
/// CIRR example for session "sess-42":
///   ```text
///   duration_where(
///     has_existed(playerStateChange == "play")
///     && !has_existed_within(playerStateChange == "seek", 5)
///     && latest_event_to_state(playerStateChange) == "buffer"
///   ) | aggregate(group_by=cdn, count, sum, avg)
///   ```
///
///   The driver spawns 8 agents (pre-order depth-first numbering):
///     node-1: DurationWhere(node-2)                    ← root (TimelineProcessor)
///     node-2: And(node-3, node-7)                      (TimelineProcessor)
///     node-3: And(node-4, node-5)                      (TimelineProcessor)
///     node-4: TlHasExisted(playerStateChange == "play") (EventProcessor LEAF)
///     node-5: Not(node-6)                              (TimelineProcessor)
///     node-6: TlHasExistedWithin(playerStateChange == "seek", 5) (EventProcessor LEAF)
///     node-7: EqualTo(node-8, "buffer")                (TimelineProcessor)
///     node-8: TlLatestEventToState("playerStateChange") (EventProcessor LEAF)
///
///   Then wires aggregation:
///     - All 3 leaves get `set_group_by_column("cdn")` — so they extract "cdn" from events
///     - Root node-1 gets `set_aggregation(group_by_column="cdn", [Count, Sum, Avg])`
///     - No Aggregator is created yet — node-1 lazily creates "aggregator-cdn-akamai"
///       on the first delta push when it receives group_by_value="akamai" from the cascade
#[agent_definition]
pub trait TimelineDriver {
    fn new(name: String) -> Self;

    /// Spawn all agents for a timeline expression and wire them together.
    ///
    /// CIRR example:
    ///   ```text
    ///   TimelineDriver("sess-42").initialize_timeline(
    ///     cirr_graph,
    ///     Some(AggregationConfig { group_by_column: "cdn", aggregations: [Count, Sum, Avg] })
    ///   )
    ///   ```
    ///   → spawns 3 EventProcessors + 5 TimelineProcessors
    ///   → wires parent refs so pushes cascade upward
    ///   → tells leaves to extract the "cdn" column from events
    ///   → tells root node-8 to push deltas to Aggregator::get("aggregator-cdn-{value}")
    ///   → returns "Timeline initialized. Result worker: sess-42-node-8"
    async fn initialize_timeline(
        &self,
        timeline: TimelineOpGraph,
        aggregation: Option<AggregationConfig>,
    ) -> Result<InitializeResult, String>;
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
    ) -> Result<InitializeResult, String> {
        let recursive_op = timeline.to_recursive();
        let mut leaves = Vec::new();
        let (result, _) = self
            .setup_node(&recursive_op, &mut 0, &None, &mut leaves)
            .await?;

        // If aggregation is configured:
        // - Tell every leaf which event column to extract for grouping (e.g., "cdn").
        //   The leaf reads this column from each event and propagates its value up the cascade.
        // - Tell the root TimelineProcessor the aggregation config so it can lazily
        //   create the Aggregator agent on first delta push.
        if let Some(ref agg_config) = aggregation {
            for leaf_name in &leaves {
                let mut client = EventProcessorClient::get(leaf_name.clone());
                client
                    .set_group_by_column(agg_config.group_by_column.clone())
                    .await;
            }

            if !result.is_leaf {
                let mut client = TimelineProcessorClient::get(result.worker_name.clone());
                client.set_aggregation(agg_config.clone()).await;
            }
        }

        Ok(InitializeResult {
            root_worker: result.worker_name,
            leaf_workers: leaves,
        })
    }
}

impl TimelineDriverImpl {
    fn setup_node<'a>(
        &'a self,
        op: &'a common_lib::TimeLineOp,
        counter: &'a mut u64,
        _parent_ref: &'a Option<ParentRef>,
        leaves: &'a mut Vec<String>,
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
                    leaves.push(worker_name.clone());
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
                    leaves.push(worker_name.clone());
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
                    leaves.push(worker_name.clone());
                    Ok((
                        SetupResult {
                            worker_name,
                            is_leaf: true,
                        },
                        true,
                    ))
                }
                common_lib::TimeLineOp::EqualTo(child, val) => {
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
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
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
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
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
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
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
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
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
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
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
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
                    let (left_result, _) =
                        self.setup_node(left, counter, _parent_ref, leaves).await?;
                    let (right_result, _) =
                        self.setup_node(right, counter, _parent_ref, leaves).await?;
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
                    let (left_result, _) =
                        self.setup_node(left, counter, _parent_ref, leaves).await?;
                    let (right_result, _) =
                        self.setup_node(right, counter, _parent_ref, leaves).await?;
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
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
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
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
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