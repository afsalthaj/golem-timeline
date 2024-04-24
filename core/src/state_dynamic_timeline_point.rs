use crate::zip_result::ZipResult;

// None represents, for the most of the time, future
#[derive(Clone, Debug, PartialEq)]
pub struct  StateDynamicsTimeLinePoint<T> {
    pub t1: u64,
    pub t2: Option<u64>,
    pub value: T,
}

impl StateDynamicsTimeLinePoint<bool> {
    pub fn is_true(&self) -> bool {
        self.value
    }

    pub fn is_false(&self) -> bool {
        !self.value
    }
}


impl<T: Clone> StateDynamicsTimeLinePoint<T> {
    pub fn to_zip_result(&self) -> StateDynamicsTimeLinePoint<ZipResult<T>> {
        StateDynamicsTimeLinePoint {
            t1: self.t1,
            t2: self.t2,
            value: ZipResult::Singleton(&self.value),
        }
    }

    pub fn contains(&self, t: u64) -> bool {
        if let Some(t2) = self.t2 {
            t >= self.t1 && t < t2
        } else {
            t >= self.t1
        }
    }
    pub fn is_mutually_exclusive(&self, other: &StateDynamicsTimeLinePoint<T>) -> bool {
        // first timeline's end is less than second timeline's start or
        // first timeline's start is greater than second timeline's end
        optional_less_than(self.t2, Some(other.t1))
            || optional_greater_than(Some(self.t1), other.t2)
    }
    pub fn update_t2(&mut self, t2: u64) {
        self.t2 = Some(t2);
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
    pub fn apply_f<F>(&self, f: &F) -> StateDynamicsTimeLinePoint<T>  where F: Fn(&ZipResult<T>) -> T,{
        StateDynamicsTimeLinePoint {
            t1: self.t1,
            t2: self.t2,
            value: f(&self.value),
        }
    }
}