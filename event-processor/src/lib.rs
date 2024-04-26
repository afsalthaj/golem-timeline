use std::cell::RefCell;
use timeline::event_predicate::EventColumnName;
use timeline::golem_event::GolemEventValue;
use timeline::state_dynamic_timeline::StateDynamicsTimeLine;
use crate::bindings::exports::timeline::event_processor::api::{Event, EventStateResult, EventValue, Guest, LatestEventToStateResult, TimePeriod, WorkerId};

    mod bindings;

    struct Component;

    struct LatestEventToStateTracker {
        state_dynamic_timeline: StateDynamicsTimeLine<GolemEventValue>,
        col_name: Option<EventColumnName>
    }

thread_local! {
    static LATEST_EVENT_TO_STATE: RefCell<LatestEventToStateTracker> = RefCell::new(LatestEventToStateTracker {
        state_dynamic_timeline: StateDynamicsTimeLine::default(),
        col_name: None
    });

}

fn with_latest_event_to_state<T>(f: impl FnOnce(&mut LatestEventToStateTracker) -> Result<T, String>) -> Result<T, String> {
    let result = LATEST_EVENT_TO_STATE.with_borrow_mut(|state| {
        f(state)
    });

    return result;
}

impl Guest for Component {
    fn initialize_latest_event_state(worker: WorkerId, event_column_name: String) -> Result<String, String> {
        with_latest_event_to_state(|state| {
            state.col_name = Some(EventColumnName(event_column_name.clone()));
            Ok(worker.name)
        })
    }

    fn add_event(event: Event) -> Result<String, String> {
        with_latest_event_to_state(|state| {
            if let Some(state_col_name) = state.col_name.as_ref() {

                dbg!(event.event.clone());
                dbg!(state_col_name.0.clone());

                let event_value =
                    event.event.iter().find(|(key, _)| key == state_col_name.0.as_str());

                dbg!(event_value);
                match event_value {
                    Some((_, value)) => {
                        let golem_event_value = get_golem_column_event_value(value.clone());
                        state.state_dynamic_timeline.add_state_dynamic_info(event.time, golem_event_value);
                        dbg!("Added event for time {} for the latest-event-to-state of {}", event.time, &state_col_name.0);
                    },
                    None => {
                        dbg!("No event value found for the column name: {}", &state_col_name.0);
                    }
                }
            };
            Ok("Added event to the state dynamic event".to_string())
        })
    }

    fn latest_event_to_state(t1: u64) -> Result<LatestEventToStateResult, String> {
        with_latest_event_to_state(|state| {
            let latest_event = state.state_dynamic_timeline.get_state_at(t1);

            let column_name = state.col_name.as_ref().ok_or("Latest Event To State hasn't been initialised or part of the workflow yet")?;


            let result = match latest_event {
                Some(event) => LatestEventToStateResult {
                    event_col_name: column_name.0.clone(),
                    event_results: {
                        let event_result = EventStateResult {
                            time_period: TimePeriod {
                                t1: event.t1,
                                t2: event.t2.unwrap_or(u64::MAX)
                            },
                            value: get_event_value(event.value)
                        };
                        vec![event_result]
                    }
                },

                None => LatestEventToStateResult {
                    event_col_name: column_name.0.clone(),
                    event_results: vec![]
                }
            };

            Ok(result)
        })
    }
}


fn get_golem_column_event_value(event_value: EventValue) -> GolemEventValue {
    match event_value {
        EventValue::StringValue(s) => GolemEventValue::StringValue(s.clone()),
        EventValue::IntValue(i) => GolemEventValue::IntValue(i.clone()),
        EventValue::FloatValue(fl) => GolemEventValue::FloatValue(fl.clone()),
        EventValue::BoolValue(b) => GolemEventValue::BoolValue(b.clone()),
    }
}

fn get_event_value(golem_event_value: GolemEventValue) -> EventValue {
    match golem_event_value {
        GolemEventValue::StringValue(s) => EventValue::StringValue(s.clone()),
        GolemEventValue::IntValue(i) => EventValue::IntValue(i.clone()),
        GolemEventValue::FloatValue(fl) => EventValue::FloatValue(fl.clone()),
        GolemEventValue::BoolValue(b) => EventValue::BoolValue(b.clone()),
    }
}