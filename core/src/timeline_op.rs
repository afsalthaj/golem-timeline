use std::fmt::Display;
use crate::bindings::timeline::raw_events::api::EventValue as GolemEventValue;
//use crate::bindings::exports::golem::timeline::api::{/*FilterOp, TimelineNode, TimelineOp as WitTimeLineOp, TimeLineClassicComparator};
use crate::event_predicate::{EventColumn, EventPredicate, EventValue};
use crate::timeline::TimeLine;
use crate::bindings::exports::timeline::core::api::{TimelineSpecificComparator, TimelineNode, TimelineOp as WitTimeLineOp, TimelineClassicComparator};


// In paper, it is referred as object DAG
// TimeLineOp will produce numerical or state-dynamic timeline of a `Value` which can be (currently) string, int etc
// This implies your raw events needs to produce events, or state that can be coalesced to `Value`

// Id annotation for each node
// timeline-op Id + integer Id

pub struct WorkerId(pub String);

pub struct Server {
    pub worker_id: String,
    pub template_id: String
}

pub enum TimeLineOp {
    // Pretty much represents the event-timeline (not state dynamics) - source (through workerid) and collection
    Leaf(Server), // A leaf node results in a component that exposes a function accepting event and storing it in a configurable buffer
    EqualTo(WorkerId, Box<TimeLineOp>, GolemEventValue),
    GreaterThan(WorkerId, Box<TimeLineOp>, GolemEventValue),
    GreaterThanOrEqual(WorkerId, Box<TimeLineOp>, GolemEventValue),
    LessThan(WorkerId, Box<TimeLineOp>, GolemEventValue),
    LessThanOrEqual(WorkerId, Box<TimeLineOp>, GolemEventValue),
    And(WorkerId, Box<TimeLineOp>, Box<TimeLineOp>),
    Or(WorkerId, Box<TimeLineOp>, Box<TimeLineOp>),
    Not(WorkerId, Box<TimeLineOp>),

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
    TlHasExisted(WorkerId, Box<TimeLineOp>, EventPredicate<GolemEventValue>),
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
    TlHasExistedWithin(WorkerId, Box<TimeLineOp>, EventPredicate<GolemEventValue>, u64),
    // This is more or less making number of events to a very simple
    // timeline. Obviously this is corresponding to the events that are state dynamic in nature
    // t1 - t10 : CDN2
    // t10 - t11 : CDN1
    // t11- t12: CDN1
    // Output
    // t1-t10: CDN2
    // t10-t12: CDN1
    TlLatestEventToState(WorkerId, Box<TimeLineOp>, EventPredicate<GolemEventValue>),
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
    TlDurationWhere(WorkerId, Box<TimeLineOp>, EventPredicate<GolemEventValue>),

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
    TlDurationInCurState(WorkerId, Box<TimeLineOp>, EventPredicate<GolemEventValue>),
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
            TimeLineOp::Leaf(server) => write!(f, "Leaf({}.{})", server.template_id, server.worker_id),
            TimeLineOp::EqualTo(worker_id, time_line, golem_event_value) => write!(f, "EqualTo({}, {}, {})", worker_id.0, time_line, text_of(golem_event_value)),
            TimeLineOp::GreaterThan(worker_id, time_line, golem_event_value) => write!(f, "GreaterThan({}, {}, {})", worker_id.0, time_line, text_of(golem_event_value)),
            TimeLineOp::GreaterThanOrEqual(worker_id, time_line, golem_event_value) => write!(f, "GreaterThanOrEqual({}, {}, {})", worker_id.0, time_line, text_of(golem_event_value)),
            TimeLineOp::LessThan(worker_id, time_line, golem_event_value) => write!(f, "LessThan({}, {}, {})", worker_id.0, time_line, text_of(golem_event_value)),
            TimeLineOp::LessThanOrEqual(worker_id, time_line, golem_event_value) => write!(f, "LessThanOrEqual({}, {}, {})", worker_id.0, time_line, text_of(golem_event_value)),
            TimeLineOp::And(worker_id, time_line1, time_line2) => write!(f, "And({}, {}, {})", worker_id.0, time_line1, time_line2),
            TimeLineOp::Or(worker_id, time_line1, time_line2) => write!(f, "Or({}, {}, {})", worker_id.0, time_line1, time_line2),
            TimeLineOp::Not(worker_id, time_line) => write!(f, "Not({}, {})", worker_id.0, time_line),
            TimeLineOp::TlHasExisted(worker_id, time_line, event_predicate) => write!(f, "TlHasExisted({}, {}, {})", worker_id.0, time_line, event_predicate),
            TimeLineOp::TlHasExistedWithin(worker_id, time_line, event_predicate, within_time) => write!(f, "TlHasExistedWithin({}, {}, {}, {})", worker_id.0, time_line, event_predicate, within_time),
            TimeLineOp::TlLatestEventToState(worker_id, time_line, event_predicate) => write!(f, "TlLatestEventToState({}, {}, {})", worker_id.0, time_line, event_predicate),
            TimeLineOp::TlDurationWhere(worker_id, time_line, event_predicate) => write!(f, "TlDurationWhere({}, {}, {})", worker_id.0, time_line, event_predicate),
            TimeLineOp::TlDurationInCurState(worker_id, time_line, event_predicate) => write!(f, "TlDurationInCurState({}, {}, {})", worker_id.0, time_line, event_predicate),
        }
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
        TimelineNode::Primitive(timeline_classic) => {
            let time_line = build_tree(&nodes[timeline_classic.timeline as usize], nodes);
            let golem_event_value: GolemEventValue = timeline_classic.value.clone();
            let worker_id = WorkerId(timeline_classic.server.name.clone());

            match timeline_classic.op {
                TimelineClassicComparator::GreaterThan => TimeLineOp::GreaterThan(worker_id, Box::new(time_line), golem_event_value),
                TimelineClassicComparator::GreaterThanEqual => TimeLineOp::GreaterThanOrEqual(worker_id, Box::new(time_line), golem_event_value),
                TimelineClassicComparator::LessThan => TimeLineOp::LessThan(worker_id, Box::new(time_line), golem_event_value),
                TimelineClassicComparator::LessThanEqual => TimeLineOp::LessThanOrEqual(worker_id, Box::new(time_line), golem_event_value),
            }
        }
        TimelineNode::NotNode(timeline) => {
            let time_line = build_tree(&nodes[timeline.timeline as usize], nodes);
            let worker_id = WorkerId(timeline.server.name.clone());

            TimeLineOp::Not(worker_id, Box::new(time_line))
        }
        TimelineNode::TlHasExisted(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());
            let worker_id = WorkerId(filtered_timeline.server.name.clone());


            let filter = match filtered_timeline.filter {
                TimelineSpecificComparator::Equal => EventPredicate::Equals(event_column, event_value),
                TimelineSpecificComparator::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                TimelineSpecificComparator::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlHasExisted(worker_id, Box::new(time_line), filter)
        }

        TimelineNode::TlHasExistedWithin(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.filtered.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.filtered.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.filtered.event_predicate.col_name.clone());
            let max_duration = filtered_timeline.time;
            let worker_id = WorkerId(filtered_timeline.filtered.server.name.clone());

            let filter = match filtered_timeline.filtered.filter {
                TimelineSpecificComparator::Equal => EventPredicate::Equals(event_column, event_value),
                TimelineSpecificComparator::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                TimelineSpecificComparator::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlHasExistedWithin(worker_id, Box::new(time_line), filter, max_duration)
        }
        TimelineNode::TlDurationWhere(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());
            let worker_id = WorkerId(filtered_timeline.server.name.clone());

            let filter = match filtered_timeline.filter {
                TimelineSpecificComparator::Equal => EventPredicate::Equals(event_column, event_value),
                TimelineSpecificComparator::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                TimelineSpecificComparator::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlDurationWhere(worker_id, Box::new(time_line), filter)
        }
        TimelineNode::TlDurationInCurState(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());
            let worker_id = WorkerId(filtered_timeline.server.name.clone());

            let filter = match filtered_timeline.filter {
                TimelineSpecificComparator::Equal => EventPredicate::Equals(event_column, event_value),
                TimelineSpecificComparator::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                TimelineSpecificComparator::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlDurationInCurState(worker_id, Box::new(time_line), filter)
        }
        TimelineNode::Leaf(server) => TimeLineOp::Leaf(Server{
            worker_id: server.worker_id.clone(),
            template_id: server.template_id.clone()
        }),
    }
}