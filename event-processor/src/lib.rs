use crate::bindings::exports::timeline::event_processor::api::{
    Event, EventPredicate, EventStateResult, EventValue, Guest, LatestEventToStateResult,
    TimePeriod, WorkerId,
};
use crate::conversions::Conversion;
use std::cell::RefCell;
use timeline::event_predicate::{EventColumnName, GolemEventPredicate};
use timeline::golem_event::{GolemEvent, GolemEventValue};
use timeline::state_dynamic_timeline::StateDynamicsTimeLine;

mod bindings;
mod conversions;

struct Component;

struct LatestEventToStateTracker {
    state_dynamic_timeline: StateDynamicsTimeLine<GolemEventValue>,
    col_name: Option<EventColumnName>,
}

struct TLHasExistedTracker {
    state_dynamic_timeline: StateDynamicsTimeLine<bool>,
    predicate: Option<GolemEventPredicate<GolemEventValue>>,
}

thread_local! {
    static LATEST_EVENT_TO_STATE: RefCell<LatestEventToStateTracker> = RefCell::new(LatestEventToStateTracker {
        state_dynamic_timeline: StateDynamicsTimeLine::default(),
        col_name: None
    });

    static TL_HAS_EXISTED: RefCell<TLHasExistedTracker> = RefCell::new(TLHasExistedTracker {
        state_dynamic_timeline: StateDynamicsTimeLine::default(),
        predicate: None
    });
}

fn with_latest_event_to_state<T>(
    f: impl FnOnce(&mut LatestEventToStateTracker) -> Result<T, String>,
) -> Result<T, String> {
    let result = LATEST_EVENT_TO_STATE.with_borrow_mut(|state| f(state));

    return result;
}

fn with_tl_has_existed<T>(
    f: impl FnOnce(&mut TLHasExistedTracker) -> Result<T, String>,
) -> Result<T, String> {
    let result = TL_HAS_EXISTED.with_borrow_mut(|state| f(state));

    return result;
}

impl Guest for Component {
    fn initialize_latest_event_state(
        worker: WorkerId,
        event_column_name: String,
    ) -> Result<String, String> {
        with_latest_event_to_state(|state| {
            state.col_name = Some(EventColumnName(event_column_name.clone()));
            Ok(worker.name)
        })
    }

    fn initialize_tl_has_existed(
        worker: WorkerId,
        event_predicate: EventPredicate,
    ) -> Result<String, String> {
        with_tl_has_existed(|state| {
            state.predicate = Some(GolemEventPredicate::from_wit(event_predicate));
            Ok(worker.name)
        })
    }

    fn add_event(event: Event) -> Result<String, String> {
        with_latest_event_to_state(|state| {
            if let Some(state_col_name) = state.col_name.as_ref() {
                let event_value = event
                    .event
                    .iter()
                    .find(|(key, _)| key == state_col_name.0.as_str());

                match event_value {
                    Some((_, value)) => {
                        let golem_event_value = GolemEventValue::from_wit(value.clone());
                        state
                            .state_dynamic_timeline
                            .add_state_dynamic_info(event.time, golem_event_value);
                        dbg!(
                            "Added event for time {} for the latest-event-to-state of {}",
                            event.time,
                            &state_col_name.0
                        );
                    }
                    None => {
                        dbg!(
                            "No event value found for the column name: {}",
                            &state_col_name.0
                        );
                    }
                }
            };

            Ok(())
        })?;

        with_tl_has_existed(|state| {
            if let Some(predicate) = state.predicate.as_ref() {
                // If the timeline was empty, or timeline value is false already, then check predicate and set it to true once and for all
                // Nothing to do if this is already true, as we use `None` to indicate future, the timeline says true regardless of future timestamp
                // saving space and time
                if state.state_dynamic_timeline.is_empty()
                    || state.state_dynamic_timeline.future_is(false)
                {
                    let predicate_result = predicate.evaluate(&GolemEvent::from_wit(event.clone()));

                    if predicate_result {
                        dbg!(
                            "Setting timeline as true from time {} since the predicate is true!",
                            event.time
                        );
                        state
                            .state_dynamic_timeline
                            .add_state_dynamic_info(event.time, true);
                    } else {
                        // If predicate is false, and if the future is not yet set to false, then set it to false once and for all
                        if !state.state_dynamic_timeline.future_is(false) {
                            dbg!("Setting timeline as false from time {} since the predicate is false!", event.time);
                            state
                                .state_dynamic_timeline
                                .add_state_dynamic_info(event.time, false);
                        }
                    }
                }
            };

            Ok(())
        })?;

        Ok("Event tracked successfully".to_string())
    }

    fn latest_event_to_state(t1: u64) -> Result<LatestEventToStateResult, String> {
        with_latest_event_to_state(|state| {
            let latest_event = state.state_dynamic_timeline.get_state_at(t1);

            let column_name = state.col_name.as_ref().ok_or(
                "Latest Event To State hasn't been initialised or part of the workflow yet",
            )?;

            let result = match latest_event {
                Some(event) => LatestEventToStateResult {
                    event_col_name: column_name.0.clone(),
                    event_results: {
                        let event_result = EventStateResult {
                            time_period: TimePeriod {
                                t1: event.t1,
                                t2: event.t2.unwrap_or(u64::MAX),
                            },
                            value: event.value.to_wit(),
                        };
                        vec![event_result]
                    },
                },

                None => LatestEventToStateResult {
                    event_col_name: column_name.0.clone(),
                    event_results: vec![],
                },
            };

            Ok(result)
        })
    }

    fn tl_has_existed(t1: u64) -> Result<bool, String> {
        with_tl_has_existed(|state| {
            let result = state.state_dynamic_timeline.get_state_at(t1);

            result.map_or(Ok(false), |res| Ok(res.value))
        })
    }
}
