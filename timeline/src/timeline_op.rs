use std::fmt::Display;

use crate::event_predicate::{EventColumnName, GolemEventPredicate};
use crate::golem_event::GolemEventValue;
use crate::timeline_node_worker::TimeLineNodeWorkerInput;
use crate::timeline_node_worker::TimeLineWorkerIdPrefix;

#[derive(Clone, Debug)]
pub enum TimeLineOp {
    // FIXME TimeLineNodeWorkerInput can maybe be reference using a lifetime???
    // Pretty much represents the event-timeline (not state dynamics) - source (through workerid) and collection
    EqualTo(TimeLineNodeWorkerInput, Box<TimeLineOp>, GolemEventValue),
    GreaterThan(TimeLineNodeWorkerInput, Box<TimeLineOp>, GolemEventValue),
    GreaterThanOrEqual(TimeLineNodeWorkerInput, Box<TimeLineOp>, GolemEventValue),
    LessThan(TimeLineNodeWorkerInput, Box<TimeLineOp>, GolemEventValue),
    LessThanOrEqual(TimeLineNodeWorkerInput, Box<TimeLineOp>, GolemEventValue),
    And(TimeLineNodeWorkerInput, Box<TimeLineOp>, Box<TimeLineOp>),
    Or(TimeLineNodeWorkerInput, Box<TimeLineOp>, Box<TimeLineOp>),
    Not(TimeLineNodeWorkerInput, Box<TimeLineOp>),

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
    TlHasExisted(TimeLineNodeWorkerInput, GolemEventPredicate<GolemEventValue>),
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
    TlHasExistedWithin(TimeLineNodeWorkerInput, GolemEventPredicate<GolemEventValue>, u64),
    // This is more or less making number of events to a very simple
    // timeline. Obviously this is corresponding to the events that are state dynamic in nature
    // t1 - t10 : CDN2
    // t10 - t11 : CDN1
    // t11- t12: CDN1
    // Output
    // t1-t10: CDN2
    // t10-t12: CDN1
    TlLatestEventToState(TimeLineNodeWorkerInput, EventColumnName),
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
    TlDurationWhere(TimeLineNodeWorkerInput, Box<TimeLineOp>),

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
    TlDurationInCurState(TimeLineNodeWorkerInput, Box<TimeLineOp>),
}

impl TimeLineOp {
    pub fn timeline_nodes(&self) -> Vec<TimeLineNodeWorkerInput> {
        fn servers_of(time_line_op: &TimeLineOp) -> Vec<TimeLineNodeWorkerInput> {
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

#[derive(Debug)]
pub struct SimpleGolemTimelineDsl {
    leaf_node_worker: TimeLineNodeWorkerInput,
    derived_node_worker: TimeLineNodeWorkerInput,
}

impl SimpleGolemTimelineDsl {
    pub fn new(
        metric_name: String,
        leaf_node_component_id: String,
        derived_node_component_id: String,
    ) -> Self {
        SimpleGolemTimelineDsl {
            leaf_node_worker: TimeLineNodeWorkerInput {
                worker_id_prefix: TimeLineWorkerIdPrefix(metric_name.clone()),
                component_id: leaf_node_component_id,
            },
            derived_node_worker: TimeLineNodeWorkerInput {
                worker_id_prefix: TimeLineWorkerIdPrefix(metric_name.clone()),
                component_id: derived_node_component_id,
            },
        }
    }
}

pub trait TimeLineOpBuilder {
    fn has_existed(&self, predicate: GolemEventPredicate<GolemEventValue>) -> TimeLineOp;
    fn has_existed_within(
        &self,
        predicate: GolemEventPredicate<GolemEventValue>,
        d: u64,
    ) -> TimeLineOp;
    fn latest_event_to_state(&self, event_col_name: EventColumnName) -> TimeLineOp;
    fn duration_where(&self, op: TimeLineOp) -> TimeLineOp;
    fn duration_in_cur_state(&self, op: TimeLineOp) -> TimeLineOp;

    fn equal_to(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn greater_than(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn greater_than_or_equal(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn less_than(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn less_than_or_equal(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn and(&self, left: TimeLineOp, right: TimeLineOp) -> TimeLineOp;
    fn or(&self, left: TimeLineOp, right: TimeLineOp) -> TimeLineOp;
    fn not(&self, op: TimeLineOp) -> TimeLineOp;
}

impl TimeLineOpBuilder for SimpleGolemTimelineDsl {
    fn has_existed(&self, predicate: GolemEventPredicate<GolemEventValue>) -> TimeLineOp {
        TimeLineOp::TlHasExisted(self.leaf_node_worker.clone(), predicate)
    }

    fn has_existed_within(
        &self,
        predicate: GolemEventPredicate<GolemEventValue>,
        d: u64,
    ) -> TimeLineOp {
        TimeLineOp::TlHasExistedWithin(self.leaf_node_worker.clone(), predicate, d)
    }

    fn latest_event_to_state(&self, event_col_name: EventColumnName) -> TimeLineOp {
        TimeLineOp::TlLatestEventToState(self.leaf_node_worker.clone(), event_col_name)
    }

    fn duration_where(&self, op: TimeLineOp) -> TimeLineOp {
        TimeLineOp::TlDurationWhere(self.leaf_node_worker.clone(), Box::new(op))
    }

    fn duration_in_cur_state(&self, op: TimeLineOp) -> TimeLineOp {
        TimeLineOp::TlDurationInCurState(self.leaf_node_worker.clone(), Box::new(op))
    }

    fn equal_to(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::EqualTo(self.derived_node_worker.clone(), Box::new(op), value)
    }

    fn greater_than(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::GreaterThan(self.derived_node_worker.clone(), Box::new(op), value)
    }

    fn greater_than_or_equal(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::GreaterThanOrEqual(self.derived_node_worker.clone(), Box::new(op), value)
    }

    fn less_than(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::LessThan(self.derived_node_worker.clone(), Box::new(op), value)
    }

    fn less_than_or_equal(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::LessThanOrEqual(self.derived_node_worker.clone(), Box::new(op), value)
    }

    fn and(&self, left: TimeLineOp, right: TimeLineOp) -> TimeLineOp {
        TimeLineOp::And(self.derived_node_worker.clone(), Box::new(right), Box::new(left))
    }

    fn or(&self, left: TimeLineOp, right: TimeLineOp) -> TimeLineOp {
        TimeLineOp::Or(self.derived_node_worker.clone(), Box::new(right), Box::new(left))
    }

    fn not(&self, op: TimeLineOp) -> TimeLineOp {
        TimeLineOp::Not(self.derived_node_worker.clone(), Box::new(op))
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
            TimeLineOp::EqualTo(server, time_line, golem_event_value) => {
                write!(f, "EqualTo({}, {}, {})", server, time_line, text_of(golem_event_value))
            }
            TimeLineOp::GreaterThan(server, time_line, golem_event_value) => {
                write!(f, "GreaterThan({}, {}, {})", server, time_line, text_of(golem_event_value))
            }
            TimeLineOp::GreaterThanOrEqual(server, time_line, golem_event_value) => write!(
                f,
                "GreaterThanOrEqual({}, {}, {})",
                server,
                time_line,
                text_of(golem_event_value)
            ),
            TimeLineOp::LessThan(server, time_line, golem_event_value) => {
                write!(f, "LessThan({}, {}, {})", server, time_line, text_of(golem_event_value))
            }
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
            TimeLineOp::TlHasExistedWithin(server, event_predicate, within_time) => {
                write!(f, "TlHasExistedWithin({}, {}, {})", server, event_predicate, within_time)
            }
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
