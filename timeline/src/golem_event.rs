use std::collections::HashMap;
use std::fmt::Display;

use crate::event_predicate::EventColumnName;

#[derive(Clone, Debug)]
pub struct GolemEvent<T> {
    pub time: u64,
    pub event: HashMap<EventColumnName, T>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum GolemEventValue {
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BoolValue(bool),
}

impl Display for GolemEventValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GolemEventValue::StringValue(s) => write!(f, "{}", s),
            GolemEventValue::IntValue(i) => write!(f, "{}", i),
            GolemEventValue::FloatValue(fl) => write!(f, "{}", fl),
            GolemEventValue::BoolValue(b) => write!(f, "{}", b),
        }
    }
}
