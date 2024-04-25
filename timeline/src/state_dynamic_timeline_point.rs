use crate::internals::zip_result::ZipResult;

// None represents, for the most of the time, future
#[derive(Clone, Debug, PartialEq)]
pub struct  StateDynamicsTimeLinePoint<T> {
    pub t1: u64,
    pub t2: Option<u64>,
    pub value: T,
}


impl<T: Clone> StateDynamicsTimeLinePoint<T> {

    pub fn contains(&self, t: u64) -> bool {
        if let Some(t2) = self.t2 {
            t >= self.t1 && t < t2
        } else {
            t >= self.t1
        }
    }
}

fn optional_less_than(left: Option<u64>, right: Option<u64>) -> bool {
    match (left, right) {
        (Some(l), Some(r)) => l < r,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => true,
    }
}

fn optional_greater_than(left: Option<u64>, right: Option<u64>) -> bool {
    match (left, right) {
        (Some(l), Some(r)) => l > r,
        (Some(_), None) => false,
        (None, Some(_)) => true,
        (None, None) => true,
    }
}

impl<'t, T: Clone> StateDynamicsTimeLinePoint<ZipResult<'t, T>> {
    pub fn apply_f<F>(&self, f: &F) -> StateDynamicsTimeLinePoint<T>  where F: Fn(&T, &T) -> T,{
        StateDynamicsTimeLinePoint {
            t1: self.t1,
            t2: self.t2,
            value: self.value.merge(&f),
        }
    }
}