use golem_rust::{agent_definition, agent_implementation, Schema};

use common_lib::{
    EventColumnName, GolemEvent, GolemEventPredicate, GolemEventValue, StateDynamicsTimeLine,
};

// ============================================================================
// Non-recursive types for the API boundary (Schema-derivable)
// ============================================================================

/// A node index into the flat `TimelineOpGraph.nodes` array.
type NodeIndex = i64;

/// Non-recursive event value — the leaf data type in timeline expressions.
#[derive(Clone, Debug, Schema)]
pub enum EventValue {
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BoolValue(bool),
}

/// Comparison operators for comparing a timeline against a constant.
#[derive(Clone, Debug, Schema)]
pub enum CompareOp {
    EqualTo,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

/// Predicate operator for filtering events.
#[derive(Clone, Debug, Schema)]
pub enum PredicateOp {
    Equal,
    GreaterThan,
    LessThan,
}

/// A single event predicate: column op value.
#[derive(Clone, Debug, Schema)]
pub struct EventPredicate {
    pub col_name: String,
    pub value: EventValue,
    pub op: PredicateOp,
}

/// A single node in the non-recursive timeline graph.
/// Child references are `NodeIndex` values pointing into the same `Vec<TimelineNode>`.
#[derive(Clone, Debug, Schema)]
pub enum TimelineNode {
    /// Compare a child timeline against a constant value
    Comparison(CompareOp, NodeIndex, EventValue),
    /// Negate a child timeline
    Negation(NodeIndex),
    /// AND two child timelines
    And(NodeIndex, NodeIndex),
    /// OR two child timelines
    Or(NodeIndex, NodeIndex),
    /// Leaf: has the predicate ever been true?
    TlHasExisted(EventPredicate),
    /// Leaf: has the predicate been true within `duration` time units?
    TlHasExistedWithin(EventPredicate, u64),
    /// Leaf: track the latest event value for a column as state
    TlLatestEventToState(String),
    /// Cumulative duration where child timeline is true
    TlDurationWhere(NodeIndex),
    /// Duration in current state of child timeline
    TlDurationInCurState(NodeIndex),
}

/// A non-recursive timeline expression encoded as a flat graph.
/// `nodes[0]` is the root.
#[derive(Clone, Debug, Schema)]
pub struct TimelineOpGraph {
    pub nodes: Vec<TimelineNode>,
}

/// A single point in a timeline result.
#[derive(Clone, Debug, Schema)]
pub struct TimelineResultPoint {
    pub t1: u64,
    pub t2: Option<u64>,
    pub value: EventValue,
}

/// The result of querying a timeline at a point in time.
#[derive(Clone, Debug, Schema)]
pub struct TimelineResult {
    pub results: Vec<TimelineResultPoint>,
}

/// An event to be ingested by an event processor.
#[derive(Clone, Debug, Schema)]
pub struct Event {
    pub time: u64,
    pub event: Vec<(String, EventValue)>,
}

/// Which leaf computation this event processor is tracking.
#[derive(Clone, Debug, Schema)]
pub enum LeafOperation {
    LatestEventToState(String),
    TlHasExisted(EventPredicate),
    TlHasExistedWithin(EventPredicate, u64),
}

/// Which derived computation this timeline processor performs.
#[derive(Clone, Debug, Schema)]
pub enum DerivedOperation {
    Comparison(CompareOp, EventValue),
    Negation,
    And,
    Or,
    DurationWhere,
    DurationInCurState,
}

/// Identifies a child agent to fetch results from.
#[derive(Clone, Debug, Schema)]
pub struct ChildWorkerRef {
    pub worker_name: String,
    pub is_leaf: bool,
}

/// Reference to a parent agent that should be notified on state changes.
#[derive(Clone, Debug, Schema)]
pub struct ParentRef {
    pub worker_name: String,
    pub child_index: u32,
}

/// State representing a climbing or flat duration counter.
#[derive(Clone, Debug, Schema)]
pub enum DurationState {
    /// Counter is climbing: base + (t - since)
    Climbing { base: u64, since: u64 },
    /// Counter is flat at this value
    Flat { value: u64 },
}

// ============================================================================
// Conversions between API boundary types and internal domain types
// ============================================================================

impl EventValue {
    fn to_domain(&self) -> GolemEventValue {
        match self {
            EventValue::StringValue(s) => GolemEventValue::StringValue(s.clone()),
            EventValue::IntValue(i) => GolemEventValue::IntValue(*i),
            EventValue::FloatValue(f) => GolemEventValue::FloatValue(*f),
            EventValue::BoolValue(b) => GolemEventValue::BoolValue(*b),
        }
    }

    fn into_domain(self) -> GolemEventValue {
        match self {
            EventValue::StringValue(s) => GolemEventValue::StringValue(s),
            EventValue::IntValue(i) => GolemEventValue::IntValue(i),
            EventValue::FloatValue(f) => GolemEventValue::FloatValue(f),
            EventValue::BoolValue(b) => GolemEventValue::BoolValue(b),
        }
    }

    fn from_domain(v: &GolemEventValue) -> Self {
        match v {
            GolemEventValue::StringValue(s) => EventValue::StringValue(s.clone()),
            GolemEventValue::IntValue(i) => EventValue::IntValue(*i),
            GolemEventValue::FloatValue(f) => EventValue::FloatValue(*f),
            GolemEventValue::BoolValue(b) => EventValue::BoolValue(*b),
        }
    }

    fn from_domain_owned(v: GolemEventValue) -> Self {
        match v {
            GolemEventValue::StringValue(s) => EventValue::StringValue(s),
            GolemEventValue::IntValue(i) => EventValue::IntValue(i),
            GolemEventValue::FloatValue(f) => EventValue::FloatValue(f),
            GolemEventValue::BoolValue(b) => EventValue::BoolValue(b),
        }
    }
}

impl EventPredicate {
    fn to_domain(&self) -> GolemEventPredicate<GolemEventValue> {
        let col = EventColumnName(self.col_name.clone());
        let val = common_lib::EventColumnValue(self.value.to_domain());
        match self.op {
            PredicateOp::Equal => GolemEventPredicate::Equals(col, val),
            PredicateOp::GreaterThan => GolemEventPredicate::GreaterThan(col, val),
            PredicateOp::LessThan => GolemEventPredicate::LessThan(col, val),
        }
    }
}

impl Event {
    fn into_domain(self) -> GolemEvent<GolemEventValue> {
        let map = self
            .event
            .into_iter()
            .map(|(k, v)| (EventColumnName(k), v.into_domain()))
            .collect();
        GolemEvent {
            time: self.time,
            event: map,
        }
    }
}

impl TimelineOpGraph {
    /// Convert the flat graph back to the recursive `TimeLineOp`.
    pub fn to_recursive(&self) -> common_lib::TimeLineOp {
        self.build_node(0)
    }

    fn build_node(&self, idx: usize) -> common_lib::TimeLineOp {
        match &self.nodes[idx] {
            TimelineNode::Comparison(op, child, value) => {
                let child_op = self.build_node(*child as usize);
                let v = value.to_domain();
                match op {
                    CompareOp::EqualTo => common_lib::TimeLineOp::EqualTo(Box::new(child_op), v),
                    CompareOp::GreaterThan => {
                        common_lib::TimeLineOp::GreaterThan(Box::new(child_op), v)
                    }
                    CompareOp::GreaterThanOrEqual => {
                        common_lib::TimeLineOp::GreaterThanOrEqual(Box::new(child_op), v)
                    }
                    CompareOp::LessThan => common_lib::TimeLineOp::LessThan(Box::new(child_op), v),
                    CompareOp::LessThanOrEqual => {
                        common_lib::TimeLineOp::LessThanOrEqual(Box::new(child_op), v)
                    }
                }
            }
            TimelineNode::Negation(child) => {
                common_lib::TimeLineOp::Not(Box::new(self.build_node(*child as usize)))
            }
            TimelineNode::And(l, r) => common_lib::TimeLineOp::And(
                Box::new(self.build_node(*l as usize)),
                Box::new(self.build_node(*r as usize)),
            ),
            TimelineNode::Or(l, r) => common_lib::TimeLineOp::Or(
                Box::new(self.build_node(*l as usize)),
                Box::new(self.build_node(*r as usize)),
            ),
            TimelineNode::TlHasExisted(pred) => {
                common_lib::TimeLineOp::TlHasExisted(pred.to_domain())
            }
            TimelineNode::TlHasExistedWithin(pred, dur) => {
                common_lib::TimeLineOp::TlHasExistedWithin(pred.to_domain(), *dur)
            }
            TimelineNode::TlLatestEventToState(col) => {
                common_lib::TimeLineOp::TlLatestEventToState(EventColumnName(col.clone()))
            }
            TimelineNode::TlDurationWhere(child) => {
                common_lib::TimeLineOp::TlDurationWhere(Box::new(self.build_node(*child as usize)))
            }
            TimelineNode::TlDurationInCurState(child) => {
                common_lib::TimeLineOp::TlDurationInCurState(Box::new(
                    self.build_node(*child as usize),
                ))
            }
        }
    }
}

/// Build a `TimelineOpGraph` from a recursive `TimeLineOp`.
pub fn to_graph(op: &common_lib::TimeLineOp) -> TimelineOpGraph {
    let mut nodes = Vec::new();
    build_graph_node(op, &mut nodes);
    TimelineOpGraph { nodes }
}

fn build_graph_node(op: &common_lib::TimeLineOp, nodes: &mut Vec<TimelineNode>) -> NodeIndex {
    match op {
        common_lib::TimeLineOp::EqualTo(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(
                CompareOp::EqualTo,
                -1,
                EventValue::from_domain(v),
            ));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] {
                *c = child_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::GreaterThan(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(
                CompareOp::GreaterThan,
                -1,
                EventValue::from_domain(v),
            ));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] {
                *c = child_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::GreaterThanOrEqual(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(
                CompareOp::GreaterThanOrEqual,
                -1,
                EventValue::from_domain(v),
            ));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] {
                *c = child_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::LessThan(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(
                CompareOp::LessThan,
                -1,
                EventValue::from_domain(v),
            ));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] {
                *c = child_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::LessThanOrEqual(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(
                CompareOp::LessThanOrEqual,
                -1,
                EventValue::from_domain(v),
            ));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] {
                *c = child_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::Not(child) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Negation(-1));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Negation(ref mut c) = nodes[parent_idx] {
                *c = child_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::And(l, r) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::And(-1, -1));
            let l_idx = build_graph_node(l, nodes);
            let r_idx = build_graph_node(r, nodes);
            if let TimelineNode::And(ref mut li, ref mut ri) = nodes[parent_idx] {
                *li = l_idx;
                *ri = r_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::Or(l, r) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Or(-1, -1));
            let l_idx = build_graph_node(l, nodes);
            let r_idx = build_graph_node(r, nodes);
            if let TimelineNode::Or(ref mut li, ref mut ri) = nodes[parent_idx] {
                *li = l_idx;
                *ri = r_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::TlHasExisted(pred) => {
            let idx = nodes.len();
            nodes.push(TimelineNode::TlHasExisted(predicate_to_api(pred)));
            idx as NodeIndex
        }
        common_lib::TimeLineOp::TlHasExistedWithin(pred, dur) => {
            let idx = nodes.len();
            nodes.push(TimelineNode::TlHasExistedWithin(
                predicate_to_api(pred),
                *dur,
            ));
            idx as NodeIndex
        }
        common_lib::TimeLineOp::TlLatestEventToState(col) => {
            let idx = nodes.len();
            nodes.push(TimelineNode::TlLatestEventToState(col.0.clone()));
            idx as NodeIndex
        }
        common_lib::TimeLineOp::TlDurationWhere(child) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::TlDurationWhere(-1));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::TlDurationWhere(ref mut c) = nodes[parent_idx] {
                *c = child_idx;
            }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::TlDurationInCurState(child) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::TlDurationInCurState(-1));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::TlDurationInCurState(ref mut c) = nodes[parent_idx] {
                *c = child_idx;
            }
            parent_idx as NodeIndex
        }
    }
}

fn predicate_to_api(pred: &GolemEventPredicate<GolemEventValue>) -> EventPredicate {
    match pred {
        GolemEventPredicate::Equals(col, val) => EventPredicate {
            col_name: col.0.clone(),
            value: EventValue::from_domain(&val.0),
            op: PredicateOp::Equal,
        },
        GolemEventPredicate::GreaterThan(col, val) => EventPredicate {
            col_name: col.0.clone(),
            value: EventValue::from_domain(&val.0),
            op: PredicateOp::GreaterThan,
        },
        GolemEventPredicate::LessThan(col, val) => EventPredicate {
            col_name: col.0.clone(),
            value: EventValue::from_domain(&val.0),
            op: PredicateOp::LessThan,
        },
        _ => panic!("Compound predicates (And/Or) not yet supported in API encoding"),
    }
}

// ============================================================================
// Agents — Push Model
//
// Events flow in at leaf nodes, state transitions propagate up the tree,
// and every query is a point lookup on precomputed state.
// ============================================================================

// ---------------------------------------------------------------------------
// EventProcessor — leaf node agent
// ---------------------------------------------------------------------------

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
                    let previous = self.latest_event_state.get_state_at(time).map(|p| p.value.clone());
                    self.latest_event_state.add_state_dynamic_info(time, val.clone());
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
                        notify_parent(&self.parent, time + within, EventValue::BoolValue(false)).await;
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

fn state_to_result(state: Option<(u64, Option<u64>, EventValue)>) -> TimelineResult {
    match state {
        Some((t1, t2, value)) => TimelineResult {
            results: vec![TimelineResultPoint { t1, t2, value }],
        },
        None => TimelineResult { results: vec![] },
    }
}

async fn notify_parent(parent: &Option<ParentRef>, time: u64, value: EventValue) {
    if let Some(parent) = parent {
        let mut client = TimelineProcessorClient::get(parent.worker_name.clone());
        client.on_child_state_changed(parent.child_index, time, value).await;
    }
}

// ---------------------------------------------------------------------------
// TimelineProcessor — derived node agent (push model)
//
// Maintains its own state. Receives state change notifications from children,
// recomputes, and pushes to its own parent.
// ---------------------------------------------------------------------------

#[agent_definition]
pub trait TimelineProcessor {
    fn new(name: String) -> Self;
    fn initialize_derived(&mut self, operation: DerivedOperation, children: Vec<ChildWorkerRef>);
    fn set_parent(&mut self, parent: ParentRef);
    async fn on_child_state_changed(&mut self, child_index: u32, time: u64, value: EventValue);
    fn get_derived_result(&self, t1: u64) -> Result<TimelineResult, String>;
}

struct TimelineProcessorImpl {
    _name: String,
    operation: Option<DerivedOperation>,
    children: Vec<ChildWorkerRef>,
    parent: Option<ParentRef>,
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
                let prev = self.result_state.get_state_at(time).map(|p| p.value.clone());
                let new_val = GolemEventValue::BoolValue(result);
                if prev.as_ref() != Some(&new_val) {
                    self.result_state.add_state_dynamic_info(time, new_val);
                    notify_parent(&self.parent, time, EventValue::BoolValue(result)).await;
                }
            }

            DerivedOperation::Negation => {
                if let Some(b) = domain_value.get_bool() {
                    let negated = !b;
                    let prev = self.result_state.get_state_at(time).map(|p| p.value.clone());
                    let new_val = GolemEventValue::BoolValue(negated);
                    if prev.as_ref() != Some(&new_val) {
                        self.result_state.add_state_dynamic_info(time, new_val);
                        notify_parent(&self.parent, time, EventValue::BoolValue(negated)).await;
                    }
                }
            }

            DerivedOperation::And => {
                if child_index == 0 {
                    self.left_child_state.add_state_dynamic_info(time, domain_value);
                } else {
                    self.right_child_state.add_state_dynamic_info(time, domain_value);
                }
                let left_val = self.left_child_state.get_state_at(time + 1).and_then(|p| p.value.get_bool());
                let right_val = self.right_child_state.get_state_at(time + 1).and_then(|p| p.value.get_bool());
                if let (Some(l), Some(r)) = (left_val, right_val) {
                    let result = l && r;
                    let prev = self.result_state.get_state_at(time).map(|p| p.value.clone());
                    let new_val = GolemEventValue::BoolValue(result);
                    if prev.as_ref() != Some(&new_val) {
                        self.result_state.add_state_dynamic_info(time, new_val);
                        notify_parent(&self.parent, time, EventValue::BoolValue(result)).await;
                    }
                }
            }

            DerivedOperation::Or => {
                if child_index == 0 {
                    self.left_child_state.add_state_dynamic_info(time, domain_value);
                } else {
                    self.right_child_state.add_state_dynamic_info(time, domain_value);
                }
                let left_val = self.left_child_state.get_state_at(time + 1).and_then(|p| p.value.get_bool());
                let right_val = self.right_child_state.get_state_at(time + 1).and_then(|p| p.value.get_bool());
                if let (Some(l), Some(r)) = (left_val, right_val) {
                    let result = l || r;
                    let prev = self.result_state.get_state_at(time).map(|p| p.value.clone());
                    let new_val = GolemEventValue::BoolValue(result);
                    if prev.as_ref() != Some(&new_val) {
                        self.result_state.add_state_dynamic_info(time, new_val);
                        notify_parent(&self.parent, time, EventValue::BoolValue(result)).await;
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
                        self.duration_state = Some(DurationState::Climbing { base: current_count, since: time });
                    } else {
                        // Go flat at current count
                        self.duration_state = Some(DurationState::Flat { value: current_count });
                    }

                    let result_value = GolemEventValue::IntValue(current_count as i64);
                    self.result_state.add_state_dynamic_info(time, result_value.clone());
                    notify_parent(&self.parent, time, EventValue::IntValue(current_count as i64)).await;
                }
            }

            DerivedOperation::DurationInCurState => {
                // Track how long we've been in the current state
                // On any state change, reset to 0 and start climbing
                self.duration_state = Some(DurationState::Climbing { base: 0, since: time });
                let result_value = GolemEventValue::IntValue(0);
                self.result_state.add_state_dynamic_info(time, result_value);
                notify_parent(&self.parent, time, EventValue::IntValue(0)).await;
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

// ---------------------------------------------------------------------------
// TimelineDriver — orchestrator agent that takes a TimelineOpGraph,
// walks the tree, spawns EventProcessor / TimelineProcessor agents,
// and wires them together (including parent refs for push propagation).
// ---------------------------------------------------------------------------

#[agent_definition]
pub trait TimelineDriver {
    fn new(name: String) -> Self;
    async fn initialize_timeline(&self, timeline: TimelineOpGraph) -> Result<String, String>;
}

struct TimelineDriverImpl {
    name: String,
}

#[agent_implementation]
impl TimelineDriver for TimelineDriverImpl {
    fn new(name: String) -> Self {
        Self { name }
    }

    async fn initialize_timeline(&self, timeline: TimelineOpGraph) -> Result<String, String> {
        let recursive_op = timeline.to_recursive();
        let (result, _) = self.setup_node(&recursive_op, &mut 0, &None).await?;
        Ok(format!(
            "Timeline initialized. Result worker: {}",
            result.worker_name
        ))
    }
}

/// Info returned from setup_node: worker name and whether it's a leaf.
struct SetupResult {
    worker_name: String,
    is_leaf: bool,
}

async fn set_child_parent(child: &SetupResult, parent_name: &str, child_index: u32) {
    let parent_ref = ParentRef {
        worker_name: parent_name.to_string(),
        child_index,
    };
    if child.is_leaf {
        let mut client = EventProcessorClient::get(child.worker_name.clone());
        client.set_parent(parent_ref).await;
    } else {
        let mut client = TimelineProcessorClient::get(child.worker_name.clone());
        client.set_parent(parent_ref).await;
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
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(SetupResult, bool), String>> + 'a>>
    {
        Box::pin(async move {
            *counter += 1;
            let worker_name = format!("{}-node-{}", self.name, counter);

            match op {
                common_lib::TimeLineOp::TlLatestEventToState(col) => {
                    let mut ep = EventProcessorClient::get(worker_name.clone());
                    ep.initialize_leaf(LeafOperation::LatestEventToState(col.0.clone()))
                        .await;
                    Ok((SetupResult { worker_name, is_leaf: true }, true))
                }
                common_lib::TimeLineOp::TlHasExisted(pred) => {
                    let mut ep = EventProcessorClient::get(worker_name.clone());
                    ep.initialize_leaf(LeafOperation::TlHasExisted(predicate_to_api(pred)))
                        .await;
                    Ok((SetupResult { worker_name, is_leaf: true }, true))
                }
                common_lib::TimeLineOp::TlHasExistedWithin(pred, dur) => {
                    let mut ep = EventProcessorClient::get(worker_name.clone());
                    ep.initialize_leaf(LeafOperation::TlHasExistedWithin(
                        predicate_to_api(pred),
                        *dur,
                    ))
                    .await;
                    Ok((SetupResult { worker_name, is_leaf: true }, true))
                }
                common_lib::TimeLineOp::EqualTo(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(CompareOp::EqualTo, EventValue::from_domain(val)),
                        vec![ChildWorkerRef { worker_name: child_result.worker_name.clone(), is_leaf: child_result.is_leaf }],
                    ).await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::GreaterThan(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(CompareOp::GreaterThan, EventValue::from_domain(val)),
                        vec![ChildWorkerRef { worker_name: child_result.worker_name.clone(), is_leaf: child_result.is_leaf }],
                    ).await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::GreaterThanOrEqual(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(CompareOp::GreaterThanOrEqual, EventValue::from_domain(val)),
                        vec![ChildWorkerRef { worker_name: child_result.worker_name.clone(), is_leaf: child_result.is_leaf }],
                    ).await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::LessThan(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(CompareOp::LessThan, EventValue::from_domain(val)),
                        vec![ChildWorkerRef { worker_name: child_result.worker_name.clone(), is_leaf: child_result.is_leaf }],
                    ).await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::LessThanOrEqual(child, val) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Comparison(CompareOp::LessThanOrEqual, EventValue::from_domain(val)),
                        vec![ChildWorkerRef { worker_name: child_result.worker_name.clone(), is_leaf: child_result.is_leaf }],
                    ).await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::Not(child) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Negation,
                        vec![ChildWorkerRef { worker_name: child_result.worker_name.clone(), is_leaf: child_result.is_leaf }],
                    ).await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::And(left, right) => {
                    let (left_result, _) = self.setup_node(left, counter, _parent_ref).await?;
                    let (right_result, _) = self.setup_node(right, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::And,
                        vec![
                            ChildWorkerRef { worker_name: left_result.worker_name.clone(), is_leaf: left_result.is_leaf },
                            ChildWorkerRef { worker_name: right_result.worker_name.clone(), is_leaf: right_result.is_leaf },
                        ],
                    ).await;
                    set_child_parent(&left_result, &worker_name, 0).await;
                    set_child_parent(&right_result, &worker_name, 1).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::Or(left, right) => {
                    let (left_result, _) = self.setup_node(left, counter, _parent_ref).await?;
                    let (right_result, _) = self.setup_node(right, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::Or,
                        vec![
                            ChildWorkerRef { worker_name: left_result.worker_name.clone(), is_leaf: left_result.is_leaf },
                            ChildWorkerRef { worker_name: right_result.worker_name.clone(), is_leaf: right_result.is_leaf },
                        ],
                    ).await;
                    set_child_parent(&left_result, &worker_name, 0).await;
                    set_child_parent(&right_result, &worker_name, 1).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::TlDurationWhere(child) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::DurationWhere,
                        vec![ChildWorkerRef { worker_name: child_result.worker_name.clone(), is_leaf: child_result.is_leaf }],
                    ).await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
                common_lib::TimeLineOp::TlDurationInCurState(child) => {
                    let (child_result, _) = self.setup_node(child, counter, _parent_ref).await?;
                    let mut tp = TimelineProcessorClient::get(worker_name.clone());
                    tp.initialize_derived(
                        DerivedOperation::DurationInCurState,
                        vec![ChildWorkerRef { worker_name: child_result.worker_name.clone(), is_leaf: child_result.is_leaf }],
                    ).await;
                    set_child_parent(&child_result, &worker_name, 0).await;
                    Ok((SetupResult { worker_name, is_leaf: false }, false))
                }
            }
        })
    }
}
