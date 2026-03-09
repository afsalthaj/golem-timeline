// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
use golem_rust::{agent_definition, agent_implementation};

use common_lib::{GolemEventValue, StateDynamicsTimeLine};

use crate::agents::helpers::*;
use crate::types::*;

/// Derived node agent. Receives state changes from children, recomputes its own
/// state, and pushes upward. The root node (no parent) pushes to the Aggregator.
///
/// CIRR example — the full derived tree for session "sess-42":
///   ```text
///   duration_where(                                         ← node-8 (root, DurationWhere)
///     has_existed(playerStateChange == "play")               ← node-1 (leaf)
///     && !has_existed_within(playerStateChange == "seek", 5) ← node-3 (leaf), node-4 (Not)
///     && latest_event_to_state(playerStateChange) == "buffer" ← node-5 (leaf), node-6 (EqualTo)
///   )                                       node-2 (And: node-1 & node-4)
///                                           node-7 (And: node-2 & node-6)
///   ```
///
/// Push cascade when user starts buffering at time 200 (cdn = "akamai"):
///   1. Leaf node-5 pushes ("buffer", "akamai") → node-6 (EqualTo "buffer") → computes `true`
///   2. node-6 pushes (true, "akamai") → node-7 (And) → both children true → computes `true`
///   3. node-7 pushes (true, "akamai") → node-8 (DurationWhere) → starts counting
///   4. node-8 is root with aggregation config → calls `Aggregator::get("aggregator-cdn-akamai").on_delta(0.0)`
#[agent_definition]
pub trait TimelineProcessor {
    fn new(name: String) -> Self;

    /// Tell this node what to compute and which children to depend on.
    ///
    /// CIRR example:
    ///   - node-2: `initialize_derived(And, [node-1, node-4])` — AND of has_existed and Not(has_existed_within)
    ///   - node-4: `initialize_derived(Negation, [node-3])` — NOT of has_existed_within
    ///   - node-6: `initialize_derived(Comparison(EqualTo, "buffer"), [node-5])` — latest state == "buffer"
    ///   - node-7: `initialize_derived(And, [node-2, node-6])` — all three conditions combined
    ///   - node-8: `initialize_derived(DurationWhere, [node-7])` — cumulative time where all true
    fn initialize_derived(&mut self, operation: DerivedOperation, children: Vec<ChildAgentRef>);

    /// Wire this node to its parent. Not set on the root — the root pushes to the Aggregator.
    ///
    /// CIRR example:
    ///   - node-2 (And) → `set_parent("node-7", child_index: 0)` — feeds into the outer And
    ///   - node-7 (And) → `set_parent("node-8", child_index: 0)` — feeds into DurationWhere
    ///   - node-8 (root) → no set_parent call — it has set_aggregation instead
    fn set_parent(&mut self, parent: ParentRef);

    /// Called only on the root node. Tells it which event column determines
    /// aggregator routing and which functions (count, sum, avg...) to compute.
    /// The root does not pre-create the Aggregator — it lazily creates it on first delta.
    ///
    /// CIRR example:
    ///   `node-8.set_aggregation(AggregationConfig { group_by_column: "cdn", aggregations: [Count, Sum, Avg] })`
    ///   Later, when node-8 receives group_by_value="akamai" from the cascade:
    ///     → constructs aggregator name "aggregator-cdn-akamai"
    ///     → calls `Aggregator::get("aggregator-cdn-akamai").on_delta(delta)`
    fn set_aggregation(&mut self, config: AggregationConfig);

    /// Receive a state change from a child. `group_by_value` is the aggregation
    /// column value (e.g., "akamai") extracted by the leaf and propagated through
    /// every node in the cascade.
    ///
    /// CIRR example — user starts playing at time 100 on CDN "akamai":
    ///   Leaf node-1 (has_existed "play") flips to `true` and pushes to node-2:
    ///     `node-2.on_child_state_changed(0, 100, BoolValue(true), Some(StringValue("akamai")))`
    ///   node-2 (And) recomputes: left=true, right=already true → result=true
    ///   node-2 pushes to node-7, which pushes to node-8, each passing "akamai" along.
    async fn on_child_state_changed(
        &mut self,
        child_index: u32,
        time: u64,
        value: EventValue,
        group_by_value: Option<EventValue>,
    );

    /// Point-in-time query on precomputed state.
    ///
    /// CIRR example:
    ///   `node-8.get_derived_result(250)` → `IntValue(50)` — 50 seconds of connection-induced buffering by time 250
    fn get_derived_result(&self, t1: u64) -> Result<TimelineResult, String>;
}

struct TimelineProcessorImpl {
    _name: String,
    operation: Option<DerivedOperation>,
    children: Vec<ChildAgentRef>,
    parent: Option<ParentRef>,
    aggregation: Option<AggregationConfig>,
    aggregator_agent: Option<String>,
    last_aggregated_value: Option<f64>,
    left_child_state: StateDynamicsTimeLine<GolemEventValue>,
    right_child_state: StateDynamicsTimeLine<GolemEventValue>,
    result_state: StateDynamicsTimeLine<GolemEventValue>,
    duration_state: Option<DurationState>,
}

#[agent_implementation]
impl TimelineProcessor for TimelineProcessorImpl {
    fn new(name: String) -> Self {
        Self {
            _name: name,
            operation: None,
            children: Vec::new(),
            parent: None,
            aggregation: None,
            aggregator_agent: None,
            last_aggregated_value: None,
            left_child_state: StateDynamicsTimeLine::default(),
            right_child_state: StateDynamicsTimeLine::default(),
            result_state: StateDynamicsTimeLine::default(),
            duration_state: None,
        }
    }

    fn initialize_derived(&mut self, operation: DerivedOperation, children: Vec<ChildAgentRef>) {
        self.operation = Some(operation);
        self.children = children;
    }

    fn set_parent(&mut self, parent: ParentRef) {
        self.parent = Some(parent);
    }

    fn set_aggregation(&mut self, config: AggregationConfig) {
        self.aggregation = Some(config);
    }

    async fn on_child_state_changed(
        &mut self,
        child_index: u32,
        time: u64,
        value: EventValue,
        group_by_value: Option<EventValue>,
    ) {
        let op = match self.operation.as_ref() {
            Some(op) => op.clone(),
            None => return,
        };

        let domain_value = value.to_domain();

        match &op {
            DerivedOperation::Comparison(compare_op, constant) => {
                let constant = constant.to_domain();
                let result = match compare_op {
                    CompareOp::EqualTo => domain_value == constant,
                    CompareOp::GreaterThan => domain_value > constant,
                    CompareOp::GreaterThanOrEqual => domain_value >= constant,
                    CompareOp::LessThan => domain_value < constant,
                    CompareOp::LessThanOrEqual => domain_value <= constant,
                };
                let prev = self
                    .result_state
                    .get_state_at(time)
                    .map(|p| p.value.clone());
                let new_val = GolemEventValue::BoolValue(result);
                if prev.as_ref() != Some(&new_val) {
                    self.result_state.add_state_dynamic_info(time, new_val);
                    let event_val = EventValue::BoolValue(result);
                    notify_parent(&self.parent, time, event_val.clone(), &group_by_value).await;
                    notify_aggregator(
                        &self.aggregation,
                        &mut self.aggregator_agent,
                        &mut self.last_aggregated_value,
                        time,
                        &event_val,
                        &group_by_value,
                    )
                    .await;
                }
            }

            DerivedOperation::Negation => {
                if let Some(b) = domain_value.get_bool() {
                    let negated = !b;
                    let prev = self
                        .result_state
                        .get_state_at(time)
                        .map(|p| p.value.clone());
                    let new_val = GolemEventValue::BoolValue(negated);
                    if prev.as_ref() != Some(&new_val) {
                        self.result_state.add_state_dynamic_info(time, new_val);
                        let event_val = EventValue::BoolValue(negated);
                        notify_parent(&self.parent, time, event_val.clone(), &group_by_value)
                            .await;
                        notify_aggregator(
                            &self.aggregation,
                            &mut self.aggregator_agent,
                            &mut self.last_aggregated_value,
                            time,
                            &event_val,
                            &group_by_value,
                        )
                        .await;
                    }
                }
            }

            DerivedOperation::And => {
                if child_index == 0 {
                    self.left_child_state
                        .add_state_dynamic_info(time, domain_value);
                } else {
                    self.right_child_state
                        .add_state_dynamic_info(time, domain_value);
                }
                let left_val = self
                    .left_child_state
                    .get_state_at(time + 1)
                    .and_then(|p| p.value.get_bool());
                let right_val = self
                    .right_child_state
                    .get_state_at(time + 1)
                    .and_then(|p| p.value.get_bool());
                if let (Some(l), Some(r)) = (left_val, right_val) {
                    let result = l && r;
                    let prev = self
                        .result_state
                        .get_state_at(time)
                        .map(|p| p.value.clone());
                    let new_val = GolemEventValue::BoolValue(result);
                    if prev.as_ref() != Some(&new_val) {
                        self.result_state.add_state_dynamic_info(time, new_val);
                        let event_val = EventValue::BoolValue(result);
                        notify_parent(&self.parent, time, event_val.clone(), &group_by_value)
                            .await;
                        notify_aggregator(
                            &self.aggregation,
                            &mut self.aggregator_agent,
                            &mut self.last_aggregated_value,
                            time,
                            &event_val,
                            &group_by_value,
                        )
                        .await;
                    }
                }
            }

            DerivedOperation::Or => {
                if child_index == 0 {
                    self.left_child_state
                        .add_state_dynamic_info(time, domain_value);
                } else {
                    self.right_child_state
                        .add_state_dynamic_info(time, domain_value);
                }
                let left_val = self
                    .left_child_state
                    .get_state_at(time + 1)
                    .and_then(|p| p.value.get_bool());
                let right_val = self
                    .right_child_state
                    .get_state_at(time + 1)
                    .and_then(|p| p.value.get_bool());
                if let (Some(l), Some(r)) = (left_val, right_val) {
                    let result = l || r;
                    let prev = self
                        .result_state
                        .get_state_at(time)
                        .map(|p| p.value.clone());
                    let new_val = GolemEventValue::BoolValue(result);
                    if prev.as_ref() != Some(&new_val) {
                        self.result_state.add_state_dynamic_info(time, new_val);
                        let event_val = EventValue::BoolValue(result);
                        notify_parent(&self.parent, time, event_val.clone(), &group_by_value)
                            .await;
                        notify_aggregator(
                            &self.aggregation,
                            &mut self.aggregator_agent,
                            &mut self.last_aggregated_value,
                            time,
                            &event_val,
                            &group_by_value,
                        )
                        .await;
                    }
                }
            }

            DerivedOperation::DurationWhere => {
                if let Some(b) = domain_value.get_bool() {
                    let current_count = match &self.duration_state {
                        Some(DurationState::Climbing { base, since }) => base + (time - since),
                        Some(DurationState::Flat { value }) => *value,
                        None => 0,
                    };

                    if b {
                        self.duration_state = Some(DurationState::Climbing {
                            base: current_count,
                            since: time,
                        });
                    } else {
                        self.duration_state = Some(DurationState::Flat {
                            value: current_count,
                        });
                    }

                    let result_value = GolemEventValue::IntValue(current_count as i64);
                    self.result_state
                        .add_state_dynamic_info(time, result_value.clone());
                    let event_val = EventValue::IntValue(current_count as i64);
                    notify_parent(&self.parent, time, event_val.clone(), &group_by_value).await;
                    notify_aggregator(
                        &self.aggregation,
                        &mut self.aggregator_agent,
                        &mut self.last_aggregated_value,
                        time,
                        &event_val,
                        &group_by_value,
                    )
                    .await;
                }
            }

            DerivedOperation::DurationInCurState => {
                self.duration_state = Some(DurationState::Climbing {
                    base: 0,
                    since: time,
                });
                let result_value = GolemEventValue::IntValue(0);
                self.result_state.add_state_dynamic_info(time, result_value);
                let event_val = EventValue::IntValue(0);
                notify_parent(&self.parent, time, event_val.clone(), &group_by_value).await;
                notify_aggregator(
                    &self.aggregation,
                    &mut self.aggregator_agent,
                    &mut self.last_aggregated_value,
                    time,
                    &event_val,
                    &group_by_value,
                )
                .await;
            }
        }
    }

    fn get_derived_result(&self, t1: u64) -> Result<TimelineResult, String> {
        let _op = self.operation.as_ref().ok_or("Not initialized")?;

        if let Some(ref dur) = self.duration_state {
            let value = match dur {
                DurationState::Climbing { base, since } => {
                    if t1 >= *since {
                        base + (t1 - since)
                    } else {
                        *base
                    }
                }
                DurationState::Flat { value } => *value,
            };
            return Ok(TimelineResult {
                results: vec![TimelineResultPoint {
                    t1,
                    t2: None,
                    value: EventValue::IntValue(value as i64),
                }],
            });
        }

        Ok(state_to_result(
            self.result_state
                .get_state_at(t1)
                .map(|s| (s.t1, s.t2, EventValue::from_domain(&s.value))),
        ))
    }
}