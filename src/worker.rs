use crate::timeline::TimeLine;
use crate::value::Value;

pub struct Worker {
    pub key: WorkerKey,
    pub timeline: TimeLine
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct WorkerKey {
    // A time line is a worker
    pub time_line_op_name: String,
    // However we keep a timeline for a particular identity - example: playback_session_id
    pub identity: String,
}