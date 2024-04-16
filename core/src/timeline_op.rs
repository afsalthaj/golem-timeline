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

#[derive(Clone, Debug)]
pub struct Server {
    pub worker_id: String,
    pub template_id: String
}

impl From<crate::bindings::exports::timeline::core::api::Server> for Server {
    fn from(value: crate::bindings::exports::timeline::core::api::Server) -> Self {
        Server {
            worker_id: value.worker_id,
            template_id: value.template_id,
        }
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.template_id, self.worker_id)
    }
}


pub enum TimeLineOp {
    // Pretty much represents the event-timeline (not state dynamics) - source (through workerid) and collection
    Leaf(Server), // A leaf node results in a component that exposes a function accepting event and storing it in a configurable buffer
    EqualTo(Server, Box<TimeLineOp>, GolemEventValue),
    GreaterThan(Server, Box<TimeLineOp>, GolemEventValue),
    GreaterThanOrEqual(Server, Box<TimeLineOp>, GolemEventValue),
    LessThan(Server, Box<TimeLineOp>, GolemEventValue),
    LessThanOrEqual(Server, Box<TimeLineOp>, GolemEventValue),
    And(Server, Box<TimeLineOp>, Box<TimeLineOp>),
    Or(Server, Box<TimeLineOp>, Box<TimeLineOp>),
    Not(Server, Box<TimeLineOp>),

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
    TlHasExisted(Server, Box<TimeLineOp>, EventPredicate<GolemEventValue>),
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
    TlHasExistedWithin(Server, Box<TimeLineOp>, EventPredicate<GolemEventValue>, u64),
    // This is more or less making number of events to a very simple
    // timeline. Obviously this is corresponding to the events that are state dynamic in nature
    // t1 - t10 : CDN2
    // t10 - t11 : CDN1
    // t11- t12: CDN1
    // Output
    // t1-t10: CDN2
    // t10-t12: CDN1
    TlLatestEventToState(Server, Box<TimeLineOp>, EventPredicate<GolemEventValue>),
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
    TlDurationWhere(Server, Box<TimeLineOp>, EventPredicate<GolemEventValue>),

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
    TlDurationInCurState(Server, Box<TimeLineOp>, EventPredicate<GolemEventValue>),
}

impl TimeLineOp {
    fn servers(&self) -> Vec<Server> {
        fn servers_of(time_line_op: &TimeLineOp) -> Vec<Server> {
            match time_line_op {
                TimeLineOp::Leaf(server) => vec![server.clone()],
                TimeLineOp::EqualTo(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::GreaterThan(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::GreaterThanOrEqual(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::LessThan(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::LessThanOrEqual(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::And(server, time_line1, time_line2) => {
                    let mut servers = servers_of(time_line1);
                    servers.extend(servers_of(time_line2));
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::Or(server, time_line1, time_line2) => {
                    let mut servers = servers_of(time_line1);
                    servers.extend(servers_of(time_line2));
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::Not(server, time_line) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::TlHasExisted(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::TlHasExistedWithin(server, time_line, _, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::TlLatestEventToState(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::TlDurationWhere(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::TlDurationInCurState(server, timeline, _) => {
                    let mut servers = servers_of(timeline);
                    servers.push(server.clone());
                    servers
                }
            }
        }

        servers_of(self)
    }
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
            TimeLineOp::EqualTo(server, time_line, golem_event_value) => write!(f, "EqualTo({}, {}, {})", server, time_line, text_of(golem_event_value)),
            TimeLineOp::GreaterThan(server, time_line, golem_event_value) => write!(f, "GreaterThan({}, {}, {})", server, time_line, text_of(golem_event_value)),
            TimeLineOp::GreaterThanOrEqual(server, time_line, golem_event_value) => write!(f, "GreaterThanOrEqual({}, {}, {})", server, time_line, text_of(golem_event_value)),
            TimeLineOp::LessThan(server, time_line, golem_event_value) => write!(f, "LessThan({}, {}, {})", server, time_line, text_of(golem_event_value)),
            TimeLineOp::LessThanOrEqual(server, time_line, golem_event_value) => write!(f, "LessThanOrEqual({}, {}, {})", server, time_line, text_of(golem_event_value)),
            TimeLineOp::And(server, time_line1, time_line2) => write!(f, "And({}, {}, {})", server, time_line1, time_line2),
            TimeLineOp::Or(server, time_line1, time_line2) => write!(f, "Or({}, {}, {})", server, time_line1, time_line2),
            TimeLineOp::Not(server, time_line) => write!(f, "Not({}, {})", server, time_line),
            TimeLineOp::TlHasExisted(server, time_line, event_predicate) => write!(f, "TlHasExisted({}, {}, {})", server, time_line, event_predicate),
            TimeLineOp::TlHasExistedWithin(server, time_line, event_predicate, within_time) => write!(f, "TlHasExistedWithin({}, {}, {}, {})", server, time_line, event_predicate, within_time),
            TimeLineOp::TlLatestEventToState(server, time_line, event_predicate) => write!(f, "TlLatestEventToState({}, {}, {})", server, time_line, event_predicate),
            TimeLineOp::TlDurationWhere(server, time_line, event_predicate) => write!(f, "TlDurationWhere({}, {}, {})", server, time_line, event_predicate),
            TimeLineOp::TlDurationInCurState(server, time_line, event_predicate) => write!(f, "TlDurationInCurState({}, {}, {})", server, time_line, event_predicate),
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
            let server: Server = timeline_classic.server.clone().into();

            match timeline_classic.op {
                TimelineClassicComparator::GreaterThan => TimeLineOp::GreaterThan(server, Box::new(time_line), golem_event_value),
                TimelineClassicComparator::GreaterThanEqual => TimeLineOp::GreaterThanOrEqual(server, Box::new(time_line), golem_event_value),
                TimelineClassicComparator::LessThan => TimeLineOp::LessThan(server, Box::new(time_line), golem_event_value),
                TimelineClassicComparator::LessThanEqual => TimeLineOp::LessThanOrEqual(server, Box::new(time_line), golem_event_value),
            }
        }
        TimelineNode::NotNode(timeline) => {
            let time_line = build_tree(&nodes[timeline.timeline as usize], nodes);
            let server: Server = timeline.server.clone().into();

            TimeLineOp::Not(server, Box::new(time_line))
        }
        TimelineNode::TlHasExisted(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());
            let server: Server = filtered_timeline.server.clone().into();


            let filter = match filtered_timeline.filter {
                TimelineSpecificComparator::Equal => EventPredicate::Equals(event_column, event_value),
                TimelineSpecificComparator::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                TimelineSpecificComparator::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlHasExisted(server, Box::new(time_line), filter)
        }

        TimelineNode::TlHasExistedWithin(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.filtered.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.filtered.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.filtered.event_predicate.col_name.clone());
            let max_duration = filtered_timeline.time;
            let server: Server = filtered_timeline.filtered.server.clone().into();

            let filter = match filtered_timeline.filtered.filter {
                TimelineSpecificComparator::Equal => EventPredicate::Equals(event_column, event_value),
                TimelineSpecificComparator::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                TimelineSpecificComparator::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlHasExistedWithin(server, Box::new(time_line), filter, max_duration)
        }
        TimelineNode::TlDurationWhere(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());
            let server: Server = filtered_timeline.server.clone().into();

            let filter = match filtered_timeline.filter {
                TimelineSpecificComparator::Equal => EventPredicate::Equals(event_column, event_value),
                TimelineSpecificComparator::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                TimelineSpecificComparator::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlDurationWhere(server, Box::new(time_line), filter)
        }
        TimelineNode::TlDurationInCurState(filtered_timeline) => {
            let time_line = build_tree(&nodes[filtered_timeline.node as usize], nodes);
            let event_value: EventValue<GolemEventValue> =  filtered_timeline.event_predicate.value.clone().into();
            let event_column = EventColumn(filtered_timeline.event_predicate.col_name.clone());
            let server: Server = filtered_timeline.server.clone().into();

            let filter = match filtered_timeline.filter {
                TimelineSpecificComparator::Equal => EventPredicate::Equals(event_column, event_value),
                TimelineSpecificComparator::GreaterThan => EventPredicate::GreaterThan(event_column, event_value),
                TimelineSpecificComparator::LessThan => EventPredicate::LessThan(event_column, event_value),
            };

            TimeLineOp::TlDurationInCurState(server, Box::new(time_line), filter)
        }
        TimelineNode::Leaf(server) => TimeLineOp::Leaf(Server{
            worker_id: server.worker_id.clone(),
            template_id: server.template_id.clone()
        }),
    }
}