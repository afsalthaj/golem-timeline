use crate::event_timeline::EventTimeLine;
use crate::state_dynamic_timeline_point::StateDynamicsTimeLinePoint;
use crate::state_dynamics_timeline::StateDynamicsTimeLine;
use std::fmt::Debug;

pub enum TimeLine<T> {
    StateDynamic(StateDynamicsTimeLine<T>),
    EventTime(EventTimeLine<T>),
}

impl<T: Debug + Clone + Eq + PartialOrd> TimeLine<T> {
    pub fn add_info(&mut self, time: u64, value: T) {
        match self {
            TimeLine::StateDynamic(timeline) => {
                timeline.add_state_dynamic_info(time, value);
            }

            TimeLine::EventTime(timeline) => {
                timeline.add_event_info(time, value);
            }
        }
    }
}
