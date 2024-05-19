use std::fmt::Display;

use crate::event_predicate::{EventColumnName, GolemEventPredicate};
use crate::golem_event::GolemEventValue;
use crate::timeline_node_worker::TimeLineNodeWorkerInput;
use crate::timeline_node_worker::TimeLineWorkerIdPrefix;
use std::convert::From;

#[derive(Clone, Debug)]
pub enum TimeLineOp {
    // FIXME TimeLineNodeWorkerInput can maybe be reference using a lifetime???
    // Pretty much represents the event-timeline (not state dynamics) - source (through workerid) and collection
    EqualTo(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, GolemEventValue),
    GreaterThan(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, GolemEventValue),
    GreaterThanOrEqual(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, GolemEventValue),
    LessThan(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, GolemEventValue),
    LessThanOrEqual(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, GolemEventValue),
    And(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, Box<TimeLineOp>),
    Or(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, Box<TimeLineOp>),
    Not(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>),

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
    TlHasExisted(Option<TimeLineNodeWorkerInput>, GolemEventPredicate<GolemEventValue>),
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
    TlHasExistedWithin(Option<TimeLineNodeWorkerInput>, GolemEventPredicate<GolemEventValue>, u64),
    // This is more or less making number of events to a very simple
    // timeline. Obviously this is corresponding to the events that are state dynamic in nature
    // t1 - t10 : CDN2
    // t10 - t11 : CDN1
    // t11- t12: CDN1
    // Output
    // t1-t10: CDN2
    // t10-t12: CDN1
    TlLatestEventToState(Option<TimeLineNodeWorkerInput>, EventColumnName),
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
    TlDurationWhere(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>),

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
    TlDurationInCurState(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>),
}

impl TimeLineOp {
    pub fn timeline_nodes(&self) -> Vec<TimeLineNodeWorkerInput> {
        fn to_vec(details: &Option<TimeLineNodeWorkerInput>) -> Vec<TimeLineNodeWorkerInput> {
            details.clone().map_or(vec![], |details| vec![details])
        }
        fn servers_of(time_line_op: &TimeLineOp) -> Vec<TimeLineNodeWorkerInput> {
            match time_line_op {
                TimeLineOp::TlHasExisted(server, _event_predicate) => to_vec(server),

                TimeLineOp::TlLatestEventToState(server, _) => to_vec(server),
                TimeLineOp::TlHasExistedWithin(server, _, _) => to_vec(server),

                TimeLineOp::EqualTo(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::GreaterThan(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::GreaterThanOrEqual(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::LessThan(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::LessThanOrEqual(server, time_line, _) => {
                    let mut servers = servers_of(time_line);
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::And(server, time_line1, time_line2) => {
                    let mut servers = servers_of(time_line1);
                    servers.extend(servers_of(time_line2));
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::Or(server, time_line1, time_line2) => {
                    let mut servers = servers_of(time_line1);
                    servers.extend(servers_of(time_line2));
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::Not(server, time_line) => {
                    let mut servers = servers_of(time_line);
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::TlDurationWhere(server, time_line) => {
                    let mut servers = servers_of(time_line);
                    servers.extend(to_vec(server));
                    servers
                }
                TimeLineOp::TlDurationInCurState(server, timeline) => {
                    let mut servers = servers_of(timeline);
                    servers.extend(to_vec(server));
                    servers
                }
            }
        }

        servers_of(self)
    }

    pub fn with_worker_details(
        &self,
        worker_prefix: String,
        event_processor_id: String,
        timeline_processor_id: String,
    ) -> TimeLineOp {
        let event_processor_worker_details = TimeLineNodeWorkerInput {
            worker_id_prefix: TimeLineWorkerIdPrefix(worker_prefix.clone()),
            component_id: event_processor_id,
        };

        let timeline_processor_worker_details = TimeLineNodeWorkerInput {
            worker_id_prefix: TimeLineWorkerIdPrefix(worker_prefix.clone()),
            component_id: timeline_processor_id,
        };

        fn go(
            op_builder: TimeLineOp,
            event_processor_worker_details: &TimeLineNodeWorkerInput,
            timeline_processor_worker_details: &TimeLineNodeWorkerInput,
        ) -> TimeLineOp {
            match op_builder {
                TimeLineOp::EqualTo(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::EqualTo(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOp::EqualTo(wd @ Some(_), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::EqualTo(wd, Box::new(child_op), value)
                }
                TimeLineOp::GreaterThan(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThan(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOp::GreaterThan(wd @ Some(_), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThan(wd, Box::new(child_op), value)
                }
                TimeLineOp::GreaterThanOrEqual(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThanOrEqual(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOp::GreaterThanOrEqual(wd @ Some(_), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThanOrEqual(wd, Box::new(child_op), value)
                }
                TimeLineOp::LessThan(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThan(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOp::LessThan(wd @ Some(_), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThan(wd, Box::new(child_op), value)
                }
                TimeLineOp::LessThanOrEqual(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThanOrEqual(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOp::LessThanOrEqual(wd @ Some(_), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThanOrEqual(wd, Box::new(child_op), value)
                }
                TimeLineOp::And(None, left, right) => {
                    let left_child_op = go(
                        *left,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    let right_child_op = go(
                        *right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::And(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineOp::And(wd @ Some(_), left, right) => {
                    let left_child_op = go(
                        *left,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    let right_child_op = go(
                        *right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::And(wd, Box::new(left_child_op), Box::new(right_child_op))
                }
                TimeLineOp::Or(None, left, right) => {
                    let left_child_op = go(
                        *left,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    let right_child_op = go(
                        *right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::Or(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineOp::Or(wd @ Some(_), left, right) => {
                    let left_child_op = go(
                        *left,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    let right_child_op = go(
                        *right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::Or(wd, Box::new(left_child_op), Box::new(right_child_op))
                }
                TimeLineOp::Not(None, op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::Not(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(child_op),
                    )
                }
                TimeLineOp::Not(wd @ Some(_), op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::Not(wd, Box::new(child_op))
                }
                TimeLineOp::TlHasExisted(None, predicate) => TimeLineOp::TlHasExisted(
                    Some(event_processor_worker_details.clone()),
                    predicate,
                ),
                TimeLineOp::TlHasExisted(wd @ Some(_), predicate) => {
                    TimeLineOp::TlHasExisted(wd, predicate)
                }
                TimeLineOp::TlHasExistedWithin(None, predicate, duration) => {
                    TimeLineOp::TlHasExistedWithin(
                        Some(event_processor_worker_details.clone()),
                        predicate,
                        duration,
                    )
                }
                TimeLineOp::TlHasExistedWithin(wd @ Some(_), predicate, duration) => {
                    TimeLineOp::TlHasExistedWithin(wd, predicate, duration)
                }
                TimeLineOp::TlLatestEventToState(None, column) => TimeLineOp::TlLatestEventToState(
                    Some(event_processor_worker_details.clone()),
                    column,
                ),
                TimeLineOp::TlLatestEventToState(wd @ Some(_), column) => {
                    TimeLineOp::TlLatestEventToState(wd, column)
                }
                TimeLineOp::TlDurationWhere(None, op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationWhere(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(child_op),
                    )
                }
                TimeLineOp::TlDurationWhere(wd @ Some(_), op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationWhere(wd, Box::new(child_op))
                }
                TimeLineOp::TlDurationInCurState(None, op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationInCurState(
                        Some(timeline_processor_worker_details.clone()),
                        Box::new(child_op),
                    )
                }
                TimeLineOp::TlDurationInCurState(wd @ Some(_), op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationInCurState(wd, Box::new(child_op))
                }
            }
        }

        go(self.clone(), &event_processor_worker_details, &timeline_processor_worker_details)
    }
}

impl From<TimeLineOp> for OpBuildStage {
    fn from(value: TimeLineOp) -> Self {
        OpBuildStage::Built(value)
    }
}

#[derive(Clone, Debug)]
pub enum TimeLineOpBuilder {
    EqualTo(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>, GolemEventValue),
    GreaterThan(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>, GolemEventValue),
    GreaterThanOrEqual(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>, GolemEventValue),
    LessThan(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>, GolemEventValue),
    LessThanOrEqual(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>, GolemEventValue),
    And(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>, Box<TimeLineOpBuilder>),
    Or(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>, Box<TimeLineOpBuilder>),
    Not(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>),

    TlHasExisted(Option<TimeLineNodeWorkerInput>, GolemEventPredicate<GolemEventValue>),
    TlHasExistedWithin(Option<TimeLineNodeWorkerInput>, GolemEventPredicate<GolemEventValue>, u64),
    TlLatestEventToState(Option<TimeLineNodeWorkerInput>, EventColumnName),
    TlDurationWhere(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>),
    TlDurationInCurState(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>),

    PartiallyBuiltEqualTo(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, GolemEventValue),
    PartiallyBuiltGreaterThan(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, GolemEventValue),
    PartiallyBuiltGreaterThanOrEqual(
        Option<TimeLineNodeWorkerInput>,
        Box<TimeLineOp>,
        GolemEventValue,
    ),
    PartiallyBuiltLessThan(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, GolemEventValue),
    PartiallyBuiltLessThanOrEqual(
        Option<TimeLineNodeWorkerInput>,
        Box<TimeLineOp>,
        GolemEventValue,
    ),

    PartiallyBuiltLeftAnd(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, Box<TimeLineOpBuilder>),
    PartiallyBuiltRightAnd(
        Option<TimeLineNodeWorkerInput>,
        Box<TimeLineOpBuilder>,
        Box<TimeLineOp>,
    ),
    PartiallyBuiltAnd(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, Box<TimeLineOp>),

    PartiallyBuiltLeftOr(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, Box<TimeLineOpBuilder>),
    PartiallyBuiltRightOr(Option<TimeLineNodeWorkerInput>, Box<TimeLineOpBuilder>, Box<TimeLineOp>),
    PartiallyBuiltOr(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>, Box<TimeLineOp>),

    PartiallyBuiltNot(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>),

    PartiallyBuiltTlDurationWhere(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>),
    PartiallyBuiltTlDurationInCurState(Option<TimeLineNodeWorkerInput>, Box<TimeLineOp>),
}

impl From<TimeLineOpBuilder> for OpBuildStage {
    fn from(value: TimeLineOpBuilder) -> Self {
        OpBuildStage::BeingBuilt(value)
    }
}

pub enum OpBuildStage {
    BeingBuilt(TimeLineOpBuilder),
    Built(TimeLineOp),
}

pub fn tl_has_existed(predicate: GolemEventPredicate<GolemEventValue>) -> TimeLineOp {
    TimeLineOp::TlHasExisted(None, predicate)
}

pub fn tl_has_existed_within(
    predicate: GolemEventPredicate<GolemEventValue>,
    duration: u64,
) -> TimeLineOp {
    TimeLineOp::TlHasExistedWithin(None, predicate, duration)
}

pub fn tl_latest_event_to_state(event_col_name: EventColumnName) -> TimeLineOp {
    TimeLineOp::TlLatestEventToState(None, event_col_name)
}

pub fn tl_duration_where(op: TimeLineOp) -> TimeLineOp {
    TimeLineOp::TlDurationWhere(None, Box::new(op))
}

pub fn tl_duration_in_cur_state(op: TimeLineOp) -> TimeLineOp {
    TimeLineOp::TlDurationInCurState(None, Box::new(op))
}

pub fn tl_equal_to(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::EqualTo(None, Box::new(op), value)
}

pub fn tl_greater_than(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::GreaterThan(None, Box::new(op), value)
}

pub fn tl_greater_than_or_equal(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::GreaterThanOrEqual(None, Box::new(op), value)
}
pub fn tl_less_than(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::LessThan(None, Box::new(op), value)
}

pub fn tl_less_than_or_equal(op: TimeLineOp, value: GolemEventValue) -> TimeLineOp {
    TimeLineOp::LessThanOrEqual(None, Box::new(op), value)
}

pub fn tl_and(left: TimeLineOp, right: TimeLineOp) -> TimeLineOp {
    TimeLineOp::And(None, Box::new(left), Box::new(right))
}

pub fn tl_or(left: TimeLineOp, right: TimeLineOp) -> TimeLineOp {
    TimeLineOp::Or(None, Box::new(left), Box::new(right))
}

pub fn tl_not(op: TimeLineOp) -> TimeLineOp {
    TimeLineOp::Not(None, Box::new(op))
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
            TimeLineOp::EqualTo(_, time_line, golem_event_value) => {
                write!(f, "EqualTo({}, {})", time_line, text_of(golem_event_value))
            }
            TimeLineOp::GreaterThan(_, time_line, golem_event_value) => {
                write!(f, "GreaterThan({}, {})", time_line, text_of(golem_event_value))
            }
            TimeLineOp::GreaterThanOrEqual(_, time_line, golem_event_value) => {
                write!(f, "GreaterThanOrEqual({}, {})", time_line, text_of(golem_event_value))
            }
            TimeLineOp::LessThan(_, time_line, golem_event_value) => {
                write!(f, "LessThan({}, {})", time_line, text_of(golem_event_value))
            }
            TimeLineOp::LessThanOrEqual(_, time_line, golem_event_value) => {
                write!(f, "LessThanOrEqual({}, {})", time_line, text_of(golem_event_value))
            }
            TimeLineOp::And(_, time_line1, time_line2) => {
                write!(f, "And({}, {})", time_line1, time_line2)
            }
            TimeLineOp::Or(_, time_line1, time_line2) => {
                write!(f, "Or({}, {})", time_line1, time_line2)
            }
            TimeLineOp::Not(_, time_line) => write!(f, "Not({})", time_line),
            TimeLineOp::TlHasExisted(_, event_predicate) => {
                write!(f, "TlHasExisted({})", event_predicate)
            }
            TimeLineOp::TlHasExistedWithin(_, event_predicate, within_time) => {
                write!(f, "TlHasExistedWithin({}, {})", event_predicate, within_time)
            }
            TimeLineOp::TlLatestEventToState(_, event_column) => {
                write!(f, "TlLatestEventToState({})", event_column)
            }
            TimeLineOp::TlDurationWhere(_, time_line) => {
                write!(f, "TlDurationWhere({})", time_line)
            }
            TimeLineOp::TlDurationInCurState(_, time_line) => {
                write!(f, "TlDurationInCurState({}, )", time_line)
            }
        }
    }
}
