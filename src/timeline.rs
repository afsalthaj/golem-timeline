use crate::event_timeline::EventTimeLine;
use crate::state_dynamic_timeline_point::StateDynamicsTimeLinePoint;
use crate::state_dynamics_timeline::StateDynamicsTimeLine;

pub enum TimeLine<T> {
    StateDynamic(StateDynamicsTimeLine<T>),
    EventTime(EventTimeLine<T>)
}


impl<T> TimeLine<T> {
    pub fn state_points(&self) -> Option<&Vec<StateDynamicsTimeLinePoint<T>>> {
        match self {
            TimeLine::StateDynamic(value) => {
                Some(&value.points)
            }

            TimeLine::EventTime(_) => None
        }
    }
}