use crate::event_predicate::EventPredicate;
use crate::event_stream::EventStream;
use crate::state_dynamics_timeline::StateDynamicsTimeLine;
use crate::value::Value;
use crate::timeline::TimeLine;

// In paper, it is referred as object DAG
// TimeLineOp will produce numerical or state-dynamic timeline of a `Value` which can be (currently) string, int etc
// This implies your raw events needs to produce events, or state that can be coalesced to `Value`
pub enum TimeLineOp {
    // Essentially based on paper, there is only numerical timeline and state dynamic timeline
    // A state dynamic is pretty much state that is dynamic. Consider this as a constant value
    // At this stage of development, I am not thinking of state dynamic in TimeLineOp, as I am not
    // sure of the exact implication of skipping this concept, and I wanted to see how it goes
    // during the timeline, while numerical keeps moving
    // A numerical timeline essentially cannot be pattern matched, as it is a continuous value
    // Refer paper to understand what these operations are
    Leaf(EventStream),
    EqualTo(Box<TimeLineOp>, Value),
    GreaterThan(Box<TimeLineOp>, Value),
    LessThan(Box<TimeLineOp>, Value),
    And(Box<TimeLineOp>, Box<TimeLineOp>),
    Or(Box<TimeLineOp>, Box<TimeLineOp>),
    Not(Box<TimeLineOp>),

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
    TlHasExisted(Box<TimeLineOp>, EventPredicate<Value>),
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
    TlHasExistedWithin(Box<TimeLineOp>, EventPredicate<Value>),
    // This is more or less making number of events to a very simple
    // timeline. Obviously this is corresponding to the events that are state dynamic in nature
    // t1 - t10 : CDN2
    // t10 - t11 : CDN1
    // t11- t12: CDN1
    // Output
    // t1-t10: CDN2
    // t10-t12: CDN1
    TlLatestEventToState(Box<TimeLineOp>, EventPredicate<Value>),
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
    TlDurationWhere(Box<TimeLineOp>, EventPredicate<Value>),

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
    TlDurationInCurState(Box<TimeLineOp>),
}

impl TimeLineOp {
    fn is_boolean_timeline(&self) -> bool {
        match self {
            TimeLineOp::EqualTo(_, _) => true,
            TimeLineOp::GreaterThan(_, _) => true,
            TimeLineOp::LessThan(_, _) => true,
            TimeLineOp::And(_, _) => true,
            TimeLineOp::Or(_, _) => true,
            TimeLineOp::Not(_) => true,
            TimeLineOp::TlHasExisted(_, _) => true,
            TimeLineOp::TlHasExistedWithin(_, _) => true,
            TimeLineOp::TlLatestEventToState(_, _) => true,
            _ => false,
        }
    }

    fn evaluate(&self) -> TimeLine<Value> {
        unimplemented!("evaluate not implemented")
    }

    fn tl_has_existed(self, event_predicate: EventPredicate<Value>) -> TimeLineOp {
        TimeLineOp::TlHasExisted(Box::new(self), event_predicate)
    }

    fn tl_has_existed_within(self, event_predicate: EventPredicate<Value>) -> TimeLineOp {
        TimeLineOp::TlHasExistedWithin(Box::new(self), event_predicate)
    }

    fn tl_latest_event_to_state(self, event_predicate: EventPredicate<Value>) -> TimeLineOp {
        TimeLineOp::TlLatestEventToState(Box::new(self), event_predicate)
    }

    fn tl_duration_where(self, event_predicate: EventPredicate<Value>) -> TimeLineOp {
        TimeLineOp::TlDurationWhere(Box::new(self), event_predicate)
    }

    fn tl_duration_in_cur_state(self) -> TimeLineOp {
        TimeLineOp::TlDurationInCurState(Box::new(self))
    }
}
