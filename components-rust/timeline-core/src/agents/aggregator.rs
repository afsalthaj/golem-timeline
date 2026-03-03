use golem_rust::{agent_definition, agent_implementation};

use crate::types::*;

#[agent_definition]
pub trait Aggregator {
    fn new(name: String) -> Self;
    fn initialize_aggregator(&mut self, aggregations: Vec<AggregationType>);
    fn on_delta(&mut self, delta: f64);
    fn register_session(&mut self);
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
