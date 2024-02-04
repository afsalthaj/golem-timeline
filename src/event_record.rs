use crate::event_type::EventType;

pub struct RawEventRecord<T> {
    pub time: u64,
    pub event_type: EventType,
    pub key: String
}

impl<T> RawEventRecord<T> {
    fn new(key: String, time: u64, event_type: EventType) -> RawEventRecord<T> {
        RawEventRecord {
            time,
            event_type,
            key
        }
    }
}