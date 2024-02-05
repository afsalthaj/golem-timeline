use crate::value::Value;

#[derive(Clone, Debug)]
pub struct TimeLinePoint {
    pub t1: u64,
    pub t2: u64,
    pub value: Value
}
