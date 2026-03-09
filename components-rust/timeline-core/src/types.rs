use golem_rust::Schema;

/// A node index into the flat `TimelineOpGraph.nodes` array.
pub type NodeIndex = i64;

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

/// Which aggregation functions to compute.
#[derive(Clone, Debug, Schema)]
pub enum AggregationType {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

/// Configuration for cross-session aggregation.
///
/// `group_by_column` is the event column name (from the DSL, e.g. `cdn`).
/// The feeder extracts the concrete value of this column from event data at
/// runtime and routes each session's root node to the correct aggregator agent.
#[derive(Clone, Debug, Schema)]
pub struct AggregationConfig {
    pub group_by_column: String,
    pub aggregations: Vec<AggregationType>,
}

/// Result of initializing a timeline via the TimelineDriver.
///
/// Contains the root worker name and all leaf worker names so the feeder
/// knows exactly where to send events — no graph walking needed.
#[derive(Clone, Debug, Schema)]
pub struct InitializeResult {
    pub root_worker: String,
    pub leaf_workers: Vec<String>,
}

/// Result of querying an aggregator.
#[derive(Clone, Debug, Schema)]
pub struct AggregationResult {
    pub count: u64,
    pub sum: f64,
    pub avg: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
}
