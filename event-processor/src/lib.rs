use std::cell::RefCell;
use crate::bindings::exports::timeline::event_processor::api::{Event, EventValue, Guest, WorkerId};
use core::state_dynamics_timeline::StateDynamicsTimeLine;
use crate::golem_event_value::GolemEventValue;

    mod bindings;
    pub mod raw_event;
    pub mod golem_event_value;

    struct Component;

    struct State {
        state_dynamic_timeline: StateDynamicsTimeLine<EventValue>
    }

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        state_dynamic_timeline: StateDynamicsTimeLine::default()
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
        Ok("Succeeded".to_string())
    }

    fn add_event(order: Event) {
        with_state(|state| {
            state.state_dynamic_timeline.add_state_dynamic_info(order.time, order.event);
        });
    }

    fn get_event(time: u64) -> Event {
        with_state(|state| {
            state.orders.iter().find(|event| event.time == time).unwrap().clone()
        })
    }

    fn get_events() -> Vec<Event> {
        with_state(|state| {
            state.orders.clone()
        })
    }
}

