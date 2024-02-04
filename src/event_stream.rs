use crate::event_record::RawEventRecord;
use crate::value::Value;

enum EventStream {
    InMemoryEvents(Vec<RawEventRecord<Value>>),
}

// Forward iterator for the event stream
impl Iterator for EventStream {
    type Item = RawEventRecord<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EventStream::InMemoryEvents(events) => {
                // Check if there are events in the vector
                if events.is_empty() {
                    None // If the vector is empty, return None
                } else {
                    // If there are events, remove and return the first one
                    Some(events.remove(0))
                }
            }
        }
    }
}