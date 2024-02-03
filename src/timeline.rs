use crate::event_type::EventType;
struct TimeLineRecord {
    time: f64,
    event_type: EventType,
    value: f64
}

impl TimeLineRecord {
    fn new(time: f64, event_type: EventType, value: f64) -> TimeLineRecord {
        TimeLineRecord {
            time,
            event_type,
            value
        }
    }
}

// This is more of tracking a StateDynamic event, as a cumulative OR
// Input
// t1: false
// t2: true
// t3: false

// Output
// t1-t2: false
// t2-t3: true
fn tl_has_existed() -> bool {
    unimplemented!("tl_has_existed not implemented")
}

