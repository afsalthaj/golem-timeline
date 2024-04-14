use crate::bindings::exports::timeline::rawevents::api::{Event, Guest};

mod bindings;

struct Component;

struct State {
    orders: Vec<Event>,
}

static mut STATE: State = State {
    orders: Vec::new()
};

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    let result = unsafe { f(&mut STATE) };

    return result;
}

impl Guest for Component {
    fn add_event(order: Event) {
        with_state(|state| {
            state.orders.push(order);
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

