use crate::event_record::RawEventRecord;
use crate::value::Value;


pub enum EventStream {
    InMemoryEvents(InMemoryEventStream),
}


struct InMemoryEventStream {
    pub events: Vec<RawEventRecord<Value>>,
}

// Forward iterator for the event stream
impl Iterator for InMemoryEventStream {
    type Item = RawEventRecord<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.events.is_empty() {
            None // If the vector is empty, return None
        } else {
            // If there are events, remove and return the first one
            Some(self.events.remove(0))
        }

    }
}
