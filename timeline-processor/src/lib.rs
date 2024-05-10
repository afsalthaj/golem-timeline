use crate::bindings::exports::timeline::timeline_processor::api::{
    EventValue, Guest, TimelineResult, TypedTimelineResultWorker,
};

use conversions::Conversion;
use extensions::WorkerResultExt;
use std::cell::RefCell;

use timeline::golem_event::GolemEventValue;
use timeline::state_dynamic_timeline::StateDynamicsTimeLine;

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;
mod conversions;
mod extensions;

struct Component;

struct TLEqual {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>,
}

struct TLGreaterThan {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>,
}

struct TLGreaterThanOrEqualTo {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>,
}

struct TLLessThan {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>,
}

struct TLLessThanOrEqualTo {
    child_worker: Option<TypedTimelineResultWorker>,
    event_value: Option<EventValue>,
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
    static ACTIVE_STATE: RefCell<Option<ActiveState>> = const { RefCell::new(None) };

    static TL_EQUAL_STATE: RefCell<TLEqual> = const {RefCell::new(TLEqual {
        child_worker: None,
        event_value: None
    })};

    static TL_GREATER_THAN_STATE: RefCell<TLGreaterThan> = const { RefCell::new(TLGreaterThan {
        child_worker: None,
        event_value: None
    })};

    static TL_GREATER_THAN_OR_EQUAL_TO_STATE: RefCell<TLGreaterThanOrEqualTo> = const { RefCell::new(TLGreaterThanOrEqualTo {
        child_worker: None,
        event_value: None
    })};

    static TL_LESS_THAN_STATE: RefCell<TLLessThan> = const {RefCell::new(TLLessThan {
        child_worker: None,
        event_value: None
    })};

    static TL_LESS_THAN_OR_EQUAL_TO_STATE: RefCell<TLLessThanOrEqualTo> = const { RefCell::new(TLLessThanOrEqualTo {
        child_worker: None,
        event_value: None
    })};

    static TL_AND_STATE: RefCell<TLAnd> = const { RefCell::new(TLAnd {
        child_worker1: None,
        child_worker2: None
    })};

    static TL_OR_STATE: RefCell<TLOr> = const {RefCell::new(TLOr {
        child_worker1: None,
        child_worker2: None
    })};

    static TL_NOT_STATE: RefCell<TLNot> = const { RefCell::new( TLNot {
        child_worker: None
    })};
}

fn with_equal_state<T>(f: impl FnOnce(&mut TLEqual) -> Result<T, String>) -> Result<T, String> {
    TL_EQUAL_STATE.with_borrow_mut(|state| f(state))
}

fn with_greater_than_state<T>(
    f: impl FnOnce(&mut TLGreaterThan) -> Result<T, String>,
) -> Result<T, String> {
    TL_GREATER_THAN_STATE.with_borrow_mut(|state| f(state))
}

fn with_greater_than_or_equal_to_state<T>(
    f: impl FnOnce(&mut TLGreaterThanOrEqualTo) -> Result<T, String>,
) -> Result<T, String> {
    TL_GREATER_THAN_OR_EQUAL_TO_STATE.with_borrow_mut(|state| f(state))
}

fn with_less_than_state<T>(
    f: impl FnOnce(&mut TLLessThan) -> Result<T, String>,
) -> Result<T, String> {
    TL_LESS_THAN_STATE.with_borrow_mut(|state| f(state))
}

fn with_less_than_or_equal_to_state<T>(
    f: impl FnOnce(&mut TLLessThanOrEqualTo) -> Result<T, String>,
) -> Result<T, String> {
    TL_LESS_THAN_OR_EQUAL_TO_STATE.with_borrow_mut(|state| f(state))
}

fn with_and_state<T>(f: impl FnOnce(&mut TLAnd) -> Result<T, String>) -> Result<T, String> {
    TL_AND_STATE.with_borrow_mut(|state| f(state))
}

fn with_or_state<T>(f: impl FnOnce(&mut TLOr) -> Result<T, String>) -> Result<T, String> {
    TL_OR_STATE.with_borrow_mut(|state| f(state))
}

fn with_not_state<T>(f: impl FnOnce(&mut TLNot) -> Result<T, String>) -> Result<T, String> {
    TL_NOT_STATE.with_borrow_mut(|state| f(state))
}

fn with_active_state<T>(f: impl FnOnce(&mut Option<ActiveState>) -> T) -> T {
    ACTIVE_STATE.with_borrow_mut(|state| f(state))
}

impl Guest for Component {
    fn initialize_equal(
        child_worker: TypedTimelineResultWorker,
        event_value: EventValue,
    ) -> Result<String, String> {
        with_equal_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated equal computation worker".to_string())
        })?;

        with_active_state(|state| {
            *state = Some(ActiveState::Equal);
        });

        Ok("Successfully initiated the worker to compute equals".to_string())
    }

    fn initialize_greater_than(
        child_worker: TypedTimelineResultWorker,
        event_value: EventValue,
    ) -> Result<String, String> {
        with_greater_than_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated greater than computation worker".to_string())
        })?;

        with_active_state(|state| {
            *state = Some(ActiveState::GreaterThan);
        });

        Ok("Successfully initiated the worker to compute greater than".to_string())
    }

    fn initialize_greater_than_or_equal_to(
        child_worker: TypedTimelineResultWorker,
        event_value: EventValue,
    ) -> Result<String, String> {
        with_greater_than_or_equal_to_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated greater than or equal to computation worker".to_string())
        })?;

        with_active_state(|state| {
            *state = Some(ActiveState::GreaterThanOrEqualTo);
        });

        Ok("Successfully initiated the worker to compute greater than or equal to".to_string())
    }

    fn initialize_less_than(
        child_worker: TypedTimelineResultWorker,
        event_value: EventValue,
    ) -> Result<String, String> {
        with_less_than_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated less than computation worker".to_string())
        })?;

        with_active_state(|state| {
            *state = Some(ActiveState::LessThan);
        });

        Ok("Successfully initiated the worker to compute less than".to_string())
    }

    fn initialize_less_than_or_equal_to(
        child_worker: TypedTimelineResultWorker,
        event_value: EventValue,
    ) -> Result<String, String> {
        with_less_than_or_equal_to_state(|state| {
            state.child_worker = Some(child_worker);
            state.event_value = Some(event_value);
            Ok("Successfully initiated less than or equal to computation worker".to_string())
        })?;

        with_active_state(|state| {
            *state = Some(ActiveState::LessThanOrEqualTo);
        });

        Ok("Successfully initiated the worker to compute less than or equal to".to_string())
    }

    fn initialize_and(
        child_worker1: TypedTimelineResultWorker,
        child_worker2: TypedTimelineResultWorker,
    ) -> Result<String, String> {
        with_and_state(|state| {
            state.child_worker1 = Some(child_worker1);
            state.child_worker2 = Some(child_worker2);
            Ok("Successfully initiated and computation worker".to_string())
        })?;

        with_active_state(|state| {
            *state = Some(ActiveState::And);
        });

        Ok("Successfully initiated the worker to compute and".to_string())
    }

    fn initialize_or(
        child_worker1: TypedTimelineResultWorker,
        child_worker2: TypedTimelineResultWorker,
    ) -> Result<String, String> {
        with_or_state(|state| {
            state.child_worker1 = Some(child_worker1);
            state.child_worker2 = Some(child_worker2);
            Ok("Successfully initiated or computation worker".to_string())
        })?;

        with_active_state(|state| {
            *state = Some(ActiveState::Or);
        });

        Ok("Successfully initiated the worker to compute or".to_string())
    }

    fn initialize_not(child_worker: TypedTimelineResultWorker) -> Result<String, String> {
        with_not_state(|state| {
            state.child_worker = Some(child_worker);
            Ok("Successfully initiated not computation worker".to_string())
        })?;

        with_active_state(|state| {
            *state = Some(ActiveState::Not);
        });

        Ok("Successfully initiated the worker to compute not".to_string())
    }

    fn get_timeline_result(t1: u64) -> Result<TimelineResult, String> {
        with_active_state(|state| match state {
            Some(ActiveState::Equal) => with_equal_state(|state| {
                let child_worker = state.child_worker.as_ref().unwrap();
                let event_value = state.event_value.as_ref().unwrap();
                let golem_event_value = GolemEventValue::from_wit(event_value.clone());
                let time_line_result = child_worker.get_timeline_result(t1)?;
                let state_dynamic_timeline = StateDynamicsTimeLine::from_wit(time_line_result);
                let result = state_dynamic_timeline
                    .equal_to(golem_event_value)
                    .map(|x| GolemEventValue::BoolValue(*x));

                Ok(result.to_wit())
            }),
            Some(ActiveState::GreaterThan) => with_greater_than_state(|state| {
                let child_worker = state.child_worker.as_ref().unwrap();
                let event_value = state.event_value.as_ref().unwrap();
                let golem_event_value = GolemEventValue::from_wit(event_value.clone());
                let time_line_result = child_worker.get_timeline_result(t1)?;
                let state_dynamic_timeline = StateDynamicsTimeLine::from_wit(time_line_result);
                let result = state_dynamic_timeline
                    .greater_than(golem_event_value)
                    .map(|x| GolemEventValue::BoolValue(*x));

                Ok(result.to_wit())
            }),
            Some(ActiveState::GreaterThanOrEqualTo) => {
                with_greater_than_or_equal_to_state(|state| {
                    let child_worker = state.child_worker.as_ref().unwrap();
                    let event_value = state.event_value.as_ref().unwrap();
                    let golem_event_value = GolemEventValue::from_wit(event_value.clone());
                    let time_line_result = child_worker.get_timeline_result(t1)?;
                    let state_dynamic_timeline = StateDynamicsTimeLine::from_wit(time_line_result);
                    let result = state_dynamic_timeline
                        .greater_than_or_equal_to(golem_event_value)
                        .map(|x| GolemEventValue::BoolValue(*x));

                    Ok(result.to_wit())
                })
            }

            Some(ActiveState::LessThan) => with_less_than_state(|state| {
                let child_worker = state.child_worker.as_ref().unwrap();
                let event_value = state.event_value.as_ref().unwrap();
                let golem_event_value = GolemEventValue::from_wit(event_value.clone());
                let time_line_result = child_worker.get_timeline_result(t1)?;
                let state_dynamic_timeline = StateDynamicsTimeLine::from_wit(time_line_result);
                let result = state_dynamic_timeline
                    .less_than(golem_event_value)
                    .map(|x| GolemEventValue::BoolValue(*x));

                Ok(result.to_wit())
            }),

            Some(ActiveState::LessThanOrEqualTo) => with_less_than_or_equal_to_state(|state| {
                let child_worker = state.child_worker.as_ref().unwrap();
                let event_value = state.event_value.as_ref().unwrap();
                let golem_event_value = GolemEventValue::from_wit(event_value.clone());
                let time_line_result = child_worker.get_timeline_result(t1)?;
                let state_dynamic_timeline = StateDynamicsTimeLine::from_wit(time_line_result);
                let result = state_dynamic_timeline
                    .less_than_or_equal_to(golem_event_value)
                    .map(|x| GolemEventValue::BoolValue(*x));

                Ok(result.to_wit())
            }),

            Some(ActiveState::And) => with_and_state(|state| {
                let child_worker1 = state.child_worker1.as_ref().unwrap();
                let child_worker2 = state.child_worker2.as_ref().unwrap();
                let time_line_result1 = child_worker1.get_timeline_result(t1)?;
                let time_line_result2 = child_worker2.get_timeline_result(t1)?;
                let state_dynamic_timeline1 = StateDynamicsTimeLine::from_wit(time_line_result1)
                    .map_fallible(|x| {
                        x.get_bool().ok_or(
                            "Timeline is not a boolean timeline to apply AND logic".to_string(),
                        )
                    })?;
                let state_dynamic_timeline2 = StateDynamicsTimeLine::from_wit(time_line_result2)
                    .map_fallible(|x| {
                        x.get_bool().ok_or(
                            "Timeline is not a boolean timeline to apply AND logic".to_string(),
                        )
                    })?;
                let result = state_dynamic_timeline1
                    .and(state_dynamic_timeline2)
                    .map(|x| GolemEventValue::BoolValue(*x));

                Ok(result.to_wit())
            }),

            Some(ActiveState::Or) => with_or_state(|state| {
                let child_worker1 = state.child_worker1.as_ref().unwrap();
                let child_worker2 = state.child_worker2.as_ref().unwrap();
                let time_line_result1 = child_worker1.get_timeline_result(t1)?;
                let time_line_result2 = child_worker2.get_timeline_result(t1)?;
                let state_dynamic_timeline1 = StateDynamicsTimeLine::from_wit(time_line_result1)
                    .map_fallible(|x| {
                        x.get_bool().ok_or(
                            "Timeline is not a boolean timeline to apply OR logic".to_string(),
                        )
                    })?;
                let state_dynamic_timeline2 = StateDynamicsTimeLine::from_wit(time_line_result2)
                    .map_fallible(|x| {
                        x.get_bool().ok_or(
                            "Timeline is not a boolean timeline to apply OR logic".to_string(),
                        )
                    })?;
                let result = state_dynamic_timeline1
                    .or(state_dynamic_timeline2)
                    .map(|x| GolemEventValue::BoolValue(*x));

                Ok(result.to_wit())
            }),

            Some(ActiveState::Not) => with_not_state(|state| {
                let child_worker = state.child_worker.as_ref().unwrap();
                let time_line_result = child_worker.get_timeline_result(t1)?;
                let state_dynamic_timeline = StateDynamicsTimeLine::from_wit(time_line_result);
                let result = state_dynamic_timeline.map_fallible(|x| {
                    x.get_bool()
                        .map(|bool| GolemEventValue::BoolValue(!bool))
                        .ok_or("Timeline is not a boolean timeline to apply NOT logic".to_string())
                })?;

                Ok(result.to_wit())
            }),

            None => Err("No active state to compute in derived timeline workers".to_string()),
        })
    }
}

enum ActiveState {
    Equal,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    And,
    Or,
    Not,
}
