use crate::value::Value;

// More closer to state dynamic event
#[derive(Clone)]
struct EventState {
    column_of_state: String, // col("player_state_change_change")
}

// More closer to event
struct EventAction {
    column_of_action: String, // col("player_action")
}

enum EventIndex {
    StateDynamic(EventState),
    Event(EventAction),
}

impl EventIndex {
    fn equal_to(self, value: EventValue) -> EventPredicate {
        EventPredicate::Equals(self, value)
    }

    fn greater_than(self, value: EventValue) -> EventPredicate {
        EventPredicate::GreaterThan(self, value)
    }

    fn less_than(self, value: EventValue) -> EventPredicate {
        EventPredicate::LessThan(self, value)
    }
}

fn event(column_name: &str) -> EventIndex {
    EventIndex::Event(EventAction {
        column_of_action: column_name.to_string(),
    })
}
fn action(column_name: &str) -> EventIndex {
    EventIndex::StateDynamic(EventState {
        column_of_state: column_name.to_string(),
    })
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
    Equals(EventIndex, EventValue),
    GreaterThan(EventIndex, EventValue),
    LessThan(EventIndex, EventValue),
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
