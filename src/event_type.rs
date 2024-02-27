use crate::value::Value;

// Based on these event types we pre-assume a few things when buildinh timelines
// We disallow creation of broken timelines. For example: It doesn't make much sense for
// user to configure TL_HasExisted on top of a CIRR result. So it is far better to keep being
// conscious about event type through out building DSL. In other words, we want users to think about event types
// when operating on timelines.
// Another example, most of the times, it is the StateDynamic event timeline that can be converted to boolean timelines.
//   boolean
//      |
//     T|----
//     F|     -----------------
//      +----|----------------|---- t
//           t1               t2
//
//   boolean
//      |
//     T|---------------------
//     F|                      -----
//      +---------------------|---- t
//           t1               t2
//
// And
////       |
// //     T|---------------------
// //     F|                      -----
// //      +---------------------|---- t
// //           t1               t2
//
pub enum EventType {
    // This has value at each point in continuous time, and it can be different from one to the other. It keeps changing
    Numerical(u64), // CIRR

    // This has a value at each point in continuous time, however it changes only at dicsrete time
    StateDynamic(String), // CDN - a value at each point in continuous time, but changes at discrete points. Ex: CDN state

    // An event may not have a value at each point in continuous time, an event captures a sequence of discrete events
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