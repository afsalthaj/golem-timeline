use crate::value::Value;

// Having t2 as None allows
// us to assume for StateDynamic and Events together
// If None, may be the Value is an event
// And if not None, then it implies the Value is valid for the duration of t1 to t2
#[derive(Clone, Debug, PartialEq)]
pub struct TimeLinePoint {
    pub t1: u64,
    pub t2: Option<u64>,
    pub value: Value
}

impl TimeLinePoint {
    pub fn update_t2(&mut self, t2: u64) {
        self.t2 = Some(t2);
    }
}
