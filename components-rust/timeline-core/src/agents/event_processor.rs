use golem_rust::{agent_definition, agent_implementation};

use common_lib::{EventColumnName, GolemEventValue, StateDynamicsTimeLine};

use crate::agents::helpers::*;
use crate::types::*;

/// Leaf node agent. Ingests raw events and computes a leaf timeline operation.
///
/// CIRR example:
///   ```text
///   duration_where(
///     has_existed(playerStateChange == "play")              ← leaf node-1
///     && !has_existed_within(playerStateChange == "seek", 5) ← leaf node-3
///     && latest_event_to_state(playerStateChange) == "buffer" ← leaf node-5
///   ) | aggregate(group_by=cdn, count, sum, avg)
///   ```
///
/// The feeder sends every raw event to all three leaves. Each leaf evaluates its
/// own operation and pushes state changes up to its parent TimelineProcessor.
///
/// Full push flow for a single event `{ time: 100, playerStateChange: "play", cdn: "akamai" }`:
///   1. node-1 (has_existed): predicate matches → state becomes `true` → pushes `(true, "akamai")` to parent
///   2. node-3 (has_existed_within): predicate doesn't match → no push
///   3. node-5 (latest_event_to_state): state becomes `"play"` → pushes `("play", "akamai")` to parent
///   The `"akamai"` is the group_by column value, extracted from the event and carried through the cascade.
#[agent_definition]
pub trait EventProcessor {
    fn new(name: String) -> Self;

    /// Tell this leaf what to compute.
    ///
    /// CIRR example — three leaves are initialized:
    ///   - node-1: `initialize_leaf(TlHasExisted(playerStateChange == "play"))`
    ///   - node-3: `initialize_leaf(TlHasExistedWithin(playerStateChange == "seek", 5))`
    ///   - node-5: `initialize_leaf(LatestEventToState("playerStateChange"))`
    fn initialize_leaf(&mut self, operation: LeafOperation);

    /// Wire this leaf to its parent TimelineProcessor so state changes push upward.
    ///
    /// CIRR example:
    ///   - node-1 (has_existed "play") → `set_parent(ParentRef { worker_name: "node-2", child_index: 0 })`
    ///     When node-1's state changes, it calls `node-2.on_child_state_changed(0, time, value, group_by)`.
    fn set_parent(&mut self, parent: ParentRef);

    /// Tell this leaf which event column to extract for aggregation grouping.
    /// The extracted value travels up the entire push cascade to the root.
    ///
    /// CIRR example:
    ///   All three leaves get `set_group_by_column("cdn")`.
    ///   When an event `{ playerStateChange: "play", cdn: "akamai" }` arrives,
    ///   the leaf extracts `"akamai"` and includes it in every parent notification.
    fn set_group_by_column(&mut self, column: String);

    /// Ingest a raw event. Evaluates the leaf operation, and if state changed,
    /// pushes `(new_state, group_by_value)` to the parent.
    ///
    /// CIRR example — feeder sends to node-1 (has_existed "play"):
    ///   `add_event({ time: 100, event: [("playerStateChange", "play"), ("cdn", "akamai")] })`
    ///   → predicate matches → state flips to `true`
    ///   → pushes `on_child_state_changed(0, 100, true, Some("akamai"))` to parent node-2
    async fn add_event(&mut self, event: Event) -> Result<String, String>;

    /// Point-in-time query. Returns the precomputed state at time `t1`.
    ///
    /// CIRR example:
    ///   `node-1.get_leaf_result(100)` → `true` (play has existed by time 100)
    fn get_leaf_result(&self, t1: u64) -> Result<TimelineResult, String>;
}

struct EventProcessorImpl {
    _name: String,
    operation: Option<LeafOperation>,
    parent: Option<ParentRef>,
    group_by_column: Option<String>,
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
            group_by_column: None,
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

    fn set_group_by_column(&mut self, column: String) {
        self.group_by_column = Some(column);
    }

    async fn add_event(&mut self, event: Event) -> Result<String, String> {
        let op = self.operation.as_ref().ok_or("Not initialized")?;
        let time = event.time;

        let group_by_value = self.group_by_column.as_ref().and_then(|col| {
            event
                .event
                .iter()
                .find(|(k, _)| k == col)
                .map(|(_, v)| v.clone())
        });

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
                        notify_parent(
                            &self.parent,
                            time,
                            EventValue::from_domain_owned(val),
                            &group_by_value,
                        )
                        .await;
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
                        notify_parent(
                            &self.parent,
                            time,
                            EventValue::BoolValue(true),
                            &group_by_value,
                        )
                        .await;
                    } else if !self.tl_has_existed_state.future_is(false) {
                        self.tl_has_existed_state
                            .add_state_dynamic_info(time, false);
                        notify_parent(
                            &self.parent,
                            time,
                            EventValue::BoolValue(false),
                            &group_by_value,
                        )
                        .await;
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
                        notify_parent(
                            &self.parent,
                            time,
                            EventValue::BoolValue(true),
                            &group_by_value,
                        )
                        .await;
                        self.tl_has_existed_within_state
                            .add_state_dynamic_info(time + within, false);
                        notify_parent(
                            &self.parent,
                            time + within,
                            EventValue::BoolValue(false),
                            &group_by_value,
                        )
                        .await;
                    } else if !self.tl_has_existed_within_state.future_is(false) {
                        self.tl_has_existed_within_state
                            .add_state_dynamic_info(time, false);
                        notify_parent(
                            &self.parent,
                            time,
                            EventValue::BoolValue(false),
                            &group_by_value,
                        )
                        .await;
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
