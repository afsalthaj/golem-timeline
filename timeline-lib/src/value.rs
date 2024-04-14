// Represents an event or state value

use std::cmp::Ordering;
use crate::bindings::exports::golem::timeline::api::EventValue as WitEventValue;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    StringValue(String),
    IntValue(i64),
    BooleanValue(bool),
    FloatValue(f64),
}

impl Eq for Value {}

impl From<WitEventValue> for Value {
    fn from(value: WitEventValue) -> Self {
        match value {
            WitEventValue::StringValue(value) => Value::StringValue(value),
            WitEventValue::IntValue(value) => Value::IntValue(value),
            WitEventValue::BoolValue(value) => Value::BooleanValue(value),
            WitEventValue::FloatValue(value) => Value::FloatValue(value),
        }
    }
}



impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::StringValue(value) => value.clone(),
            _ => panic!("Value is not a string"),
        }
    }
}
