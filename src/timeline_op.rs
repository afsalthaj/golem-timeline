use crate::event_predicate::EventPredicate;
use crate::timeline::TimeLine;
use crate::event_stream::EventStream;

// In paper, it is referred as object DAG
pub enum TimeLineOp<T> {
    // Essentially based on paper, there is only numerical timeline and state dynamic timeline
    // A state dynamic is pretty much state that is dynamic. Consider this as a constant value
    // At this stage of development, I am not thinking of state dynamic in TimeLineOp, as I am not
    // sure of the exact implication of skipping this concept, and I wanted to see how it goes
    // during the timeline, while numerical keeps moving
    // A numerical timeline essentially cannot be pattern matched, as it is a continuous value
    // Refer paper to understand what these operations are
    Leaf(EventStream),
    EqualTo(TimeLineOp<T>, T),
    GreaterThan(TimeLineOp<T>, T),
    LessThan(TimeLineOp<T>, T),
    And(TimeLineOp<T>, TimeLineOp<T>),
    Or(TimeLineOp<T>, TimeLineOp<T>),
    Not(TimeLineOp<T>),

     // Each o the below functions invokes a worker
     // Each worker is responsible for forgetting past beyond an extent
     // This limitation exists in any real world system
     // This is more of tracking a StateDynamic event, as a cumulative OR
     // Input
     // t1: false
     // t2: true
     // t3: false
     // Output
     // t1-t2: false
     // t2-t3: true
    TlHasExisted(TimeLineOp<T>, EventPredicate),
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
    TlHasExistedWithin(TimeLineOp<T>, EventPredicate),
    // This is more or less making number of events to a very simple
    // timeline. Obviously this is corresponding to the events that are state dynamic in nature
    // t1 - t10 : CDN2
    // t10 - t11 : CDN1
    // t11- t12: CDN1
    // Output
    // t1-t10: CDN2
    // t10-t12: CDN1
    TlLatestEventToState(TimeLineOp<T>, EventPredicate),
    // A Numerical Timeline of
    // the cumulative duration
    // where the state was True
    // t1 - t3: false
    // t3 - t8: true
    // t8 - t14: false
    // t14 - t20: true
    // Output
    // t1 - t3: 0
    // t3 - t8 : 5
    // t8 - t4 : 5
    // t14 - t20: 11
    TlDurationWhere(TimeLineOp<T>, EventPredicate),

    // A Numerical Timeline of
    // the duration since the last
    // state change
    // t1-t3: buffer
    // t3-t8: play
    // t8-t14: buffer
    // t14-t20: pause
    // Output
    // t1-t3: 3
    // t3- t8: 5
    // t8-t14: 6
    // t14- t20: 6
    TlDurationInCurState(TimeLineOp<T>),
}

impl<T> TimeLineOp<T> {
    fn evaluate(&self) -> TimeLine<T> {
        unimplemented!("evaluate not implemented")
    }

   fn tl_has_existed(self, event_predicate: EventPredicate) -> TimeLineOp<bool> {
        TimeLineOp::TlHasExisted(self, event_predicate)
    }

    fn tl_has_existed_within(self, event_predicate: EventPredicate) -> TimeLineOp<bool> {
        TimeLineOp::TlHasExistedWithin(self, event_predicate)
    }

    fn tl_latest_event_to_state(self, event_predicate: EventPredicate) -> TimeLineOp<bool> {
        TimeLineOp::TlLatestEventToState(self, event_predicate)
    }

    fn tl_duration_where(self, event_predicate: EventPredicate) -> TimeLineOp<u64> {
        TimeLineOp::TlDurationWhere(self, event_predicate)
    }

    fn tl_duration_in_cur_state(self) -> TimeLineOp<u64> {
        TimeLineOp::TlDurationInCurState(self)
    }
}
