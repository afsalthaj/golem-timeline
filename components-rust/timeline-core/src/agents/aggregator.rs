use golem_rust::{agent_definition, agent_implementation};

use crate::types::*;

/// Cross-session accumulator. One Aggregator per (column, value) pair.
/// Created lazily by the root TimelineProcessor on the first delta push.
///
/// CIRR example with 10,000 sessions across 3 CDNs:
///   - "aggregator-cdn-akamai"    ← receives deltas from all sessions where cdn="akamai"
///   - "aggregator-cdn-cloudfront" ← receives deltas from all sessions where cdn="cloudfront"
///   - "aggregator-cdn-fastly"    ← receives deltas from all sessions where cdn="fastly"
///
///   Session "sess-42" (cdn="akamai") starts buffering:
///     Root node-8 computes CIRR duration delta = 3.0 seconds
///     → calls `Aggregator::get("aggregator-cdn-akamai").on_delta(3.0)`
///
///   Session "sess-99" (also cdn="akamai") has CIRR delta = 1.5 seconds
///     → calls the same `Aggregator::get("aggregator-cdn-akamai").on_delta(1.5)`
///
///   Querying "aggregator-cdn-akamai":
///     → `{ count: 5000, sum: 12000.0, avg: 2.4, ... }`
///     "across 5000 akamai sessions, total CIRR is 12000s, average 2.4s per session"
#[agent_definition]
pub trait Aggregator {
    fn new(name: String) -> Self;

    /// Set which aggregation functions to compute. Called once by the root
    /// TimelineProcessor on its first delta push (lazy initialization).
    ///
    /// CIRR example:
    ///   `Aggregator::get("aggregator-cdn-akamai").initialize_aggregator([Count, Sum, Avg])`
    fn initialize_aggregator(&mut self, aggregations: Vec<AggregationType>);

    /// Receive a CIRR duration delta from a session's root node.
    ///
    /// CIRR example:
    ///   Session "sess-42" CIRR went from 3s to 5s → `on_delta(2.0)`
    ///   The aggregator adds 2.0 to its running sum.
    fn on_delta(&mut self, delta: f64);

    /// Register a new session contributing to this aggregator.
    /// Called once per session by the root TimelineProcessor on first delta.
    ///
    /// CIRR example:
    ///   Session "sess-42" starts pushing → `register_session()` → count goes from 4999 to 5000
    fn register_session(&mut self);

    /// Query accumulated metrics across all sessions for this group.
    ///
    /// CIRR example:
    ///   `Aggregator::get("aggregator-cdn-akamai").get_aggregation_result()`
    ///   → `{ count: 5000, sum: 12000.0, avg: 2.4, min: None, max: None }`
    fn get_aggregation_result(&self) -> AggregationResult;
}

struct AggregatorImpl {
    _name: String,
    aggregations: Vec<AggregationType>,
    count: u64,
    sum: f64,
}

#[agent_implementation]
impl Aggregator for AggregatorImpl {
    fn new(name: String) -> Self {
        Self {
            _name: name,
            aggregations: Vec::new(),
            count: 0,
            sum: 0.0,
        }
    }

    fn initialize_aggregator(&mut self, aggregations: Vec<AggregationType>) {
        self.aggregations = aggregations;
    }

    fn on_delta(&mut self, delta: f64) {
        self.sum += delta;
    }

    fn register_session(&mut self) {
        self.count += 1;
    }

    fn get_aggregation_result(&self) -> AggregationResult {
        let avg = if self.count > 0 {
            self.sum / self.count as f64
        } else {
            0.0
        };
        AggregationResult {
            count: self.count,
            sum: self.sum,
            avg,
            min: None,
            max: None,
        }
    }
}
