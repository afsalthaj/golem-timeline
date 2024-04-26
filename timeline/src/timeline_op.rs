use std::fmt::Display;

use crate::event_predicate::{EventColumnName, GolemEventPredicate};
use crate::golem_event::GolemEventValue;
use crate::timeline_node_worker::TimeLineNodeWorker;

#[derive(Clone, Debug)]
pub enum TimeLineOp {
    // Pretty much represents the event-timeline (not state dynamics) - source (through workerid) and collection
    EqualTo(TimeLineNodeWorker, Box<TimeLineOp>, GolemEventValue),
    GreaterThan(TimeLineNodeWorker, Box<TimeLineOp>, GolemEventValue),
    GreaterThanOrEqual(TimeLineNodeWorker, Box<TimeLineOp>, GolemEventValue),
    LessThan(TimeLineNodeWorker, Box<TimeLineOp>, GolemEventValue),
    LessThanOrEqual(TimeLineNodeWorker, Box<TimeLineOp>, GolemEventValue),
    And(TimeLineNodeWorker, Box<TimeLineOp>, Box<TimeLineOp>),
    Or(TimeLineNodeWorker, Box<TimeLineOp>, Box<TimeLineOp>),
    Not(TimeLineNodeWorker, Box<TimeLineOp>),

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
    TlHasExisted(TimeLineNodeWorker, GolemEventPredicate<GolemEventValue>),
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
    TlHasExistedWithin(
        TimeLineNodeWorker,
        GolemEventPredicate<GolemEventValue>,
        u64,
    ),
    // This is more or less making number of events to a very simple
    // timeline. Obviously this is corresponding to the events that are state dynamic in nature
    // t1 - t10 : CDN2
    // t10 - t11 : CDN1
    // t11- t12: CDN1
    // Output
    // t1-t10: CDN2
    // t10-t12: CDN1
    TlLatestEventToState(TimeLineNodeWorker, EventColumnName),
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
    TlDurationWhere(TimeLineNodeWorker, Box<TimeLineOp>),

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
    TlDurationInCurState(TimeLineNodeWorker, Box<TimeLineOp>),
}

impl TimeLineOp {
    pub fn timeline_nodes(&self) -> Vec<TimeLineNodeWorker> {
        fn servers_of(time_line_op: &TimeLineOp) -> Vec<TimeLineNodeWorker> {
            match time_line_op {
                TimeLineOp::TlHasExisted(server, _event_predicate) => vec![server.clone()],

                TimeLineOp::TlLatestEventToState(server, _) => vec![server.clone()],
                TimeLineOp::TlHasExistedWithin(server, _, _) => vec![server.clone()],

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
                TimeLineOp::TlDurationWhere(server, time_line) => {
                    let mut servers = servers_of(time_line);
                    servers.push(server.clone());
                    servers
                }
                TimeLineOp::TlDurationInCurState(server, timeline) => {
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
            TimeLineOp::EqualTo(server, time_line, golem_event_value) => write!(
                f,
                "EqualTo({}, {}, {})",
                server,
                time_line,
                text_of(golem_event_value)
            ),
            TimeLineOp::GreaterThan(server, time_line, golem_event_value) => write!(
                f,
                "GreaterThan({}, {}, {})",
                server,
                time_line,
                text_of(golem_event_value)
            ),
            TimeLineOp::GreaterThanOrEqual(server, time_line, golem_event_value) => write!(
                f,
                "GreaterThanOrEqual({}, {}, {})",
                server,
                time_line,
                text_of(golem_event_value)
            ),
            TimeLineOp::LessThan(server, time_line, golem_event_value) => write!(
                f,
                "LessThan({}, {}, {})",
                server,
                time_line,
                text_of(golem_event_value)
            ),
            TimeLineOp::LessThanOrEqual(server, time_line, golem_event_value) => write!(
                f,
                "LessThanOrEqual({}, {}, {})",
                server,
                time_line,
                text_of(golem_event_value)
            ),
            TimeLineOp::And(server, time_line1, time_line2) => {
                write!(f, "And({}, {}, {})", server, time_line1, time_line2)
            }
            TimeLineOp::Or(server, time_line1, time_line2) => {
                write!(f, "Or({}, {}, {})", server, time_line1, time_line2)
            }
            TimeLineOp::Not(server, time_line) => write!(f, "Not({}, {})", server, time_line),
            TimeLineOp::TlHasExisted(server, event_predicate) => {
                write!(f, "TlHasExisted({}, {})", server, event_predicate)
            }
            TimeLineOp::TlHasExistedWithin(server, event_predicate, within_time) => write!(
                f,
                "TlHasExistedWithin({}, {}, {})",
                server, event_predicate, within_time
            ),
            TimeLineOp::TlLatestEventToState(server, event_column) => {
                write!(f, "TlLatestEventToState({}, {})", server, event_column)
            }
            TimeLineOp::TlDurationWhere(server, time_line) => {
                write!(f, "TlDurationWhere({}, {})", server, time_line)
            }
            TimeLineOp::TlDurationInCurState(server, time_line) => {
                write!(f, "TlDurationInCurState({}, {})", server, time_line)
            }
        }
    }
}
