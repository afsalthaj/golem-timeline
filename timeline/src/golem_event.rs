use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct GolemEvent {
    pub time: u64,
    pub event: HashMap<String, GolemEventValue>,
}


#[derive(Clone, PartialEq, Debug)]
pub enum GolemEventValue{
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