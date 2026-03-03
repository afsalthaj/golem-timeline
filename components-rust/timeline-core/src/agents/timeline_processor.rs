use golem_rust::{agent_definition, agent_implementation};

use common_lib::{GolemEventValue, StateDynamicsTimeLine};

use crate::agents::helpers::*;
use crate::types::*;

#[agent_definition]
pub trait TimelineProcessor {
    fn new(name: String) -> Self;
    fn initialize_derived(&mut self, operation: DerivedOperation, children: Vec<ChildWorkerRef>);
    fn set_parent(&mut self, parent: ParentRef);
    fn set_aggregator(&mut self, aggregator: AggregatorRef);
    async fn on_child_state_changed(&mut self, child_index: u32, time: u64, value: EventValue);
    fn get_derived_result(&self, t1: u64) -> Result<TimelineResult, String>;
}

struct TimelineProcessorImpl {
    _name: String,
    operation: Option<DerivedOperation>,
    children: Vec<ChildWorkerRef>,
    parent: Option<ParentRef>,
    aggregator: Option<AggregatorRef>,
    last_aggregated_value: Option<f64>,
    // Child state storage for binary ops (And/Or)
    left_child_state: StateDynamicsTimeLine<GolemEventValue>,
    right_child_state: StateDynamicsTimeLine<GolemEventValue>,
    // Result state
    result_state: StateDynamicsTimeLine<GolemEventValue>,
    // Duration tracking for TlDurationWhere
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
            aggregator: None,
            last_aggregated_value: None,
            left_child_state: StateDynamicsTimeLine::default(),
            right_child_state: StateDynamicsTimeLine::default(),
            result_state: StateDynamicsTimeLine::default(),
            duration_state: None,
        }
    }

    fn initialize_derived(&mut self, operation: DerivedOperation, children: Vec<ChildWorkerRef>) {
        self.operation = Some(operation);
        self.children = children;
    }

    fn set_parent(&mut self, parent: ParentRef) {
        self.parent = Some(parent);
    }

    fn set_aggregator(&mut self, aggregator: AggregatorRef) {
        self.aggregator = Some(aggregator);
    }

    async fn on_child_state_changed(&mut self, child_index: u32, time: u64, value: EventValue) {
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
                    notify_parent(&self.parent, time, event_val.clone()).await;
                    notify_aggregator(
                        &self.aggregator,
                        &mut self.last_aggregated_value,
                        time,
                        &event_val,
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
                        notify_parent(&self.parent, time, event_val.clone()).await;
                        notify_aggregator(
                            &self.aggregator,
                            &mut self.last_aggregated_value,
                            time,
                            &event_val,
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
                        notify_parent(&self.parent, time, event_val.clone()).await;
                        notify_aggregator(
                            &self.aggregator,
                            &mut self.last_aggregated_value,
                            time,
                            &event_val,
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
                        notify_parent(&self.parent, time, event_val.clone()).await;
                        notify_aggregator(
                            &self.aggregator,
                            &mut self.last_aggregated_value,
                            time,
                            &event_val,
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
                        // Start climbing from current count
                        self.duration_state = Some(DurationState::Climbing {
                            base: current_count,
                            since: time,
                        });
                    } else {
                        // Go flat at current count
                        self.duration_state = Some(DurationState::Flat {
                            value: current_count,
                        });
                    }

                    let result_value = GolemEventValue::IntValue(current_count as i64);
                    self.result_state
                        .add_state_dynamic_info(time, result_value.clone());
                    let event_val = EventValue::IntValue(current_count as i64);
                    notify_parent(&self.parent, time, event_val.clone()).await;
                    notify_aggregator(
                        &self.aggregator,
                        &mut self.last_aggregated_value,
                        time,
                        &event_val,
                    )
                    .await;
                }
            }

            DerivedOperation::DurationInCurState => {
                // Track how long we've been in the current state
                // On any state change, reset to 0 and start climbing
                self.duration_state = Some(DurationState::Climbing {
                    base: 0,
                    since: time,
                });
                let result_value = GolemEventValue::IntValue(0);
                self.result_state.add_state_dynamic_info(time, result_value);
                let event_val = EventValue::IntValue(0);
                notify_parent(&self.parent, time, event_val.clone()).await;
                notify_aggregator(
                    &self.aggregator,
                    &mut self.last_aggregated_value,
                    time,
                    &event_val,
                )
                .await;
            }
        }
    }

    fn get_derived_result(&self, t1: u64) -> Result<TimelineResult, String> {
        let _op = self.operation.as_ref().ok_or("Not initialized")?;

        // For DurationWhere/DurationInCurState, compute the actual value at t1
        // based on whether we're climbing or flat
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
