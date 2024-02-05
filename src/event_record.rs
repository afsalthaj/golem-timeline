use crate::event_type::EventType;

pub struct RawEventRecord {
    pub time: u64,
    pub event_type: EventType,
    pub key: String
}

impl RawEventRecord {
    pub fn new(key: String, time: u64, event_type: EventType) -> RawEventRecord {
        RawEventRecord {
            time,
            event_type,
            key
        }
    }
}