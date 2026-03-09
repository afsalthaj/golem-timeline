// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use common_lib::TimeLineOp;
use timeline_dsl::{parse, AggregationFunction, ParsedAggregation, ParsedTimeline};

// ── Newtypes ──

/// A human-readable name for a registered metric (e.g., "cirr", "idle-time").
#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct MetricName(pub String);

/// The raw DSL text as written by the developer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DslText(pub String);

/// The agent name suffix, e.g. "has-existed-4".
/// Full agent name is `{session-id}-{metric-name}-{suffix}`.
#[derive(Clone, Debug, Serialize)]
pub struct AgentSuffix(pub String);

/// The name of an event column (e.g., "playerStateChange", "cdn").
#[derive(Clone, Debug, Serialize, Hash, Eq, PartialEq)]
pub struct EventColumnName(pub String);

/// The name of the column used for group-by aggregation.
#[derive(Clone, Debug, Serialize)]
pub struct GroupByColumn(pub String);

/// Human-readable description of what a node computes.
#[derive(Clone, Debug, Serialize)]
pub struct NodeDescription(pub String);

/// The operation name in kebab-case (e.g., "duration-where", "has-existed").
#[derive(Clone, Debug, Serialize)]
pub struct OperationName(pub String);

/// Name of an aggregation function (e.g., "count", "sum").
#[derive(Clone, Debug, Serialize)]
pub struct AggregationFunctionName(pub String);

// ── Domain types ──

/// A leaf agent in the compiled tree.
#[derive(Clone, Debug, Serialize)]
pub struct LeafInfo {
    pub agent_suffix: AgentSuffix,
    pub event_columns: Vec<EventColumnName>,
    pub description: NodeDescription,
}

/// A derived (non-leaf) agent in the compiled tree.
#[derive(Clone, Debug, Serialize)]
pub struct DerivedInfo {
    pub agent_suffix: AgentSuffix,
    pub operation: OperationName,
    pub description: NodeDescription,
}

/// Maps an event column to the agent suffixes of leaves that need events
/// containing that column.
#[derive(Clone, Debug, Serialize)]
pub struct LeafRoutingTable {
    pub routes: HashMap<EventColumnName, Vec<AgentSuffix>>,
}

/// Aggregation configuration as derived from the DSL.
#[derive(Clone, Debug, Serialize)]
pub struct AggregationInfo {
    pub group_by_column: GroupByColumn,
    pub functions: Vec<AggregationFunctionName>,
}

/// A registered metric with its compiled representation and routing info.
#[derive(Clone, Debug, Serialize)]
pub struct RegisteredMetric {
    pub name: MetricName,
    pub dsl: DslText,
    pub leaves: Vec<LeafInfo>,
    pub derived_nodes: Vec<DerivedInfo>,
    pub routing_table: LeafRoutingTable,
    pub aggregation: Option<AggregationInfo>,
}

/// Request body for POST /api/metrics.
#[derive(Deserialize)]
pub struct RegisterMetricRequest {
    pub name: MetricName,
    pub dsl: DslText,
}

// ── Registry ──

/// In-memory metric registry. Deduplicates by normalized DSL text.
#[derive(Clone)]
pub struct MetricRegistry {
    inner: Arc<Mutex<RegistryInner>>,
}

/// Internal storage for a registered metric alongside its parsed form.
struct StoredMetric {
    metric: RegisteredMetric,
    parsed: ParsedTimeline,
}

struct RegistryInner {
    metrics: Vec<StoredMetric>,
    /// Normalized DSL text → index into `metrics` for deduplication.
    dsl_index: HashMap<String, usize>,
}

impl MetricRegistry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(RegistryInner {
                metrics: Vec::new(),
                dsl_index: HashMap::new(),
            })),
        }
    }

    /// Register a metric. Returns the registered metric plus WAVE strings for
    /// the feeder, or an error.
    /// If an identical DSL is already registered, returns the existing one.
    pub fn register(
        &self,
        req: &RegisterMetricRequest,
    ) -> Result<(RegisteredMetric, String, String), String> {
        let mut inner = self.inner.lock().map_err(|e| e.to_string())?;

        let normalized = normalize_dsl(&req.dsl.0);
        if let Some(&idx) = inner.dsl_index.get(&normalized) {
            let existing = &inner.metrics[idx];
            let graph_wave = crate::wave::timeline_op_to_wave(&existing.parsed.op);
            let agg_wave = crate::wave::aggregation_to_wave(&existing.parsed.aggregation);
            return Ok((existing.metric.clone(), graph_wave, agg_wave));
        }

        let parsed = parse(&req.dsl.0).map_err(|e| e.to_string())?;

        let mut leaves = Vec::new();
        let mut derived_nodes = Vec::new();
        let mut counter: u64 = 0;
        walk_tree(&parsed.op, &mut counter, &mut leaves, &mut derived_nodes);

        let mut routes: HashMap<EventColumnName, Vec<AgentSuffix>> = HashMap::new();
        for leaf in &leaves {
            for col in &leaf.event_columns {
                routes
                    .entry(col.clone())
                    .or_default()
                    .push(leaf.agent_suffix.clone());
            }
        }

        let aggregation = parsed.aggregation.as_ref().map(|a| convert_aggregation(a));

        let graph_wave = crate::wave::timeline_op_to_wave(&parsed.op);
        let agg_wave = crate::wave::aggregation_to_wave(&parsed.aggregation);

        let metric = RegisteredMetric {
            name: req.name.clone(),
            dsl: req.dsl.clone(),
            leaves,
            derived_nodes,
            routing_table: LeafRoutingTable { routes },
            aggregation,
        };

        let idx = inner.metrics.len();
        inner.dsl_index.insert(normalized, idx);
        inner.metrics.push(StoredMetric {
            metric: metric.clone(),
            parsed,
        });

        Ok((metric, graph_wave, agg_wave))
    }

    /// List all registered metrics.
    pub fn list(&self) -> Result<Vec<RegisteredMetric>, String> {
        let inner = self.inner.lock().map_err(|e| e.to_string())?;
        Ok(inner.metrics.iter().map(|s| s.metric.clone()).collect())
    }
}

// ── Tree walking ──

/// Walk the recursive TimeLineOp tree in pre-order DFS (matching the driver's
/// numbering scheme) and collect leaf/derived info.
fn walk_tree(
    op: &TimeLineOp,
    counter: &mut u64,
    leaves: &mut Vec<LeafInfo>,
    derived: &mut Vec<DerivedInfo>,
) {
    *counter += 1;
    let n = *counter;

    match op {
        TimeLineOp::TlLatestEventToState(col) => {
            leaves.push(LeafInfo {
                agent_suffix: AgentSuffix(format!("latest-event-to-state-{}", n)),
                event_columns: vec![EventColumnName(col.0.clone())],
                description: NodeDescription(format!(
                    "Track the latest value of \"{}\" as state",
                    col.0
                )),
            });
        }
        TimeLineOp::TlHasExisted(pred) => {
            leaves.push(LeafInfo {
                agent_suffix: AgentSuffix(format!("has-existed-{}", n)),
                event_columns: vec![predicate_column(pred)],
                description: NodeDescription(format!("Has {} ever been true?", pred)),
            });
        }
        TimeLineOp::TlHasExistedWithin(pred, dur) => {
            leaves.push(LeafInfo {
                agent_suffix: AgentSuffix(format!("has-existed-within-{}", n)),
                event_columns: vec![predicate_column(pred)],
                description: NodeDescription(format!(
                    "Has {} been true within the last {} time units?",
                    pred, dur
                )),
            });
        }
        TimeLineOp::EqualTo(child, val) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("equal-to-{}", n)),
                operation: OperationName(format!("equal-to({})", val)),
                description: NodeDescription(format!("Is the child's state equal to {}?", val)),
            });
            walk_tree(child, counter, leaves, derived);
        }
        TimeLineOp::GreaterThan(child, val) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("greater-than-{}", n)),
                operation: OperationName(format!("greater-than({})", val)),
                description: NodeDescription(format!(
                    "Is the child's state greater than {}?",
                    val
                )),
            });
            walk_tree(child, counter, leaves, derived);
        }
        TimeLineOp::GreaterThanOrEqual(child, val) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("greater-than-or-equal-{}", n)),
                operation: OperationName(format!("greater-than-or-equal({})", val)),
                description: NodeDescription(format!("Is the child's state ≥ {}?", val)),
            });
            walk_tree(child, counter, leaves, derived);
        }
        TimeLineOp::LessThan(child, val) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("less-than-{}", n)),
                operation: OperationName(format!("less-than({})", val)),
                description: NodeDescription(format!(
                    "Is the child's state less than {}?",
                    val
                )),
            });
            walk_tree(child, counter, leaves, derived);
        }
        TimeLineOp::LessThanOrEqual(child, val) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("less-than-or-equal-{}", n)),
                operation: OperationName(format!("less-than-or-equal({})", val)),
                description: NodeDescription(format!("Is the child's state ≤ {}?", val)),
            });
            walk_tree(child, counter, leaves, derived);
        }
        TimeLineOp::Not(child) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("not-{}", n)),
                operation: OperationName("not".to_string()),
                description: NodeDescription("Negate the child's boolean state".to_string()),
            });
            walk_tree(child, counter, leaves, derived);
        }
        TimeLineOp::And(left, right) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("and-{}", n)),
                operation: OperationName("and".to_string()),
                description: NodeDescription("Both children must be true".to_string()),
            });
            walk_tree(left, counter, leaves, derived);
            walk_tree(right, counter, leaves, derived);
        }
        TimeLineOp::Or(left, right) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("or-{}", n)),
                operation: OperationName("or".to_string()),
                description: NodeDescription("Either child must be true".to_string()),
            });
            walk_tree(left, counter, leaves, derived);
            walk_tree(right, counter, leaves, derived);
        }
        TimeLineOp::TlDurationWhere(child) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("duration-where-{}", n)),
                operation: OperationName("duration-where".to_string()),
                description: NodeDescription(
                    "Cumulative duration where child is true".to_string(),
                ),
            });
            walk_tree(child, counter, leaves, derived);
        }
        TimeLineOp::TlDurationInCurState(child) => {
            derived.push(DerivedInfo {
                agent_suffix: AgentSuffix(format!("duration-in-cur-state-{}", n)),
                operation: OperationName("duration-in-cur-state".to_string()),
                description: NodeDescription(
                    "Duration in the child's current state".to_string(),
                ),
            });
            walk_tree(child, counter, leaves, derived);
        }
    }
}

/// Extract the column name from a predicate.
fn predicate_column(
    pred: &common_lib::GolemEventPredicate<common_lib::GolemEventValue>,
) -> EventColumnName {
    match pred {
        common_lib::GolemEventPredicate::Equals(col, _) => EventColumnName(col.0.clone()),
        common_lib::GolemEventPredicate::GreaterThan(col, _) => EventColumnName(col.0.clone()),
        common_lib::GolemEventPredicate::LessThan(col, _) => EventColumnName(col.0.clone()),
        common_lib::GolemEventPredicate::And(left, _) => predicate_column(left),
        common_lib::GolemEventPredicate::Or(left, _) => predicate_column(left),
    }
}

fn convert_aggregation(a: &ParsedAggregation) -> AggregationInfo {
    AggregationInfo {
        group_by_column: GroupByColumn(a.group_by.clone()),
        functions: a
            .functions
            .iter()
            .map(|f| match f {
                AggregationFunction::Count => AggregationFunctionName("count".to_string()),
                AggregationFunction::Sum => AggregationFunctionName("sum".to_string()),
                AggregationFunction::Avg => AggregationFunctionName("avg".to_string()),
                AggregationFunction::Min => AggregationFunctionName("min".to_string()),
                AggregationFunction::Max => AggregationFunctionName("max".to_string()),
            })
            .collect(),
    }
}

/// Normalize DSL whitespace for deduplication.
fn normalize_dsl(dsl: &str) -> String {
    dsl.split_whitespace().collect::<Vec<_>>().join(" ")
}
