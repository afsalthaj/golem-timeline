use std::fmt::Display;

use crate::event_predicate::{EventColumnName, GolemEventPredicate};
use crate::golem_event::GolemEventValue;

/// The recursive Timeline DSL. Internal representation used for computation.
/// At the API boundary, we use a non-recursive graph encoding (see the component code).
#[derive(Clone, Debug)]
pub enum TimeLineOp {
    EqualTo(Box<TimeLineOp>, GolemEventValue),
    GreaterThan(Box<TimeLineOp>, GolemEventValue),
    GreaterThanOrEqual(Box<TimeLineOp>, GolemEventValue),
    LessThan(Box<TimeLineOp>, GolemEventValue),
    LessThanOrEqual(Box<TimeLineOp>, GolemEventValue),
    And(Box<TimeLineOp>, Box<TimeLineOp>),
    Or(Box<TimeLineOp>, Box<TimeLineOp>),
    Not(Box<TimeLineOp>),
    TlHasExisted(GolemEventPredicate<GolemEventValue>),
    TlHasExistedWithin(GolemEventPredicate<GolemEventValue>, u64),
    TlLatestEventToState(EventColumnName),
    TlDurationWhere(Box<TimeLineOp>),
    TlDurationInCurState(Box<TimeLineOp>),
}

pub fn tl_has_existed(predicate: GolemEventPredicate<GolemEventValue>) -> TimeLineOp {
    TimeLineOp::TlHasExisted(predicate)
}

pub fn tl_has_existed_within(
    predicate: GolemEventPredicate<GolemEventValue>,
    duration: u64,
) -> TimeLineOp {
    TimeLineOp::TlHasExistedWithin(predicate, duration)
}

pub fn tl_latest_event_to_state(event_col_name: EventColumnName) -> TimeLineOp {
    TimeLineOp::TlLatestEventToState(event_col_name)
}

pub fn tl_duration_where(op: TimeLineOp) -> TimeLineOp {
    TimeLineOp::TlDurationWhere(Box::new(op))
}

pub fn tl_duration_in_cur_state(op: TimeLineOp) -> TimeLineOp {
    TimeLineOp::TlDurationInCurState(Box::new(op))
}

pub fn tl_equal_to(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::EqualTo(Box::new(op), value)
}

pub fn tl_greater_than(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::GreaterThan(Box::new(op), value)
}

pub fn tl_greater_than_or_equal(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::GreaterThanOrEqual(Box::new(op), value)
}

pub fn tl_less_than(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::LessThan(Box::new(op), value)
}

pub fn tl_less_than_or_equal(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::LessThanOrEqual(Box::new(op), value)
}

pub fn tl_and(left: TimeLineOp, right: TimeLineOp) -> TimeLineOp {
    TimeLineOp::And(Box::new(left), Box::new(right))
}

pub fn tl_or(left: TimeLineOp, right: TimeLineOp) -> TimeLineOp {
    TimeLineOp::Or(Box::new(left), Box::new(right))
}

pub fn tl_not(op: TimeLineOp) -> TimeLineOp {
    TimeLineOp::Not(Box::new(op))
}

impl Display for TimeLineOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn text_of(v: &GolemEventValue) -> String {
            match v {
                GolemEventValue::StringValue(s) => s.to_string(),
                GolemEventValue::IntValue(i) => i.to_string(),
                GolemEventValue::BoolValue(b) => b.to_string(),
                GolemEventValue::FloatValue(fl) => fl.to_string(),
            }
        }

        match self {
            TimeLineOp::EqualTo(tl, v) => write!(f, "EqualTo({}, {})", tl, text_of(v)),
            TimeLineOp::GreaterThan(tl, v) => write!(f, "GreaterThan({}, {})", tl, text_of(v)),
            TimeLineOp::GreaterThanOrEqual(tl, v) => {
                write!(f, "GreaterThanOrEqual({}, {})", tl, text_of(v))
            }
            TimeLineOp::LessThan(tl, v) => write!(f, "LessThan({}, {})", tl, text_of(v)),
            TimeLineOp::LessThanOrEqual(tl, v) => {
                write!(f, "LessThanOrEqual({}, {})", tl, text_of(v))
            }
            TimeLineOp::And(l, r) => write!(f, "And({}, {})", l, r),
            TimeLineOp::Or(l, r) => write!(f, "Or({}, {})", l, r),
            TimeLineOp::Not(tl) => write!(f, "Not({})", tl),
            TimeLineOp::TlHasExisted(p) => write!(f, "TlHasExisted({})", p),
            TimeLineOp::TlHasExistedWithin(p, t) => write!(f, "TlHasExistedWithin({}, {})", p, t),
            TimeLineOp::TlLatestEventToState(c) => write!(f, "TlLatestEventToState({})", c),
            TimeLineOp::TlDurationWhere(tl) => write!(f, "TlDurationWhere({})", tl),
            TimeLineOp::TlDurationInCurState(tl) => write!(f, "TlDurationInCurState({})", tl),
        }
    }
}
