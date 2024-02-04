use crate::event_type::EventType;
struct TimeLineRecord<T> {
    time: f64,
    event_type: EventType,
    value: T
}

impl<T> TimeLineRecord<T> {
    fn new(time: f64, event_type: EventType, value: f64) -> TimeLineRecord<T> {
        TimeLineRecord {
            time,
            event_type,
            value
        }
    }
}

struct TimeLine<T> {
    t1: f64,
    t2: f64,
    value: T
}

// A timeline stream is essentially corresponding to the original timeine
// in the paper
enum TimeLineStream<T> {
    // Essentially based on paper, there is only numerical timeline and state dynamic timeline
    // A state dynamic is pretty much state that is dynamic. Consider this as a constant value
    // during the timeline, while numerical keeps moving
    Numerical(TimeLine<T>),
    StateDynamic(TimeLine<T>)
}

// This is more of tracking a StateDynamic event, as a cumulative OR
// Input
// t1: false
// t2: true
// t3: false

// Output
// t1-t2: false
// t2-t3: true
fn tl_has_existed<T>() -> TimeLineStream<T> {
    unimplemented!("tl_has_existed not implemented")
}

// This is more of tracking a StateDynamic event, as a cumulative OR
// Input
// Duration: D = 4
// t1: false
// t3: true
// t9: true

// Output
// t1-t3: false
// t3-t7: true
// t7-t9: false
// t9-t13: true
fn tl_has_existed_within<T>() -> TimeLineStream<T> {
    unimplemented!("tl_has_existed_within not implemented")
}


// This is more or less making number of events to a very simple
// timeline
fn tl_latest_event_to_state<T>() -> TimeLineStream<T> {
    unimplemented!("tl_latest_event_to_state not implemented")
}
