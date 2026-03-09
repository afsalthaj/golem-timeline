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
///     duration-where-1: DurationWhere(and-2)                    ← root (TimelineProcessor)
///     and-2: And(and-3, equal-to-7)                             (TimelineProcessor)
///     and-3: And(has-existed-4, not-5)                          (TimelineProcessor)
///     has-existed-4: TlHasExisted(playerStateChange == "play")  (EventProcessor LEAF)
///     not-5: Not(has-existed-within-6)                          (TimelineProcessor)
///     has-existed-within-6: TlHasExistedWithin(playerStateChange == "seek", 5) (EventProcessor LEAF)
///     equal-to-7: EqualTo(latest-event-to-state-8, "buffer")   (TimelineProcessor)
///     latest-event-to-state-8: TlLatestEventToState("playerStateChange") (EventProcessor LEAF)
///
///   Then wires aggregation:
///     - All 3 leaves get `set_group_by_column("cdn")` — so they extract "cdn" from events
///     - Root duration-where-1 gets `set_aggregation(group_by_column="cdn", [Count, Sum, Avg])`
///     - No Aggregator is created yet — duration-where-1 lazily creates "aggregator-cdn-akamai"
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
    ///   → tells root duration-where-1 to push deltas to Aggregator::get("aggregator-cdn-{value}")
    ///   → returns "Timeline initialized. Result agent: sess-42-duration-where-1"
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
                let mut client = TimelineProcessorClient::get(result.agent_name.clone());
                client.set_aggregation(agg_config.clone()).await;
            }
        }

        Ok(InitializeResult {
            root_agent: result.agent_name,
            leaf_agents: leaves,
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
            let n = *counter;

            match op {
                common_lib::TimeLineOp::TlLatestEventToState(col) => {
                    let agent_name = format!("{}-latest-event-to-state-{}", self.name, n);
                    let mut ep = EventProcessorClient::get(agent_name.clone());
                    ep.initialize_leaf(LeafOperation::LatestEventToState(col.0.clone()))
                        .await;
                    leaves.push(agent_name.clone());
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: true,
                        },
                        true,
                    ))
                }
                common_lib::TimeLineOp::TlHasExisted(pred) => {
                    let agent_name = format!("{}-has-existed-{}", self.name, n);
                    let mut ep = EventProcessorClient::get(agent_name.clone());
                    ep.initialize_leaf(LeafOperation::TlHasExisted(predicate_to_api(pred)))
                        .await;
                    leaves.push(agent_name.clone());
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: true,
                        },
                        true,
                    ))
                }
                common_lib::TimeLineOp::TlHasExistedWithin(pred, dur) => {
                    let agent_name = format!("{}-has-existed-within-{}", self.name, n);
                    let mut ep = EventProcessorClient::get(agent_name.clone());
                    ep.initialize_leaf(LeafOperation::TlHasExistedWithin(
                        predicate_to_api(pred),
                        *dur,
                    ))
                    .await;
                    leaves.push(agent_name.clone());
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: true,
                        },
                        true,
                    ))
                }
                common_lib::TimeLineOp::EqualTo(child, val) => {
                    let agent_name = format!("{}-equal-to-{}", self.name, n);
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::EqualTo,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildAgentRef {
                            agent_name: child_result.agent_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &agent_name, 0).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::GreaterThan(child, val) => {
                    let agent_name = format!("{}-greater-than-{}", self.name, n);
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::GreaterThan,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildAgentRef {
                            agent_name: child_result.agent_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &agent_name, 0).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::GreaterThanOrEqual(child, val) => {
                    let agent_name = format!("{}-greater-than-or-equal-{}", self.name, n);
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::GreaterThanOrEqual,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildAgentRef {
                            agent_name: child_result.agent_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &agent_name, 0).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::LessThan(child, val) => {
                    let agent_name = format!("{}-less-than-{}", self.name, n);
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::LessThan,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildAgentRef {
                            agent_name: child_result.agent_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &agent_name, 0).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::LessThanOrEqual(child, val) => {
                    let agent_name = format!("{}-less-than-or-equal-{}", self.name, n);
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(
                            CompareOp::LessThanOrEqual,
                            EventValue::from_domain(val),
                        ),
                        vec![ChildAgentRef {
                            agent_name: child_result.agent_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &agent_name, 0).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::Not(child) => {
                    let agent_name = format!("{}-not-{}", self.name, n);
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Negation,
                        vec![ChildAgentRef {
                            agent_name: child_result.agent_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &agent_name, 0).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::And(left, right) => {
                    let agent_name = format!("{}-and-{}", self.name, n);
                    let (left_result, _) =
                        self.setup_node(left, counter, _parent_ref, leaves).await?;
                    let (right_result, _) =
                        self.setup_node(right, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::And,
                        vec![
                            ChildAgentRef {
                                agent_name: left_result.agent_name.clone(),
                                is_leaf: left_result.is_leaf,
                            },
                            ChildAgentRef {
                                agent_name: right_result.agent_name.clone(),
                                is_leaf: right_result.is_leaf,
                            },
                        ],
                    )
                    .await;
                    set_child_parent(&left_result, &agent_name, 0).await;
                    set_child_parent(&right_result, &agent_name, 1).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::Or(left, right) => {
                    let agent_name = format!("{}-or-{}", self.name, n);
                    let (left_result, _) =
                        self.setup_node(left, counter, _parent_ref, leaves).await?;
                    let (right_result, _) =
                        self.setup_node(right, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Or,
                        vec![
                            ChildAgentRef {
                                agent_name: left_result.agent_name.clone(),
                                is_leaf: left_result.is_leaf,
                            },
                            ChildAgentRef {
                                agent_name: right_result.agent_name.clone(),
                                is_leaf: right_result.is_leaf,
                            },
                        ],
                    )
                    .await;
                    set_child_parent(&left_result, &agent_name, 0).await;
                    set_child_parent(&right_result, &agent_name, 1).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::TlDurationWhere(child) => {
                    let agent_name = format!("{}-duration-where-{}", self.name, n);
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::DurationWhere,
                        vec![ChildAgentRef {
                            agent_name: child_result.agent_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &agent_name, 0).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
                common_lib::TimeLineOp::TlDurationInCurState(child) => {
                    let agent_name = format!("{}-duration-in-cur-state-{}", self.name, n);
                    let (child_result, _) =
                        self.setup_node(child, counter, _parent_ref, leaves).await?;
                    let mut tp = TimelineProcessorClient::get(agent_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::DurationInCurState,
                        vec![ChildAgentRef {
                            agent_name: child_result.agent_name.clone(),
                            is_leaf: child_result.is_leaf,
                        }],
                    )
                    .await;
                    set_child_parent(&child_result, &agent_name, 0).await;
                    Ok((
                        SetupResult {
                            agent_name,
                            is_leaf: false,
                        },
                        false,
                    ))
                }
            }
        })
    }
}