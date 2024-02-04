use crate::value::Value;

// More closer to state dynamic event
#[derive(Clone)]
struct EventState {
    column_of_state : String, // col("player_state_change_change")
}

// More closer to event
struct EventAction {
    column_of_action : String, // col("player_action")
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
    EventIndex:: Event (EventAction {
        column_of_action: column_name.to_string()
    })
}

// Bring typesafety
// A string can be action or state
// col("state") == "buffer" && col("action") == "seek"
fn col(event: &str) -> EventIndex {
    if event.contains("state") {
        EventIndex::StateDynamic(EventState {
            column_of_state: event.to_string()
        })
    } else {
        EventIndex::Event(EventAction {
            column_of_action: event.to_string()
        })
    }
}


struct EventValue {
    value: Value
}

pub fn string_value(value: &str) -> EventValue {
    EventValue {
        value: Value::StringValue(value.to_string())
    }
}

pub fn int_value(value: i64) -> EventValue {
    EventValue {
        value: Value::IntValue(value)
    }
}

pub fn float_value(value: f64) -> EventValue {
    EventValue {
        value: Value::FloatValue(value)
    }
}

pub enum EventPredicate {
    Equals(EventIndex, EventValue),
    GreaterThan(EventIndex, EventValue),
    LessThan(EventIndex, EventValue),
    And(EventPredicate, EventPredicate),
    Or(EventPredicate, EventPredicate),
}

impl EventPredicate {
    pub fn and(self, other: EventPredicate) -> EventPredicate {
        EventPredicate::And(self, other)
    }

    pub fn or(self, other: EventPredicate) -> EventPredicate {
        EventPredicate::Or(self, other)
    }
}