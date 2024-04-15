use std::fmt::Display;
use crate::bindings::timeline::raw_events::api::EventValue as GolemEventValue;
//use crate::bindings::exports::golem::timeline::api::FilterOp;
//use crate::bindings::exports::golem::timeline::api::EventValue as WitEventValue;

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

// A much more generic structure that act as a new type and more safer to use across the core
// Mostly decorated with raw-event component's actual EventValue
pub struct EventValue<T> {
    pub value: T,
}

impl Display for EventValue<GolemEventValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            GolemEventValue::StringValue(value) => write!(f, "{}", value),
            GolemEventValue::IntValue(value) => write!(f, "{}", value),
            GolemEventValue::BoolValue(value) => write!(f, "{}", value),
            GolemEventValue::FloatValue(value) => write!(f, "{}", value),
        }
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

impl Display for EventPredicate<GolemEventValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventPredicate::Equals(column, value) => write!(f, "{} == {}", column.0, value),
            EventPredicate::GreaterThan(column, value) => write!(f, "{} > {}", column.0, value),
            EventPredicate::LessThan(column, value) => write!(f, "{} < {}", column.0, value),
            EventPredicate::And(left, right) => write!(f, "{} && {}", left, right),
            EventPredicate::Or(left, right) => write!(f, "{} || {}", left, right),
        }
    }
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
