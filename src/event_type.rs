pub enum EventType {
    Numerical(u64), // CIRR
    StateDynamic(String), // CDN - a value at each point in continuous time, but changes at discrete points. Ex: CDN state
    Event(String), // Captures a sequence of discrete events. Ex: seek events, player state updates and CDN updates. Each point in time is a collection of one more events.
}