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

pub(crate) async fn notify_parent(parent: &Option<ParentRef>, time: u64, value: EventValue) {
    if let Some(parent) = parent {
        let mut client = TimelineProcessorClient::get(parent.worker_name.clone());
        client
            .on_child_state_changed(parent.child_index, time, value)
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

pub(crate) async fn notify_aggregator(
    aggregator: &Option<AggregatorRef>,
    last_aggregated_value: &mut Option<f64>,
    _time: u64,
    value: &EventValue,
) {
    if let Some(agg) = aggregator {
        let new_value = match event_value_to_f64(value) {
            Some(v) => v,
            None => return,
        };
        let delta = new_value - last_aggregated_value.unwrap_or(0.0);
        *last_aggregated_value = Some(new_value);
        if delta != 0.0 {
            let mut client = AggregatorClient::get(agg.worker_name.clone());
            client.on_delta(delta).await;
        }
    }
}

pub(crate) struct SetupResult {
    pub worker_name: String,
    pub is_leaf: bool,
}

pub(crate) async fn set_child_parent(child: &SetupResult, parent_name: &str, child_index: u32) {
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
