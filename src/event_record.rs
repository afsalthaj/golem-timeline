use crate::value::Value;

pub struct RawEventRecord {
    pub time: u64,
    pub event: Value,
    pub key: String,
}

impl RawEventRecord {
    pub fn new(key: String, time: u64, event: Value) -> RawEventRecord {
        RawEventRecord {
            time,
            event,
            key,
        }
    }
}
