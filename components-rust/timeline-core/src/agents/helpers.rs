// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
use crate::types::*;
use crate::agents::timeline_processor::TimelineProcessorClient;
use crate::agents::event_processor::EventProcessorClient;
use crate::agents::aggregator::AggregatorClient;

pub(crate) fn state_to_result(state: Option<(u64, Option<u64>, EventValue)>) -> TimelineResult {
    match state {
        Some((t1, t2, value)) => TimelineResult {
            results: vec![TimelineResultPoint { t1, t2, value }],
        },
        None => TimelineResult { results: vec![] },
    }
}

pub(crate) async fn notify_parent(
    parent: &Option<ParentRef>,
    time: u64,
    value: EventValue,
    group_by_value: &Option<EventValue>,
) {
    if let Some(parent) = parent {
        let mut client = TimelineProcessorClient::get(parent.agent_name.clone());
        client
            .on_child_state_changed(parent.child_index, time, value, group_by_value.clone())
            .await;
    }
}

pub(crate) fn event_value_to_f64(value: &EventValue) -> Option<f64> {
    match value {
        EventValue::IntValue(i) => Some(*i as f64),
        EventValue::FloatValue(f) => Some(*f),
        EventValue::BoolValue(b) => Some(if *b { 1.0 } else { 0.0 }),
        _ => None,
    }
}

fn event_value_to_string(value: &EventValue) -> String {
    match value {
        EventValue::StringValue(s) => s.clone(),
        EventValue::IntValue(i) => i.to_string(),
        EventValue::FloatValue(f) => f.to_string(),
        EventValue::BoolValue(b) => b.to_string(),
    }
}

pub(crate) async fn notify_aggregator(
    aggregation: &Option<AggregationConfig>,
    aggregator_agent: &mut Option<String>,
    last_aggregated_value: &mut Option<f64>,
    _time: u64,
    value: &EventValue,
    group_by_value: &Option<EventValue>,
) {
    let agg_config = match aggregation {
        Some(c) => c,
        None => return,
    };

    let group_value = match group_by_value {
        Some(v) => v,
        None => return,
    };

    let new_value = match event_value_to_f64(value) {
        Some(v) => v,
        None => return,
    };

    let delta = new_value - last_aggregated_value.unwrap_or(0.0);
    *last_aggregated_value = Some(new_value);

    if delta == 0.0 {
        return;
    }

    let first_call = aggregator_agent.is_none();
    let agent_name = aggregator_agent.get_or_insert_with(|| {
        format!(
            "aggregator-{}-{}",
            agg_config.group_by_column,
            event_value_to_string(group_value),
        )
    });

    let mut client = AggregatorClient::get(agent_name.clone());
    if first_call {
        client
            .initialize_aggregator(agg_config.aggregations.clone())
            .await;
        client.register_session().await;
    }
    client.on_delta(delta).await;
}

pub(crate) struct SetupResult {
    pub agent_name: String,
    pub is_leaf: bool,
}

pub(crate) async fn set_child_parent(child: &SetupResult, parent_name: &str, child_index: u32) {
    let parent_ref = ParentRef {
        agent_name: parent_name.to_string(),
        child_index,
    };
    if child.is_leaf {
        let mut client = EventProcessorClient::get(child.agent_name.clone());
        client.set_parent(parent_ref).await;
    } else {
        let mut client = TimelineProcessorClient::get(child.agent_name.clone());
        client.set_parent(parent_ref).await;
    }
}