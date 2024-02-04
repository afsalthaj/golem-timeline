use crate::value::Value;

pub enum EventType {
    Numerical(u64), // CIRR
    StateDynamic(String), // CDN - a value at each point in continuous time, but changes at discrete points. Ex: CDN state
    Event(String), // Captures a sequence of discrete events. Ex: seek events, player state updates and CDN updates. Each point in time is a collection of one more events.
}

impl EventType {
   pub fn to_value(&self)  -> Value {
        match self {
            EventType::Numerical(value) => Value::IntValue(*value as i64),
            EventType::StateDynamic(value) => Value::StringValue(value.to_string()),
            EventType::Event(value) => Value::StringValue(value.to_string()),
        }
    }
}