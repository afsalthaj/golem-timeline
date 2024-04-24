// Represents an event or state value

use std::cmp::Ordering;
use crate::bindings::exports::timeline::event_processor::api::EventValue as WitEventValue;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum GolemEventValue {
    StringValue(String),
    IntValue(i64),
    BooleanValue(bool),
    FloatValue(f64),
}

impl Eq for GolemEventValue {}

impl From<WitEventValue> for GolemEventValue {
    fn from(value: WitEventValue) -> Self {
        match value {
            WitEventValue::StringValue(value) => GolemEventValue::StringValue(value),
            WitEventValue::IntValue(value) => GolemEventValue::IntValue(value),
            WitEventValue::BoolValue(value) => GolemEventValue::BooleanValue(value),
            WitEventValue::FloatValue(value) => GolemEventValue::FloatValue(value),
        }
    }
}

impl Into<WitEventValue> for GolemEventValue {
    fn into(self) -> WitEventValue {
        match self {
            GolemEventValue::StringValue(value) => WitEventValue::StringValue(value),
            GolemEventValue::IntValue(value) => WitEventValue::IntValue(value),
            GolemEventValue::BooleanValue(value) => WitEventValue::BoolValue(value),
            GolemEventValue::FloatValue(value) => WitEventValue::FloatValue(value),
        }
    }
}

impl GolemEventValue {
    fn from_wit(value: WitEventValue) -> Self {
        match value {
            WitEventValue::StringValue(value) => GolemEventValue::StringValue(value),
            WitEventValue::IntValue(value) => GolemEventValue::IntValue(value),
            WitEventValue::BoolValue(value) => GolemEventValue::BooleanValue(value),
            WitEventValue::FloatValue(value) => GolemEventValue::FloatValue(value),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            GolemEventValue::StringValue(value) => value.clone(),
            _ => panic!("Value is not a string"),
        }
    }
}
