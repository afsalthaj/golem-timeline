use crate::value::Value;

pub struct EventColumn(String);
impl EventColumn {
    pub fn equal_to(self, value: EventValue) -> EventPredicate {
        EventPredicate::Equals(self, value)
    }

    pub fn greater_than(self, value: EventValue) -> EventPredicate {
        EventPredicate::GreaterThan(self, value)
    }

    pub fn less_than(self, value: EventValue) -> EventPredicate {
        EventPredicate::LessThan(self, value)
    }
}

struct EventValue {
    pub value: Value,
}

impl EventValue {
    fn to_string(self) -> String {
        match self.value {
            Value::StringValue(value) => value,
            _ => panic!("Value is not a string"),
        }
    }
}

pub fn col(column_name: &str) -> EventColumn {
    EventColumn(column_name.to_string())
}

pub fn string_value(value: &str) -> EventValue {
    EventValue {
        value: Value::StringValue(value.to_string()),
    }
}

pub fn int_value(value: i64) -> EventValue {
    EventValue {
        value: Value::IntValue(value),
    }
}

pub fn float_value(value: f64) -> EventValue {
    EventValue {
        value: Value::FloatValue(value),
    }
}

// Event predicate can be inspected to filter which event to include in the timeline
pub enum EventPredicate {
    Equals(EventColumn, EventValue),
    GreaterThan(EventColumn, EventValue),
    LessThan(EventColumn, EventValue),
    And(Box<EventPredicate>, Box<EventPredicate>),
    Or(Box<EventPredicate>, Box<EventPredicate>),
}

impl EventPredicate {
    pub fn and(self, other: EventPredicate) -> EventPredicate {
        EventPredicate::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: EventPredicate) -> EventPredicate {
        EventPredicate::Or(Box::new(self), Box::new(other))
    }
}
