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
    event_processor_spec: TimeLineNodeWorkerInput,
    timeline_processor_spec: TimeLineNodeWorkerInput,
}

impl SimpleGolemTimelineDsl {
    pub fn new(
        name: String,
        event_processor_component_id: String,
        timeline_processor_component_id: String,
    ) -> Self {
        SimpleGolemTimelineDsl {
            event_processor_spec: TimeLineNodeWorkerInput {
                worker_id_prefix: TimeLineWorkerIdPrefix(name.clone()),
                component_id: event_processor_component_id,
            },
            timeline_processor_spec: TimeLineNodeWorkerInput {
                worker_id_prefix: TimeLineWorkerIdPrefix(name.clone()),
                component_id: timeline_processor_component_id,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum TimeLineBuilderOp {
    EqualTo(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>, GolemEventValue),
    GreaterThan(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>, GolemEventValue),
    GreaterThanOrEqual(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>, GolemEventValue),
    LessThan(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>, GolemEventValue),
    LessThanOrEqual(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>, GolemEventValue),
    And(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>, Box<TimeLineBuilderOp>),
    Or(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>, Box<TimeLineBuilderOp>),
    Not(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>),

    TlHasExisted(Option<TimeLineNodeWorkerInput>, GolemEventPredicate<GolemEventValue>),
    TlHasExistedWithin(Option<TimeLineNodeWorkerInput>, GolemEventPredicate<GolemEventValue>, u64),
    TlLatestEventToState(Option<TimeLineNodeWorkerInput>, EventColumnName),
    TlDurationWhere(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>),
    TlDurationInCurState(Option<TimeLineNodeWorkerInput>, Box<TimeLineBuilderOp>),
}

impl TimeLineBuilderOp {
    pub fn tl_has_existed(predicate: GolemEventPredicate<GolemEventValue>) -> TimeLineBuilderOp {
        TimeLineBuilderOp::TlHasExisted(None, predicate)
    }

    pub fn with_worker_details(
        &self,
        worker_prefix: String,
        event_processor_id: String,
        timeline_proecessor_id: String,
    ) -> TimeLineOp {
        let event_processor_worker_details = TimeLineNodeWorkerInput {
            worker_id_prefix: TimeLineWorkerIdPrefix(worker_prefix.clone()),
            component_id: event_processor_id,
        };

        let timeline_processor_worker_details = TimeLineNodeWorkerInput {
            worker_id_prefix: TimeLineWorkerIdPrefix(worker_prefix.clone()),
            component_id: timeline_proecessor_id,
        };

        fn go(
            op_builder: Box<TimeLineBuilderOp>,
            event_processor_worker_details: &TimeLineNodeWorkerInput,
            timeline_processor_worker_details: &TimeLineNodeWorkerInput,
        ) -> TimeLineOp {
            match *op_builder {
                TimeLineBuilderOp::EqualTo(None, op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::EqualTo(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineBuilderOp::EqualTo(Some(worker_details), op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::EqualTo(worker_details, Box::new(child_op), value)
                }
                TimeLineBuilderOp::GreaterThan(None, op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThan(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineBuilderOp::GreaterThan(Some(worker_details), op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThan(worker_details, Box::new(child_op), value)
                }
                TimeLineBuilderOp::GreaterThanOrEqual(None, op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThanOrEqual(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineBuilderOp::GreaterThanOrEqual(Some(worker_details), op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThanOrEqual(worker_details, Box::new(child_op), value)
                }
                TimeLineBuilderOp::LessThan(None, op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThan(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineBuilderOp::LessThan(Some(worker_details), op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThan(worker_details, Box::new(child_op), value)
                }
                TimeLineBuilderOp::LessThanOrEqual(None, op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThanOrEqual(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineBuilderOp::LessThanOrEqual(Some(worker_details), op, value) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThanOrEqual(worker_details, Box::new(child_op), value)
                }
                TimeLineBuilderOp::And(None, left, right) => {
                    let left_child_op =
                        go(left, event_processor_worker_details, timeline_processor_worker_details);
                    let right_child_op = go(
                        right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::And(
                        timeline_processor_worker_details.clone(),
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineBuilderOp::And(Some(worker_details), left, right) => {
                    let left_child_op =
                        go(left, event_processor_worker_details, timeline_processor_worker_details);
                    let right_child_op = go(
                        right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::And(
                        worker_details,
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineBuilderOp::Or(None, left, right) => {
                    let left_child_op =
                        go(left, event_processor_worker_details, timeline_processor_worker_details);
                    let right_child_op = go(
                        right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::Or(
                        timeline_processor_worker_details.clone(),
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineBuilderOp::Or(Some(worker_details), left, right) => {
                    let left_child_op =
                        go(left, event_processor_worker_details, timeline_processor_worker_details);
                    let right_child_op = go(
                        right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::Or(
                        worker_details,
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineBuilderOp::Not(None, op) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::Not(timeline_processor_worker_details.clone(), Box::new(child_op))
                }
                TimeLineBuilderOp::Not(Some(worker_details), op) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::Not(worker_details, Box::new(child_op))
                }
                TimeLineBuilderOp::TlHasExisted(None, predicate) => {
                    TimeLineOp::TlHasExisted(event_processor_worker_details.clone(), predicate)
                }
                TimeLineBuilderOp::TlHasExisted(Some(worker_details), predicate) => {
                    TimeLineOp::TlHasExisted(worker_details, predicate)
                }
                TimeLineBuilderOp::TlHasExistedWithin(None, predicate, duration) => {
                    TimeLineOp::TlHasExistedWithin(
                        event_processor_worker_details.clone(),
                        predicate,
                        duration,
                    )
                }
                TimeLineBuilderOp::TlHasExistedWithin(
                    Some(worker_details),
                    predicate,
                    duration,
                ) => TimeLineOp::TlHasExistedWithin(worker_details, predicate, duration),
                TimeLineBuilderOp::TlLatestEventToState(None, column) => {
                    TimeLineOp::TlLatestEventToState(event_processor_worker_details.clone(), column)
                }
                TimeLineBuilderOp::TlLatestEventToState(Some(worker_details), column) => {
                    TimeLineOp::TlLatestEventToState(worker_details, column)
                }
                TimeLineBuilderOp::TlDurationWhere(None, op) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationWhere(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                    )
                }
                TimeLineBuilderOp::TlDurationWhere(Some(worker_details), op) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationWhere(worker_details, Box::new(child_op))
                }
                TimeLineBuilderOp::TlDurationInCurState(None, op) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationInCurState(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                    )
                }
                TimeLineBuilderOp::TlDurationInCurState(Some(worker_details), op) => {
                    let child_op =
                        go(op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationInCurState(worker_details, Box::new(child_op))
                }
            }
        }

        go(
            Box::new(self.clone()),
            &event_processor_worker_details,
            &timeline_processor_worker_details,
        )
    }
}

pub trait TimeLineOpBuilder {
    fn tl_has_existed(&self, predicate: GolemEventPredicate<GolemEventValue>) -> TimeLineOp;
    fn tl_has_existed_within(
        &self,
        predicate: GolemEventPredicate<GolemEventValue>,
        d: u64,
    ) -> TimeLineOp;
    fn tl_latest_event_to_state(&self, event_col_name: EventColumnName) -> TimeLineOp;
    fn tl_duration_where(&self, op: TimeLineOp) -> TimeLineOp;
    fn tl_duration_in_cur_state(&self, op: TimeLineOp) -> TimeLineOp;

    fn tl_equal_to(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn tl_greater_than(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn tl_greater_than_or_equal(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn tl_less_than(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn tl_less_than_or_equal(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp;
    fn tl_and(&self, left: TimeLineOp, right: TimeLineOp) -> TimeLineOp;
    fn tl_or(&self, left: TimeLineOp, right: TimeLineOp) -> TimeLineOp;
    fn tl_not(&self, op: TimeLineOp) -> TimeLineOp;
}

impl TimeLineOpBuilder for SimpleGolemTimelineDsl {
    fn tl_has_existed(&self, predicate: GolemEventPredicate<GolemEventValue>) -> TimeLineOp {
        TimeLineOp::TlHasExisted(self.event_processor_spec.clone(), predicate)
    }

    fn tl_has_existed_within(
        &self,
        predicate: GolemEventPredicate<GolemEventValue>,
        d: u64,
    ) -> TimeLineOp {
        TimeLineOp::TlHasExistedWithin(self.event_processor_spec.clone(), predicate, d)
    }

    fn tl_latest_event_to_state(&self, event_col_name: EventColumnName) -> TimeLineOp {
        TimeLineOp::TlLatestEventToState(self.event_processor_spec.clone(), event_col_name)
    }

    fn tl_duration_where(&self, op: TimeLineOp) -> TimeLineOp {
        TimeLineOp::TlDurationWhere(self.event_processor_spec.clone(), Box::new(op))
    }

    fn tl_duration_in_cur_state(&self, op: TimeLineOp) -> TimeLineOp {
        TimeLineOp::TlDurationInCurState(self.event_processor_spec.clone(), Box::new(op))
    }

    fn tl_equal_to(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::EqualTo(self.timeline_processor_spec.clone(), Box::new(op), value)
    }

    fn tl_greater_than(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::GreaterThan(self.timeline_processor_spec.clone(), Box::new(op), value)
    }

    fn tl_greater_than_or_equal(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::GreaterThanOrEqual(self.timeline_processor_spec.clone(), Box::new(op), value)
    }

    fn tl_less_than(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::LessThan(self.timeline_processor_spec.clone(), Box::new(op), value)
    }

    fn tl_less_than_or_equal(&self, op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
        TimeLineOp::LessThanOrEqual(self.timeline_processor_spec.clone(), Box::new(op), value)
    }

    fn tl_and(&self, left: TimeLineOp, right: TimeLineOp) -> TimeLineOp {
        TimeLineOp::And(self.timeline_processor_spec.clone(), Box::new(right), Box::new(left))
    }

    fn tl_or(&self, left: TimeLineOp, right: TimeLineOp) -> TimeLineOp {
        TimeLineOp::Or(self.timeline_processor_spec.clone(), Box::new(right), Box::new(left))
    }

    fn tl_not(&self, op: TimeLineOp) -> TimeLineOp {
        TimeLineOp::Not(self.timeline_processor_spec.clone(), Box::new(op))
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
