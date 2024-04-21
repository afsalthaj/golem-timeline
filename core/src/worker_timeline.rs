use crate::bindings::timeline::event_processor::api::EventValue;
use crate::state_dynamics_timeline::StateDynamicsTimeLine;
use crate::timeline::TimeLine;


// The data that each worker needs to care about
pub struct WorkerTimeLineData {
    pub key: WorkerKey,
    pub timeline: TimeLine<EventValue>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct WorkerKey {
    // A time line is a worker
    pub time_line_op_name: String,
    // However we keep a timeline for a particular identity - example: playback_session_id - afsal's
    pub identity: String,
}
