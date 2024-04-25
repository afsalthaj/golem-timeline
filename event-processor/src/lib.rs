use std::cell::RefCell;
use timeline::golem_event::GolemEventValue;
use timeline::state_dynamic_timeline::StateDynamicsTimeLine;
use crate::bindings::exports::timeline::event_processor::api::{Event, EventValue, Guest, WorkerId};

    mod bindings;

    struct Component;

    struct State {
        state_dynamic_timeline: StateDynamicsTimeLine<GolemEventValue>,
        col_name: Option<String>
    }

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        state_dynamic_timeline: StateDynamicsTimeLine::default(),
        col_name: None
    });

}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    let result = STATE.with_borrow_mut(|state| {
        f(state)
    });

    return result;
}

impl Guest for Component {
    fn initialize(worker: WorkerId) -> Result<String, String> {
        dbg!("Initiating raw events: {}", worker.name);
        with_state(|state| {
            state.col_name = Some("cdn_change".to_string()); //TODO; Get it from initialise
        });

        Ok("Succeeded".to_string())
    }

    fn latest_event_to_state(event: Event) -> Result<String, String> {

        with_state(|state| {
            let event_to_be_tracked = state.col_name.as_ref().expect("Illegal state. Worker uninitialized for a specific event");

            let event_value =
                event.event.iter().find(|(key, _)| key == event_to_be_tracked);

            match event_value {
                Some((_, value)) => {
                    let golem_event_value = match value {
                        EventValue::StringValue(s) => GolemEventValue::StringValue(s.clone()),
                        EventValue::IntValue(i) => GolemEventValue::IntValue(i.clone()),
                        EventValue::FloatValue(fl) => GolemEventValue::FloatValue(fl.clone()),
                        EventValue::BoolValue(b) => GolemEventValue::BoolValue(b.clone()),
                    };
                    state.state_dynamic_timeline.add_state_dynamic_info(event.time, golem_event_value);
                },
                None => {
                    dbg!("No event value found for the column name: {}", event_to_be_tracked);
                }
            }
        });

        Ok("Added event to the state dynamic event".to_string())

    }
}

