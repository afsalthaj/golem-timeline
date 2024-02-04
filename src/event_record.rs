use crate::event_type::EventType;

struct RawEventRecord<T> {
    time: u64,
    event_type: EventType,
}

impl<T> RawEventRecord<T> {
    fn new(time: u64, event_type: EventType) -> RawEventRecord<T> {
        RawEventRecord {
            time,
            event_type,
        }
    }
}