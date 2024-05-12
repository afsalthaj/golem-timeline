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

impl TimeLineOpBuilder {
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
            op_builder: TimeLineOpBuilder,
            event_processor_worker_details: &TimeLineNodeWorkerInput,
            timeline_processor_worker_details: &TimeLineNodeWorkerInput,
        ) -> TimeLineOp {
            match op_builder {
                TimeLineOpBuilder::EqualTo(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::EqualTo(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOpBuilder::EqualTo(Some(worker_details), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::EqualTo(worker_details, Box::new(child_op), value)
                }
                TimeLineOpBuilder::GreaterThan(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThan(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOpBuilder::GreaterThan(Some(worker_details), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThan(worker_details, Box::new(child_op), value)
                }
                TimeLineOpBuilder::GreaterThanOrEqual(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThanOrEqual(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOpBuilder::GreaterThanOrEqual(Some(worker_details), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::GreaterThanOrEqual(worker_details, Box::new(child_op), value)
                }
                TimeLineOpBuilder::LessThan(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThan(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOpBuilder::LessThan(Some(worker_details), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThan(worker_details, Box::new(child_op), value)
                }
                TimeLineOpBuilder::LessThanOrEqual(None, op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThanOrEqual(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                        value,
                    )
                }
                TimeLineOpBuilder::LessThanOrEqual(Some(worker_details), op, value) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::LessThanOrEqual(worker_details, Box::new(child_op), value)
                }
                TimeLineOpBuilder::And(None, left, right) => {
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
                        timeline_processor_worker_details.clone(),
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineOpBuilder::And(Some(worker_details), left, right) => {
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
                        worker_details,
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineOpBuilder::Or(None, left, right) => {
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
                        timeline_processor_worker_details.clone(),
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineOpBuilder::Or(Some(worker_details), left, right) => {
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
                        worker_details,
                        Box::new(left_child_op),
                        Box::new(right_child_op),
                    )
                }
                TimeLineOpBuilder::Not(None, op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::Not(timeline_processor_worker_details.clone(), Box::new(child_op))
                }
                TimeLineOpBuilder::Not(Some(worker_details), op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::Not(worker_details, Box::new(child_op))
                }
                TimeLineOpBuilder::TlHasExisted(None, predicate) => {
                    TimeLineOp::TlHasExisted(event_processor_worker_details.clone(), predicate)
                }
                TimeLineOpBuilder::TlHasExisted(Some(worker_details), predicate) => {
                    TimeLineOp::TlHasExisted(worker_details, predicate)
                }
                TimeLineOpBuilder::TlHasExistedWithin(None, predicate, duration) => {
                    TimeLineOp::TlHasExistedWithin(
                        event_processor_worker_details.clone(),
                        predicate,
                        duration,
                    )
                }
                TimeLineOpBuilder::TlHasExistedWithin(
                    Some(worker_details),
                    predicate,
                    duration,
                ) => TimeLineOp::TlHasExistedWithin(worker_details, predicate, duration),
                TimeLineOpBuilder::TlLatestEventToState(None, column) => {
                    TimeLineOp::TlLatestEventToState(event_processor_worker_details.clone(), column)
                }
                TimeLineOpBuilder::TlLatestEventToState(Some(worker_details), column) => {
                    TimeLineOp::TlLatestEventToState(worker_details, column)
                }
                TimeLineOpBuilder::TlDurationWhere(None, op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationWhere(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                    )
                }
                TimeLineOpBuilder::TlDurationWhere(Some(worker_details), op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationWhere(worker_details, Box::new(child_op))
                }
                TimeLineOpBuilder::TlDurationInCurState(None, op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationInCurState(
                        timeline_processor_worker_details.clone(),
                        Box::new(child_op),
                    )
                }
                TimeLineOpBuilder::TlDurationInCurState(Some(worker_details), op) => {
                    let child_op =
                        go(*op, event_processor_worker_details, timeline_processor_worker_details);
                    TimeLineOp::TlDurationInCurState(worker_details, Box::new(child_op))
                }

                TimeLineOpBuilder::PartiallyBuiltEqualTo(None, op, value) => {
                    TimeLineOp::EqualTo(timeline_processor_worker_details.clone(), op, value)
                }
                TimeLineOpBuilder::PartiallyBuiltEqualTo(Some(worker_details), op, value) => {
                    TimeLineOp::EqualTo(worker_details, op, value)
                }
                TimeLineOpBuilder::PartiallyBuiltGreaterThan(None, op, value) => {
                    TimeLineOp::GreaterThan(timeline_processor_worker_details.clone(), op, value)
                }
                TimeLineOpBuilder::PartiallyBuiltGreaterThan(Some(worker_details), op, value) => {
                    TimeLineOp::GreaterThan(worker_details, op, value)
                }
                TimeLineOpBuilder::PartiallyBuiltGreaterThanOrEqual(None, op, value) => {
                    TimeLineOp::GreaterThanOrEqual(
                        timeline_processor_worker_details.clone(),
                        op,
                        value,
                    )
                }
                TimeLineOpBuilder::PartiallyBuiltGreaterThanOrEqual(
                    Some(worker_details),
                    op,
                    value,
                ) => TimeLineOp::GreaterThanOrEqual(worker_details, op, value),
                TimeLineOpBuilder::PartiallyBuiltLessThan(None, op, value) => {
                    TimeLineOp::LessThan(timeline_processor_worker_details.clone(), op, value)
                }
                TimeLineOpBuilder::PartiallyBuiltLessThan(Some(worker_details), op, value) => {
                    TimeLineOp::LessThan(worker_details, op, value)
                }
                TimeLineOpBuilder::PartiallyBuiltLessThanOrEqual(None, op, value) => {
                    TimeLineOp::LessThanOrEqual(
                        timeline_processor_worker_details.clone(),
                        op,
                        value,
                    )
                }
                TimeLineOpBuilder::PartiallyBuiltLessThanOrEqual(
                    Some(worker_details),
                    op,
                    value,
                ) => TimeLineOp::LessThanOrEqual(worker_details, op, value),
                TimeLineOpBuilder::PartiallyBuiltLeftAnd(None, left, right) => {
                    let right_child = go(
                        *right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::And(
                        timeline_processor_worker_details.clone(),
                        left,
                        Box::new(right_child),
                    )
                }
                TimeLineOpBuilder::PartiallyBuiltLeftAnd(Some(worker_details), left, right) => {
                    let right_child = go(
                        *right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::And(worker_details, left, Box::new(right_child))
                }
                TimeLineOpBuilder::PartiallyBuiltRightAnd(None, left, right) => {
                    let left_child = go(
                        *left,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::And(
                        timeline_processor_worker_details.clone(),
                        Box::new(left_child),
                        right,
                    )
                }
                TimeLineOpBuilder::PartiallyBuiltRightAnd(Some(worker_details), left, right) => {
                    let left_child = go(
                        *left,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::And(worker_details, Box::new(left_child), right)
                }
                TimeLineOpBuilder::PartiallyBuiltAnd(None, left, right) => {
                    TimeLineOp::And(timeline_processor_worker_details.clone(), left, right)
                }
                TimeLineOpBuilder::PartiallyBuiltAnd(Some(worker_details), left, right) => {
                    TimeLineOp::And(worker_details, left, right)
                }

                TimeLineOpBuilder::PartiallyBuiltLeftOr(None, left, right) => {
                    let right_child = go(
                        *right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::Or(
                        timeline_processor_worker_details.clone(),
                        left,
                        Box::new(right_child),
                    )
                }
                TimeLineOpBuilder::PartiallyBuiltLeftOr(Some(worker_details), left, right) => {
                    let right_child = go(
                        *right,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::Or(worker_details, left, Box::new(right_child))
                }
                TimeLineOpBuilder::PartiallyBuiltRightOr(None, left, right) => {
                    let left_child = go(
                        *left,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::Or(
                        timeline_processor_worker_details.clone(),
                        Box::new(left_child),
                        right,
                    )
                }
                TimeLineOpBuilder::PartiallyBuiltRightOr(Some(worker_details), left, right) => {
                    let left_child = go(
                        *left,
                        event_processor_worker_details,
                        timeline_processor_worker_details,
                    );
                    TimeLineOp::Or(worker_details, Box::new(left_child), right)
                }
                TimeLineOpBuilder::PartiallyBuiltOr(None, left, right) => {
                    TimeLineOp::Or(timeline_processor_worker_details.clone(), left, right)
                }
                TimeLineOpBuilder::PartiallyBuiltOr(Some(worker_details), left, right) => {
                    TimeLineOp::Or(worker_details, left, right)
                }
                TimeLineOpBuilder::PartiallyBuiltNot(None, op) => {
                    TimeLineOp::Not(timeline_processor_worker_details.clone(), op)
                }
                TimeLineOpBuilder::PartiallyBuiltNot(Some(worker_details), op) => {
                    TimeLineOp::Not(worker_details, op)
                }
                TimeLineOpBuilder::PartiallyBuiltTlDurationWhere(None, op) => {
                    TimeLineOp::TlDurationWhere(timeline_processor_worker_details.clone(), op)
                }
                TimeLineOpBuilder::PartiallyBuiltTlDurationWhere(Some(worker_details), op) => {
                    TimeLineOp::TlDurationWhere(worker_details, op)
                }
                TimeLineOpBuilder::PartiallyBuiltTlDurationInCurState(None, op) => {
                    TimeLineOp::TlDurationInCurState(timeline_processor_worker_details.clone(), op)
                }
                TimeLineOpBuilder::PartiallyBuiltTlDurationInCurState(Some(worker_details), op) => {
                    TimeLineOp::TlDurationInCurState(worker_details, op)
                }
            }
        }

        go(self.clone(), &event_processor_worker_details, &timeline_processor_worker_details)
    }
}

pub enum OpBuildStage {
    BeingBuilt(TimeLineOpBuilder),
    Built(TimeLineOp),
}

pub fn tl_has_existed(predicate: GolemEventPredicate<GolemEventValue>) -> TimeLineOpBuilder {
    TimeLineOpBuilder::TlHasExisted(None, predicate)
}

pub fn tl_has_existed_within(
    predicate: GolemEventPredicate<GolemEventValue>,
    duration: u64,
) -> TimeLineOpBuilder {
    TimeLineOpBuilder::TlHasExistedWithin(None, predicate, duration)
}

pub fn tl_latest_event_to_state(event_col_name: EventColumnName) -> TimeLineOpBuilder {
    TimeLineOpBuilder::TlLatestEventToState(None, event_col_name)
}

pub fn tl_duration_where<T: Into<OpBuildStage>>(op: T) -> TimeLineOpBuilder {
    let op_build_stage = op.into();

    match op_build_stage {
        OpBuildStage::BeingBuilt(op) => TimeLineOpBuilder::TlDurationWhere(None, Box::new(op)),
        OpBuildStage::Built(op) => {
            TimeLineOpBuilder::PartiallyBuiltTlDurationWhere(None, Box::new(op))
        }
    }
}

pub fn tl_duration_in_cur_state<T: Into<OpBuildStage>>(op: T) -> TimeLineOpBuilder {
    let op_build_stage = op.into();

    match op_build_stage {
        OpBuildStage::BeingBuilt(op) => TimeLineOpBuilder::TlDurationInCurState(None, Box::new(op)),
        OpBuildStage::Built(op) => {
            TimeLineOpBuilder::PartiallyBuiltTlDurationInCurState(None, Box::new(op))
        }
    }
}

pub fn tl_equal_to<T: Into<OpBuildStage>>(op: T, value: GolemEventValue) -> TimeLineOpBuilder {
    let op_build_stage = op.into();

    match op_build_stage {
        OpBuildStage::BeingBuilt(op) => TimeLineOpBuilder::EqualTo(None, Box::new(op), value),
        OpBuildStage::Built(op) => {
            TimeLineOpBuilder::PartiallyBuiltEqualTo(None, Box::new(op), value)
        }
    }
}

pub fn tl_greater_than<T: Into<OpBuildStage>>(op: T, value: GolemEventValue) -> TimeLineOpBuilder {
    let op_build_stage = op.into();

    match op_build_stage {
        OpBuildStage::BeingBuilt(op) => TimeLineOpBuilder::GreaterThan(None, Box::new(op), value),
        OpBuildStage::Built(op) => {
            TimeLineOpBuilder::PartiallyBuiltGreaterThan(None, Box::new(op), value)
        }
    }
}

pub fn tl_greater_than_or_equal<T: Into<OpBuildStage>>(
    op: T,
    value: GolemEventValue,
) -> TimeLineOpBuilder {
    let op_build_stage = op.into();

    match op_build_stage {
        OpBuildStage::BeingBuilt(op) => {
            TimeLineOpBuilder::GreaterThanOrEqual(None, Box::new(op), value)
        }
        OpBuildStage::Built(op) => {
            TimeLineOpBuilder::PartiallyBuiltGreaterThanOrEqual(None, Box::new(op), value)
        }
    }
}
pub fn tl_less_than<T: Into<OpBuildStage>>(op: T, value: GolemEventValue) -> TimeLineOpBuilder {
    let op_build_stage = op.into();

    match op_build_stage {
        OpBuildStage::BeingBuilt(op) => TimeLineOpBuilder::LessThan(None, Box::new(op), value),
        OpBuildStage::Built(op) => {
            TimeLineOpBuilder::PartiallyBuiltLessThan(None, Box::new(op), value)
        }
    }
}

pub fn tl_less_than_or_equal<T: Into<OpBuildStage>>(
    op: T,
    value: GolemEventValue,
) -> TimeLineOpBuilder {
    let op_build_stage = op.into();

    match op_build_stage {
        OpBuildStage::BeingBuilt(op) => {
            TimeLineOpBuilder::LessThanOrEqual(None, Box::new(op), value)
        }
        OpBuildStage::Built(op) => {
            TimeLineOpBuilder::PartiallyBuiltLessThanOrEqual(None, Box::new(op), value)
        }
    }
}

pub fn tl_and<S: Into<OpBuildStage>, T: Into<OpBuildStage>>(
    left: S,
    right: T,
) -> TimeLineOpBuilder {
    let left_op_build_stage = left.into();
    let right_op_build_stage = right.into();

    match (left_op_build_stage, right_op_build_stage) {
        (OpBuildStage::Built(left), OpBuildStage::Built(right)) => {
            TimeLineOpBuilder::PartiallyBuiltAnd(None, Box::new(left), Box::new(right))
        }
        (OpBuildStage::Built(left), OpBuildStage::BeingBuilt(right)) => {
            TimeLineOpBuilder::PartiallyBuiltLeftAnd(None, Box::new(left), Box::new(right))
        }
        (OpBuildStage::BeingBuilt(left), OpBuildStage::Built(right)) => {
            TimeLineOpBuilder::PartiallyBuiltRightAnd(None, Box::new(left), Box::new(right))
        }
        (OpBuildStage::BeingBuilt(left), OpBuildStage::BeingBuilt(right)) => {
            TimeLineOpBuilder::And(None, Box::new(left), Box::new(right))
        }
    }
}

pub fn tl_or<S: Into<OpBuildStage>, T: Into<OpBuildStage>>(left: S, right: T) -> TimeLineOpBuilder {
    let left_op_build_stage = left.into();
    let right_op_build_stage = right.into();

    match (left_op_build_stage, right_op_build_stage) {
        (OpBuildStage::Built(left), OpBuildStage::Built(right)) => {
            TimeLineOpBuilder::PartiallyBuiltOr(None, Box::new(left), Box::new(right))
        }
        (OpBuildStage::Built(left), OpBuildStage::BeingBuilt(right)) => {
            TimeLineOpBuilder::PartiallyBuiltLeftOr(None, Box::new(left), Box::new(right))
        }
        (OpBuildStage::BeingBuilt(left), OpBuildStage::Built(right)) => {
            TimeLineOpBuilder::PartiallyBuiltRightOr(None, Box::new(left), Box::new(right))
        }
        (OpBuildStage::BeingBuilt(left), OpBuildStage::BeingBuilt(right)) => {
            TimeLineOpBuilder::Or(None, Box::new(left), Box::new(right))
        }
    }
}

pub fn tl_not<T: Into<OpBuildStage>>(op: T) -> TimeLineOpBuilder {
    let op_build_stage = op.into();

    match op_build_stage {
        OpBuildStage::BeingBuilt(op) => TimeLineOpBuilder::Not(None, Box::new(op)),
        OpBuildStage::Built(op) => TimeLineOpBuilder::PartiallyBuiltNot(None, Box::new(op)),
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
