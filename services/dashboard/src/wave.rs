// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
//! Converts parsed DSL structures into WAVE format strings for `golem agent invoke`.

use common_lib::{GolemEventPredicate, GolemEventValue, TimeLineOp};
use timeline_dsl::ParsedAggregation;

/// Convert a `GolemEventValue` to its WAVE representation.
fn event_value_to_wave(v: &GolemEventValue) -> String {
    match v {
        GolemEventValue::StringValue(s) => format!("string-value(\"{}\")", s),
        GolemEventValue::IntValue(i) => format!("int-value({})", i),
        GolemEventValue::FloatValue(f) => format!("float-value({})", f),
        GolemEventValue::BoolValue(b) => format!("bool-value({})", b),
    }
}

/// Convert a `GolemEventPredicate` to its WAVE representation.
/// Compound predicates (And/Or) are not supported in the API encoding.
fn predicate_to_wave(pred: &GolemEventPredicate<GolemEventValue>) -> String {
    match pred {
        GolemEventPredicate::Equals(col, val) => {
            format!(
                "{{col-name: \"{}\", value: {}, op: equal}}",
                col.0,
                event_value_to_wave(&val.0)
            )
        }
        GolemEventPredicate::GreaterThan(col, val) => {
            format!(
                "{{col-name: \"{}\", value: {}, op: greater-than}}",
                col.0,
                event_value_to_wave(&val.0)
            )
        }
        GolemEventPredicate::LessThan(col, val) => {
            format!(
                "{{col-name: \"{}\", value: {}, op: less-than}}",
                col.0,
                event_value_to_wave(&val.0)
            )
        }
        GolemEventPredicate::And(_, _) | GolemEventPredicate::Or(_, _) => {
            // Compound predicates not supported in API encoding (same as conversions.rs)
            String::from("{}")
        }
    }
}

/// Convert a compare-op variant name to kebab-case WAVE.
fn compare_op_name(op: &TimeLineOp) -> &'static str {
    match op {
        TimeLineOp::EqualTo(_, _) => "equal-to",
        TimeLineOp::GreaterThan(_, _) => "greater-than",
        TimeLineOp::GreaterThanOrEqual(_, _) => "greater-than-or-equal",
        TimeLineOp::LessThan(_, _) => "less-than",
        TimeLineOp::LessThanOrEqual(_, _) => "less-than-or-equal",
        _ => "",
    }
}

/// Recursively build the flat WAVE node array. Returns the index of the node
/// that was just added.
fn build_wave_node(op: &TimeLineOp, nodes: &mut Vec<String>) -> i32 {
    match op {
        TimeLineOp::TlLatestEventToState(col) => {
            let idx = nodes.len();
            nodes.push(format!("tl-latest-event-to-state(\"{}\")", col.0));
            idx as i32
        }
        TimeLineOp::TlHasExisted(pred) => {
            let idx = nodes.len();
            nodes.push(format!("tl-has-existed({})", predicate_to_wave(pred)));
            idx as i32
        }
        TimeLineOp::TlHasExistedWithin(pred, dur) => {
            let idx = nodes.len();
            nodes.push(format!(
                "tl-has-existed-within(({}, {}))",
                predicate_to_wave(pred),
                dur
            ));
            idx as i32
        }
        TimeLineOp::TlDurationWhere(child) => {
            let parent_idx = nodes.len();
            nodes.push(String::new()); // placeholder
            let child_idx = build_wave_node(child, nodes);
            nodes[parent_idx] = format!("tl-duration-where({})", child_idx);
            parent_idx as i32
        }
        TimeLineOp::TlDurationInCurState(child) => {
            let parent_idx = nodes.len();
            nodes.push(String::new());
            let child_idx = build_wave_node(child, nodes);
            nodes[parent_idx] = format!("tl-duration-in-cur-state({})", child_idx);
            parent_idx as i32
        }
        TimeLineOp::Not(child) => {
            let parent_idx = nodes.len();
            nodes.push(String::new());
            let child_idx = build_wave_node(child, nodes);
            nodes[parent_idx] = format!("negation({})", child_idx);
            parent_idx as i32
        }
        TimeLineOp::And(left, right) => {
            let parent_idx = nodes.len();
            nodes.push(String::new());
            let l_idx = build_wave_node(left, nodes);
            let r_idx = build_wave_node(right, nodes);
            nodes[parent_idx] = format!("and(({}, {}))", l_idx, r_idx);
            parent_idx as i32
        }
        TimeLineOp::Or(left, right) => {
            let parent_idx = nodes.len();
            nodes.push(String::new());
            let l_idx = build_wave_node(left, nodes);
            let r_idx = build_wave_node(right, nodes);
            nodes[parent_idx] = format!("or(({}, {}))", l_idx, r_idx);
            parent_idx as i32
        }
        TimeLineOp::EqualTo(child, val)
        | TimeLineOp::GreaterThan(child, val)
        | TimeLineOp::GreaterThanOrEqual(child, val)
        | TimeLineOp::LessThan(child, val)
        | TimeLineOp::LessThanOrEqual(child, val) => {
            let op_name = compare_op_name(op);
            let parent_idx = nodes.len();
            nodes.push(String::new());
            let child_idx = build_wave_node(child, nodes);
            nodes[parent_idx] = format!(
                "comparison(({}, {}, {}))",
                op_name,
                child_idx,
                event_value_to_wave(val)
            );
            parent_idx as i32
        }
    }
}

/// Convert a parsed `TimeLineOp` into a WAVE-format `TimelineOpGraph` string.
///
/// Builds a flat graph (nodes array, root at index 0) from the recursive
/// `TimeLineOp` — the same algorithm as `to_graph` in `conversions.rs` but
/// producing WAVE strings instead of typed nodes.
pub fn timeline_op_to_wave(op: &TimeLineOp) -> String {
    let mut nodes = Vec::new();
    build_wave_node(op, &mut nodes);
    let inner = nodes.join(", ");
    format!("{{nodes: [{}]}}", inner)
}

/// Convert aggregation config to WAVE format.
/// Returns `"none"` if aggregation is `None`.
pub fn aggregation_to_wave(agg: &Option<ParsedAggregation>) -> String {
    match agg {
        None => "none".to_string(),
        Some(a) => {
            let fns: Vec<&str> = a
                .functions
                .iter()
                .map(|f| match f {
                    timeline_dsl::AggregationFunction::Count => "count",
                    timeline_dsl::AggregationFunction::Sum => "sum",
                    timeline_dsl::AggregationFunction::Avg => "avg",
                    timeline_dsl::AggregationFunction::Min => "min",
                    timeline_dsl::AggregationFunction::Max => "max",
                })
                .collect();
            format!(
                "some({{group-by-column: \"{}\", aggregations: [{}]}})",
                a.group_by,
                fns.join(", ")
            )
        }
    }
}

/// Convert a JSON event from Kafka into WAVE format for `golem agent invoke`.
///
/// Expects JSON of the form:
/// ```json
/// {"time": N, "event": [["col", {"string-value": "val"}], ...]}
/// ```
///
/// Produces:
/// ```text
/// {time: N, event: [("col", string-value("val")), ...]}
/// ```
pub fn event_json_to_wave(json: &serde_json::Value) -> Option<String> {
    let time = json.get("time")?.as_u64()?;
    let event_arr = json.get("event")?.as_array()?;

    let mut pairs = Vec::new();
    for entry in event_arr {
        let pair = entry.as_array()?;
        if pair.len() != 2 {
            return None;
        }
        let col_name = pair[0].as_str()?;
        let value_obj = pair[1].as_object()?;
        let wave_val = json_value_to_wave(value_obj)?;
        pairs.push(format!("(\"{}\", {})", col_name, wave_val));
    }

    Some(format!("{{time: {}, event: [{}]}}", time, pairs.join(", ")))
}

/// Convert a JSON event-value object (e.g., `{"string-value": "s"}`) to WAVE.
fn json_value_to_wave(obj: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    if let Some(v) = obj.get("string-value") {
        return Some(format!("string-value(\"{}\")", v.as_str()?));
    }
    if let Some(v) = obj.get("int-value") {
        return Some(format!("int-value({})", v.as_i64()?));
    }
    if let Some(v) = obj.get("float-value") {
        return Some(format!("float-value({})", v.as_f64()?));
    }
    if let Some(v) = obj.get("bool-value") {
        return Some(format!("bool-value({})", v.as_bool()?));
    }
    None
}
