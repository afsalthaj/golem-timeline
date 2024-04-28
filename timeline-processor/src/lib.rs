use std::cell::RefCell;
use timeline::event_predicate::EventColumnName;
use timeline::golem_event::GolemEventValue;
use timeline::state_dynamic_timeline::StateDynamicsTimeLine;
use crate::bindings::exports::timeline::timeline_processor::api::{EventValue, Guest, TypedTimelineResultWorker};
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
//use crate::bindings::timeline::timeline_processor_stub::stub_timeline_processor;
use crate::bindings::golem::rpc::types::Uri;

mod bindings;

struct Component;

struct TLEqual {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>
}

struct TLGreaterThan {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>
}

struct TLGreaterThanOrEqualTo {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>
}

struct TLLessThan {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>
}

struct TLLessThanOrEqualTo {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>
}

struct TLAnd {
    child_worker1: Option<TypedTimelineResultWorker>,
    child_worker2: Option<TypedTimelineResultWorker>,
}

struct TLNot {
    child_worker: Option<TypedTimelineResultWorker>,
}

struct TLOr {
    child_worker1: Option<TypedTimelineResultWorker>,
    child_worker2: Option<TypedTimelineResultWorker>,
}


thread_local! {
    static TL_EQUAL_STATE: RefCell<TLEqual> = RefCell::new(TLEqual {
        child_worker: None,
        event_value: None
    });

    static TL_GREATER_THAN_STATE: RefCell<TLGreaterThan> = RefCell::new(TLGreaterThan {
        child_worker: None,
        event_value: None
    });

    static TL_GREATER_THAN_OR_EQUAL_TO_STATE: RefCell<TLGreaterThanOrEqualTo> = RefCell::new(TLGreaterThanOrEqualTo {
        child_worker: None,
        event_value: None
    });

    static TL_LESS_THAN_STATE: RefCell<TLLessThan> = RefCell::new(TLLessThan {
        child_worker: None,
        event_value: None
    });

    static TL_LESS_THAN_OR_EQUAL_TO_STATE: RefCell<TLLessThanOrEqualTo> = RefCell::new(TLLessThanOrEqualTo {
        child_worker: None,
        event_value: None
    });

    static TL_AND_STATE: RefCell<TLAnd> = RefCell::new(TLAnd {
        child_worker1: None,
        child_worker2: None
    });

    static TL_OR_STATE: RefCell<TLOr> = RefCell::new(TLOr {
        child_worker1: None,
        child_worker2: None
    });

    static TL_NOT_STATE: RefCell<TLNot> = RefCell::new( TLNot {
        child_worker: None
    });
}

fn with_equal_state<T>(
    f: impl FnOnce(&mut TLEqual) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_EQUAL_STATE.with_borrow_mut(|state| f(state));

    return result;
}

fn with_greater_than_state<T>(
    f: impl FnOnce(&mut TLGreaterThan) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_GREATER_THAN_STATE.with_borrow_mut(|state| f(state));

    return result;
}

fn with_greater_than_or_equal_to_state<T>(
    f: impl FnOnce(&mut TLGreaterThanOrEqualTo) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_GREATER_THAN_OR_EQUAL_TO_STATE.with_borrow_mut(|state| f(state));

    return result;
}

fn with_less_than_state<T>(
    f: impl FnOnce(&mut TLLessThan) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_LESS_THAN_STATE.with_borrow_mut(|state| f(state));

    return result;
}

fn with_less_than_or_equal_to_state<T>(
    f: impl FnOnce(&mut TLLessThanOrEqualTo) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_LESS_THAN_OR_EQUAL_TO_STATE.with_borrow_mut(|state| f(state));

    return result;
}

fn with_and_state<T>(
    f: impl FnOnce(&mut TLAnd) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_AND_STATE.with_borrow_mut(|state| f(state));

    return result;
}

fn with_or_state<T>(
    f: impl FnOnce(&mut TLOr) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_OR_STATE.with_borrow_mut(|state| f(state));

    return result;
}

fn with_not_state<T>(
    f: impl FnOnce(&mut TLNot) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_NOT_STATE.with_borrow_mut(|state| f(state));

    return result;
}



impl Guest for Component {
    fn initialize_equal(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        with_equal_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated equal computation worker".to_string())
        })?;

        Ok("Successfully initiated the worker to compute equals".to_string())
    }

    fn initialize_greater_than(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        with_greater_than_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated greater than computation worker".to_string())
        })?;

        Ok("Successfully initiated the worker to compute greater than".to_string())
    }

    fn initialize_greater_than_or_equal_to(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        with_greater_than_or_equal_to_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated greater than or equal to computation worker".to_string())
        })?;

        Ok("Successfully initiated the worker to compute greater than or equal to".to_string())
    }

    fn initialize_less_than(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        with_less_than_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated less than computation worker".to_string())
        })?;

        Ok("Successfully initiated the worker to compute less than".to_string())
    }

    fn initialize_less_than_or_equal_to(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        with_less_than_or_equal_to_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated less than or equal to computation worker".to_string())
        })?;

        Ok("Successfully initiated the worker to compute less than or equal to".to_string())
    }

    fn initialize_and(child_worker1: TypedTimelineResultWorker, child_worker2: TypedTimelineResultWorker) -> Result<String, String> {
        with_and_state(|state| {
            state.child_worker1 = Some(child_worker1);
            state.child_worker2 = Some(child_worker2);
            Ok("Successfully initiated and computation worker".to_string())
        })?;

        Ok("Successfully initiated the worker to compute and".to_string())
    }

    fn initialize_or(child_worker1: TypedTimelineResultWorker, child_worker2: TypedTimelineResultWorker) -> Result<String, String> {
        with_or_state(|state| {
            state.child_worker1 = Some(child_worker1);
            state.child_worker2 = Some(child_worker2);
            Ok("Successfully initiated or computation worker".to_string())
        })?;

        Ok("Successfully initiated the worker to compute or".to_string())
    }

    fn initialize_not(child_worker: TypedTimelineResultWorker) -> Result<String, String> {
        with_not_state(|state| {
            state.child_worker = Some(child_worker);
            Ok("Successfully initiated not computation worker".to_string())
        })?;

        Ok("Successfully initiated the worker to compute not".to_string())
    }
}