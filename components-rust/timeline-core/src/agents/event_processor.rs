use golem_rust::{agent_definition, agent_implementation};

use common_lib::{EventColumnName, GolemEventValue, StateDynamicsTimeLine};

use crate::agents::helpers::*;
use crate::types::*;

#[agent_definition]
pub trait EventProcessor {
    fn new(name: String) -> Self;
    fn initialize_leaf(&mut self, operation: LeafOperation);
    fn set_parent(&mut self, parent: ParentRef);
    async fn add_event(&mut self, event: Event) -> Result<String, String>;
    fn get_leaf_result(&self, t1: u64) -> Result<TimelineResult, String>;
}

struct EventProcessorImpl {
    _name: String,
    operation: Option<LeafOperation>,
    parent: Option<ParentRef>,
    latest_event_state: StateDynamicsTimeLine<GolemEventValue>,
    tl_has_existed_state: StateDynamicsTimeLine<bool>,
    tl_has_existed_within_state: StateDynamicsTimeLine<bool>,
}

#[agent_implementation]
impl EventProcessor for EventProcessorImpl {
    fn new(name: String) -> Self {
        Self {
            _name: name,
            operation: None,
            parent: None,
            latest_event_state: StateDynamicsTimeLine::default(),
            tl_has_existed_state: StateDynamicsTimeLine::default(),
            tl_has_existed_within_state: StateDynamicsTimeLine::default(),
        }
    }

    fn initialize_leaf(&mut self, operation: LeafOperation) {
        self.operation = Some(operation);
    }

    fn set_parent(&mut self, parent: ParentRef) {
        self.parent = Some(parent);
    }

    async fn add_event(&mut self, event: Event) -> Result<String, String> {
        let op = self.operation.as_ref().ok_or("Not initialized")?;
        let time = event.time;
        let mut domain_event = event.into_domain();

        match op {
            LeafOperation::LatestEventToState(col_name) => {
                let col = EventColumnName(col_name.clone());
                if let Some(val) = domain_event.event.remove(&col) {
                    let previous = self
                        .latest_event_state
                        .get_state_at(time)
                        .map(|p| p.value.clone());
                    self.latest_event_state
                        .add_state_dynamic_info(time, val.clone());
                    if previous.as_ref() != Some(&val) {
                        notify_parent(&self.parent, time, EventValue::from_domain_owned(val)).await;
                    }
                }
            }
            LeafOperation::TlHasExisted(pred) => {
                let predicate = pred.to_domain();
                if self.tl_has_existed_state.is_empty()
                    || self.tl_has_existed_state.future_is(false)
                {
                    let result = predicate.evaluate(&domain_event);
                    if result {
                        self.tl_has_existed_state.add_state_dynamic_info(time, true);
                        notify_parent(&self.parent, time, EventValue::BoolValue(true)).await;
                    } else if !self.tl_has_existed_state.future_is(false) {
                        self.tl_has_existed_state
                            .add_state_dynamic_info(time, false);
                        notify_parent(&self.parent, time, EventValue::BoolValue(false)).await;
                    }
                }
            }
            LeafOperation::TlHasExistedWithin(pred, within) => {
                let predicate = pred.to_domain();
                if self.tl_has_existed_within_state.is_empty()
                    || self.tl_has_existed_within_state.future_is(false)
                {
                    let result = predicate.evaluate(&domain_event);
                    if result {
                        self.tl_has_existed_within_state
                            .add_state_dynamic_info(time, true);
                        notify_parent(&self.parent, time, EventValue::BoolValue(true)).await;
                        self.tl_has_existed_within_state
                            .add_state_dynamic_info(time + within, false);
                        notify_parent(&self.parent, time + within, EventValue::BoolValue(false))
                            .await;
                    } else if !self.tl_has_existed_within_state.future_is(false) {
                        self.tl_has_existed_within_state
                            .add_state_dynamic_info(time, false);
                        notify_parent(&self.parent, time, EventValue::BoolValue(false)).await;
                    }
                }
            }
        }

        Ok("Event tracked".to_string())
    }

    fn get_leaf_result(&self, t1: u64) -> Result<TimelineResult, String> {
        let op = self.operation.as_ref().ok_or("Not initialized")?;

        match op {
            LeafOperation::LatestEventToState(_) => Ok(state_to_result(
                self.latest_event_state
                    .get_state_at(t1)
                    .map(|s| (s.t1, s.t2, EventValue::from_domain(&s.value))),
            )),
            LeafOperation::TlHasExisted(_) => Ok(state_to_result(
                self.tl_has_existed_state
                    .get_state_at(t1)
                    .map(|s| (s.t1, s.t2, EventValue::BoolValue(s.value))),
            )),
            LeafOperation::TlHasExistedWithin(_, _) => Ok(state_to_result(
                self.tl_has_existed_within_state
                    .get_state_at(t1)
                    .map(|s| (s.t1, s.t2, EventValue::BoolValue(s.value))),
            )),
        }
    }
}
