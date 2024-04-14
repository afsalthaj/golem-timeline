use crate::golem_event_value::GolemEventValue;

pub struct RawEventRecord {
    pub time: u64,
    pub event: GolemEventValue,
    pub key: String,
}

impl RawEventRecord {
    pub fn new(key: String, time: u64, event: GolemEventValue) -> RawEventRecord {
        RawEventRecord { time, event, key }
    }
}
