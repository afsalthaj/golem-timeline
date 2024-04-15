use std::fmt::Display;
use crate::bindings::timeline::raw_events::api::EventValue as GolemEventValue;
//use crate::bindings::exports::golem::timeline::api::{/*FilterOp, TimelineNode, TimelineOp as WitTimeLineOp, TimelinePrimitiveOp};
use crate::event_predicate::{EventColumn, EventPredicate, EventValue};
use crate::timeline::TimeLine;
use crate::bindings::exports::timeline::core::api::{FilterOp, TimelineNode, TimelineOp as WitTimeLineOp, TimelinePrimitiveOp};


// In paper, it is referred as object DAG
// TimeLineOp will produce numerical or state-dynamic timeline of a `Value` which can be (currently) string, int etc
// This implies your raw events needs to produce events, or state that can be coalesced to `Value`

// Id annotation for each node
// timeline-op Id + integer Id

pub enum TimeLineOp {
    // Pretty much represents the event-timeline (not state dynamics) - source (through workerid) and collection
    Leaf(), // A leaf node results in a component that exposes a function accepting event and storing it in a configurable buffer
    EqualTo(Box<TimeLineOp>, GolemEventValue),
    GreaterThan(Box<TimeLineOp>, GolemEventValue),
    GreaterThanOrEqual(Box<TimeLineOp>, GolemEventValue),
    LessThan(Box<TimeLineOp>, GolemEventValue),
    LessThanOrEqual(Box<TimeLineOp>, GolemEventValue),
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
    TlHasExisted(Box<TimeLineOp>, EventPredicate<GolemEventValue>),
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
    TlHasExistedWithin(Box<TimeLineOp>, EventPredicate<GolemEventValue>, u64),
    // This is more or less making number of events to a very simple
    // timeline. Obviously this is corresponding to the events that are state dynamic in nature
    // t1 - t10 : CDN2
    // t10 - t11 : CDN1
    // t11- t12: CDN1
    // Output
    // t1-t10: CDN2
    // t10-t12: CDN1
    TlLatestEventToState(Box<TimeLineOp>, EventPredicate<GolemEventValue>),
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
    TlDurationWhere(Box<TimeLineOp>, EventPredicate<GolemEventValue>),

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
    TlDurationInCurState(Box<TimeLineOp>, EventPredicate<GolemEventValue>),
}

impl Display for TimeLineOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn text_of(golem_event_value: &GolemEventValue) -> String {
            match golem_event_value {
                GolemEventValue::StringValue(value) => value.to_string(),
                GolemEventValue::IntValue(value) => value.to_string(),
                GolemEventValue::BoolValue(value) => value.to_string(),
                GolemEventValue::FloatValue(value) => value.to_string(),
            }
        }

        match self {
            TimeLineOp::Leaf() => write!(f, "Leaf"),
            TimeLineOp::EqualTo(tl, value) => write!(f, "EqualTo({}, {})", tl, text_of(value)),
            TimeLineOp::GreaterThan(tl, value) => write!(f, "GreaterThan({}, {})", tl, text_of(value)),
            TimeLineOp::GreaterThanOrEqual(tl, value) => write!(f, "GreaterThanOrEqual({}, {})", tl, text_of(value)),
            TimeLineOp::LessThan(tl, value) => write!(f, "LessThan({}, {})", tl, text_of(value)),
            TimeLineOp::LessThanOrEqual(tl, value) => write!(f, "LessThanOrEqual({}, {})", tl, text_of(value)),
            TimeLineOp::And(tl1, tl2) => write!(f, "And({}, {})", tl1, tl2),
            TimeLineOp::Or(tl1, tl2) => write!(f, "Or({}, {})", tl1, tl2),
            TimeLineOp::Not(tl) => write!(f, "Not({})", tl),
            TimeLineOp::TlHasExisted(tl, event_predicate) => write!(f, "TlHasExisted({}, {})", tl, event_predicate),
            TimeLineOp::TlHasExistedWithin(tl, event_predicate, within_time) => write!(f, "TlHasExistedWithin({}, {}, {})", tl, event_predicate, within_time),
            TimeLineOp::TlLatestEventToState(tl, event_predicate) => write!(f, "TlLatestEventToState({}, {})", tl, event_predicate),
            TimeLineOp::TlDurationWhere(tl, event_predicate) => write!(f, "TlDurationWhere({}, {})", tl, event_predicate),
            TimeLineOp::TlDurationInCurState(tl, event_predicate) => write!(f, "TlDurationInCurState({}, {})", tl, event_predicate),
        }
    }
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
            TimeLineOp::TlHasExistedWithin(_, _, _) => true,
            TimeLineOp::TlLatestEventToState(_, _) => true,
            _ => false,
        }
    }

    fn evaluate(&self) -> TimeLine<GolemEventValue> {
        unimplemented!("evaluate not implemented")
    }

    fn tl_has_existed(self, event_predicate: EventPredicate<GolemEventValue>) -> TimeLineOp {
        TimeLineOp::TlHasExisted(Box::new(self), event_predicate)
    }

    fn tl_has_existed_within(self, event_predicate: EventPredicate<GolemEventValue>, within_time: u64) -> TimeLineOp {
        TimeLineOp::TlHasExistedWithin(Box::new(self), event_predicate, within_time)
    }

    fn tl_latest_event_to_state(self, event_predicate: EventPredicate<GolemEventValue>) -> TimeLineOp {
        TimeLineOp::TlLatestEventToState(Box::new(self), event_predicate)
    }

    fn tl_duration_where(self, event_predicate: EventPredicate<GolemEventValue>) -> TimeLineOp {
        TimeLineOp::TlDurationWhere(Box::new(self), event_predicate)
    }

    fn tl_duration_in_cur_state(self, event_predicate: EventPredicate<GolemEventValue>) -> TimeLineOp {
        TimeLineOp::TlDurationInCurState(Box::new(self), event_predicate)
    }
}

impl From<WitTimeLineOp> for TimeLineOp {
    fn from(value: WitTimeLineOp) -> Self {
        assert!(!value.nodes.is_empty());
        build_tree(&value.nodes[0], &value.nodes)

    }
}

fn build_tree(node: &TimelineNode, nodes: &[TimelineNode]) -> TimeLineOp {
    match node {
        TimelineNode::Primitive(primitive_timeline) => {
            let time_line = build_tree(&nodes[primitive_timeline.timeline as usize], nodes);
            let golem_event_value: GolemEventValue = primitive_timeline.value.clone();

            match primitive_timeline.op {
                TimelinePrimitiveOp::GreaterThan => TimeLineOp::GreaterThan(Box::new(time_line), golem_event_value),
                TimelinePrimitiveOp::GreaterThanEqual => TimeLineOp::GreaterThanOrEqual(Box::new(time_line), golem_event_value),
                TimelinePrimitiveOp::LessThan => TimeLineOp::LessThan(Box::new(time_line), golem_event_value),
                TimelinePrimitiveOp::LessThanEqual => TimeLineOp::LessThanOrEqual(Box::new(time_line), golem_event_value),
            }
        }
        TimelineNode::NotNode(node_index) => {
            let time_line = build_tree(&nodes[*node_index as usize], nodes);
            TimeLineOp::Not(Box::new(time_line))
        }
        TimelineNode::TlHasExisted(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());

            let filter = match filtered_timeline.filter {
                FilterOp::Equal => EventPredicate::Equals(event_column, event_value),
                FilterOp::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                FilterOp::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlHasExisted(Box::new(time_line), filter)
        }

        TimelineNode::TlHasExistedWithin(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.filtered.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.filtered.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.filtered.event_predicate.col_name.clone());
            let max_duration = filtered_timeline.time;

            let filter = match filtered_timeline.filtered.filter {
                FilterOp::Equal => EventPredicate::Equals(event_column, event_value),
                FilterOp::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                FilterOp::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlHasExistedWithin(Box::new(time_line), filter, max_duration)
        }
        TimelineNode::TlDurationWhere(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());

            let filter = match filtered_timeline.filter {
                FilterOp::Equal => EventPredicate::Equals(event_column, event_value),
                FilterOp::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                FilterOp::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlDurationWhere(Box::new(time_line), filter)
        }
        TimelineNode::TlDurationInCurState(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());

            let filter = match filtered_timeline.filter {
                FilterOp::Equal => EventPredicate::Equals(event_column, event_value),
                FilterOp::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                FilterOp::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlDurationInCurState(Box::new(time_line), filter)
        }
        TimelineNode::Leaf(_) => TimeLineOp::Leaf(),
    }
}