use std::fmt::Display;
use raw_events::golem_event_value::GolemEventValue;
use crate::bindings::exports::golem::timeline::api::FilterOp;
use crate::bindings::exports::golem::timeline::api::EventValue as WitEventValue;
pub struct EventColumn(pub String);
impl EventColumn {
    pub fn equal_to<T>(self, value: EventValue<T>) -> EventPredicate<T> {
        EventPredicate::Equals(self, value)
    }

    pub fn greater_than<T>(self, value: EventValue<T>) -> EventPredicate<T> {
        EventPredicate::GreaterThan(self, value)
    }

    pub fn less_than<T>(self, value: EventValue<T>) -> EventPredicate<T> {
        EventPredicate::LessThan(self, value)
    }
}

pub struct EventValue<T> {
    pub value: T,
}

impl From<crate::bindings::exports::golem::timeline::api::EventValue> for EventValue<GolemEventValue> {
    fn from(value: crate::bindings::exports::golem::timeline::api::EventValue) -> Self {
       let value: GolemEventValue =  match value {
           WitEventValue::StringValue(value) => GolemEventValue::StringValue(value),
           WitEventValue::IntValue(value) => GolemEventValue::IntValue(value),
           WitEventValue::BoolValue(value) => GolemEventValue::BooleanValue(value),
           WitEventValue::FloatValue(value) => GolemEventValue::FloatValue(value),
       };

        value.into()
    }
}

impl From<GolemEventValue> for EventValue<GolemEventValue> {
    fn from(value: GolemEventValue) -> Self {
        EventValue {
            value,
        }
    }
}

pub fn string(value: &str) -> EventValue<String> {
    EventValue {
        value: value.to_string(),
    }
}

pub fn col(column_name: &str) -> EventColumn {
    EventColumn(column_name.to_string())
}

pub fn string_value(value: &str) -> EventValue<GolemEventValue> {
    EventValue {
        value: GolemEventValue::StringValue(value.to_string()),
    }
}

pub fn int_value(value: i64) -> EventValue<GolemEventValue> {
    EventValue {
        value: GolemEventValue::IntValue(value),
    }
}

// Event predicate can be inspected to filter which event to include in the timeline
pub enum EventPredicate<T> {
    Equals(EventColumn, EventValue<T>),
    GreaterThan(EventColumn, EventValue<T>),
    LessThan(EventColumn, EventValue<T>),
    And(Box<EventPredicate<T>>, Box<EventPredicate<T>>),
    Or(Box<EventPredicate<T>>, Box<EventPredicate<T>>),
}

impl<T: Eq + PartialOrd> EventPredicate<T> {
    pub fn evaluate(&self, original_value: &T) -> bool {
        match self {
            EventPredicate::Equals(_, event_value) => original_value == &event_value.value,
            EventPredicate::GreaterThan(_, event_value) => original_value > &event_value.value,
            EventPredicate::LessThan(_, event_value) => original_value < &event_value.value,
            EventPredicate::And(left, right) => {
                left.evaluate(original_value) && right.evaluate(original_value)
            }
            EventPredicate::Or(left, right) => {
                left.evaluate(original_value) || right.evaluate(original_value)
            }
        }
    }
    pub fn and(self, other: EventPredicate<T>) -> EventPredicate<T> {
        EventPredicate::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: EventPredicate<T>) -> EventPredicate<T> {
        EventPredicate::Or(Box::new(self), Box::new(other))
    }
}
