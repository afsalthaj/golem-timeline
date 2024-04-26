use std::fmt::{Debug, Display};

use crate::golem_event::{GolemEvent, GolemEventValue};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EventColumnName(pub String);
impl EventColumnName {
    pub fn equal_to<T: Debug + Clone>(self, value: EventColumnValue<T>) -> GolemEventPredicate<T> {
        GolemEventPredicate::Equals(self, value)
    }

    pub fn greater_than<T: Debug + Clone>(
        self,
        value: EventColumnValue<T>,
    ) -> GolemEventPredicate<T> {
        GolemEventPredicate::GreaterThan(self, value)
    }

    pub fn less_than<T: Debug + Clone>(self, value: EventColumnValue<T>) -> GolemEventPredicate<T> {
        GolemEventPredicate::LessThan(self, value)
    }
}

impl Display for EventColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct EventColumnValue<T: Debug + Clone>(pub T);

impl<T: Display + Debug + Clone> Display for EventColumnValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Debug + Clone> From<T> for EventColumnValue<T> {
    fn from(value: T) -> Self {
        EventColumnValue(value)
    }
}

pub fn string(value: &str) -> EventColumnValue<String> {
    EventColumnValue(value.to_string())
}

pub fn int(value: &str) -> EventColumnValue<i64> {
    EventColumnValue(value.parse().unwrap())
}

pub fn float(value: &str) -> EventColumnValue<f64> {
    EventColumnValue(value.parse().unwrap())
}

pub fn boolean(value: bool) -> EventColumnValue<bool> {
    EventColumnValue(value)
}

pub fn col(column_name: &str) -> EventColumnName {
    EventColumnName(column_name.to_string())
}

#[derive(Clone, Debug)]
pub enum GolemEventPredicate<T: Clone + Debug> {
    Equals(EventColumnName, EventColumnValue<T>),
    GreaterThan(EventColumnName, EventColumnValue<T>),
    LessThan(EventColumnName, EventColumnValue<T>),
    And(Box<GolemEventPredicate<T>>, Box<GolemEventPredicate<T>>),
    Or(Box<GolemEventPredicate<T>>, Box<GolemEventPredicate<T>>),
}

impl Display for GolemEventPredicate<GolemEventValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GolemEventPredicate::Equals(column, value) => write!(f, "{} == {}", column.0, value),
            GolemEventPredicate::GreaterThan(column, value) => {
                write!(f, "{} > {}", column.0, value)
            }
            GolemEventPredicate::LessThan(column, value) => write!(f, "{} < {}", column.0, value),
            GolemEventPredicate::And(left, right) => write!(f, "{} && {}", left, right),
            GolemEventPredicate::Or(left, right) => write!(f, "{} || {}", left, right),
        }
    }
}

impl<T: PartialEq + PartialOrd + Clone + Debug> GolemEventPredicate<T> {
    pub fn evaluate(&self, event: &GolemEvent<T>) -> bool {
        match self {
            GolemEventPredicate::Equals(event_column_name, event_value) => event
                .event
                .get(event_column_name)
                .map_or(false, |v| v == &event_value.0),

            GolemEventPredicate::GreaterThan(event_column_name, event_value) => event
                .event
                .get(event_column_name)
                .map_or(false, |v| v > &event_value.0),

            GolemEventPredicate::LessThan(event_column_name, event_value) => event
                .event
                .get(event_column_name)
                .map_or(false, |v| v < &event_value.0),
            GolemEventPredicate::And(left, right) => left.evaluate(event) && right.evaluate(event),
            GolemEventPredicate::Or(left, right) => left.evaluate(event) || right.evaluate(event),
        }
    }

    pub fn and(self, other: GolemEventPredicate<T>) -> GolemEventPredicate<T> {
        GolemEventPredicate::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: GolemEventPredicate<T>) -> GolemEventPredicate<T> {
        GolemEventPredicate::Or(Box::new(self), Box::new(other))
    }
}
