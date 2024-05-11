use std::fmt::Debug;
use timeline::event_predicate::{EventColumnName, EventColumnValue, GolemEventPredicate};
use timeline::golem_event::GolemEventValue;
use timeline::timeline_node_worker::{TimeLineNodeWorkerInput, TimeLineWorkerIdPrefix};
use timeline::timeline_op::TimeLineOp;

use crate::bindings::timeline::event_processor::api::EventPredicate as WitEventPredicate;

use crate::bindings::timeline::core::api::Server as WitTimeLineNodeWorker;
use crate::bindings::timeline::event_processor::api::EventValue as WitEventValue;

use crate::bindings::timeline::core::api::TimelineOp as WitTimeLineOp;
use crate::bindings::timeline::event_processor::api::EventPredicateOp;

use crate::builder::WitValueBuilder;

pub trait Conversion: Clone + Debug {
    type WitType: Clone;
    fn from_wit(input: Self::WitType) -> Self;
    fn to_wit(&self) -> Self::WitType;
}

impl Conversion for GolemEventValue {
    type WitType = WitEventValue;

    fn from_wit(input: Self::WitType) -> Self {
        match input {
            WitEventValue::StringValue(value) => GolemEventValue::StringValue(value),
            WitEventValue::IntValue(value) => GolemEventValue::IntValue(value),
            WitEventValue::BoolValue(value) => GolemEventValue::BoolValue(value),
            WitEventValue::FloatValue(value) => GolemEventValue::FloatValue(value),
        }
    }

    fn to_wit(&self) -> Self::WitType {
        match self {
            GolemEventValue::StringValue(value) => WitEventValue::StringValue(value.clone()),
            GolemEventValue::IntValue(value) => WitEventValue::IntValue(*value),
            GolemEventValue::BoolValue(value) => WitEventValue::BoolValue(*value),
            GolemEventValue::FloatValue(value) => WitEventValue::FloatValue(*value),
        }
    }
}

impl Conversion for GolemEventPredicate<GolemEventValue> {
    type WitType = WitEventPredicate;

    fn from_wit(input: Self::WitType) -> Self {
        let event_column = EventColumnName(input.col_name.clone());
        let event_value = EventColumnValue::from(GolemEventValue::from_wit(input.value.clone()));
        match input.op {
            EventPredicateOp::Equal => GolemEventPredicate::Equals(event_column, event_value),
            EventPredicateOp::GreaterThan => {
                GolemEventPredicate::GreaterThan(event_column, event_value)
            }
            EventPredicateOp::LessThan => GolemEventPredicate::LessThan(event_column, event_value),
        }
    }

    fn to_wit(&self) -> Self::WitType {
        match self {
            GolemEventPredicate::Equals(event_column, event_value) => WitEventPredicate {
                col_name: event_column.0.clone(),
                value: event_value.0.to_wit(),
                op: EventPredicateOp::Equal
            },
            GolemEventPredicate::GreaterThan(event_column, event_value) => WitEventPredicate {
                col_name: event_column.0.clone(),
                value: event_value.0.to_wit(),
                op: EventPredicateOp::GreaterThan
            },
            GolemEventPredicate::LessThan(event_column, event_value) => WitEventPredicate {
                col_name: event_column.0.clone(),
                value: event_value.0.to_wit(),
                op: EventPredicateOp::LessThan
            },
            _ => panic!("Not all possible event predicate represented in WIT. This will be included in near future")
        }
    }
}

impl Conversion for TimeLineNodeWorkerInput {
    type WitType = WitTimeLineNodeWorker;

    fn from_wit(input: Self::WitType) -> Self {
        TimeLineNodeWorkerInput {
            worker_id_prefix: TimeLineWorkerIdPrefix(input.worker_id_prefix),
            component_id: input.template_id,
        }
    }

    fn to_wit(&self) -> Self::WitType {
        WitTimeLineNodeWorker {
            worker_id_prefix: self.worker_id_prefix.0.clone(),
            template_id: self.component_id.clone(),
        }
    }
}

// FIXME: This is repeated in core module because api::TypedTimeLineResultWorker is different because of binding differences

// TimeLineOp conversion
impl Conversion for TimeLineOp {
    type WitType = WitTimeLineOp;

    fn from_wit(input: Self::WitType) -> Self {
        assert!(!input.nodes.is_empty());
        internals::build_timeline_tree(&input.nodes[0], &input.nodes)
    }

    fn to_wit(&self) -> Self::WitType {
        let mut builder = WitValueBuilder::new();
        builder.build_timeline_op(self);
        builder.build()
    }
}

mod internals {
    use timeline::event_predicate::{EventColumnName, GolemEventPredicate};
    use timeline::golem_event::GolemEventValue;
    use timeline::timeline_node_worker::TimeLineNodeWorkerInput;
    use timeline::timeline_op::TimeLineOp;

    use crate::bindings::timeline::core::api::{
        TimelineConstantComparator, TimelineNode as WitTimeLineNode, TimelineNode,
    };

    use super::Conversion;

    pub(crate) fn build_timeline_tree(node: &TimelineNode, nodes: &[TimelineNode]) -> TimeLineOp {
        match node {
            WitTimeLineNode::TimelineComparison(timeline_constant_compared) => {
                let time_line = build_timeline_tree(
                    &nodes[timeline_constant_compared.timeline as usize],
                    nodes,
                );
                let golem_event_value: GolemEventValue =
                    GolemEventValue::from_wit(timeline_constant_compared.value.clone());
                let timeline_node_worker =
                    TimeLineNodeWorkerInput::from_wit(timeline_constant_compared.server.clone());

                match timeline_constant_compared.op {
                    TimelineConstantComparator::GreaterThan => TimeLineOp::GreaterThan(
                        timeline_node_worker,
                        Box::new(time_line),
                        golem_event_value,
                    ),
                    TimelineConstantComparator::GreaterThanEqual => TimeLineOp::GreaterThanOrEqual(
                        timeline_node_worker,
                        Box::new(time_line),
                        golem_event_value,
                    ),
                    TimelineConstantComparator::LessThan => TimeLineOp::LessThan(
                        timeline_node_worker,
                        Box::new(time_line),
                        golem_event_value,
                    ),
                    TimelineConstantComparator::LessThanEqual => TimeLineOp::LessThanOrEqual(
                        timeline_node_worker,
                        Box::new(time_line),
                        golem_event_value,
                    ),
                }
            }
            WitTimeLineNode::TimelineNegation(timeline_negation) => {
                let time_line =
                    build_timeline_tree(&nodes[timeline_negation.timeline as usize], nodes);
                let timeline_node_worker: TimeLineNodeWorkerInput =
                    TimeLineNodeWorkerInput::from_wit(timeline_negation.server.clone());

                TimeLineOp::Not(timeline_node_worker, Box::new(time_line))
            }
            WitTimeLineNode::TlHasExisted(server_with_event_predicate) => {
                let server: TimeLineNodeWorkerInput =
                    TimeLineNodeWorkerInput::from_wit(server_with_event_predicate.server.clone());
                let filter = GolemEventPredicate::from_wit(
                    server_with_event_predicate.event_predicate.clone(),
                );
                TimeLineOp::TlHasExisted(server, filter)
            }

            WitTimeLineNode::TlHasExistedWithin(server_with_event_predicate_within) => {
                let max_duration = server_with_event_predicate_within.time;
                let server: TimeLineNodeWorkerInput = TimeLineNodeWorkerInput::from_wit(
                    server_with_event_predicate_within.filtered.server.clone(),
                );

                let filter = GolemEventPredicate::from_wit(
                    server_with_event_predicate_within.filtered.event_predicate.clone(),
                );

                TimeLineOp::TlHasExistedWithin(server, filter, max_duration)
            }
            WitTimeLineNode::TlDurationWhere(tl) => {
                let time_line = build_timeline_tree(&nodes[tl.timeline as usize], nodes);

                TimeLineOp::TlDurationWhere(
                    TimeLineNodeWorkerInput::from_wit(tl.server.clone()),
                    Box::new(time_line),
                )
            }
            WitTimeLineNode::TlDurationInCurState(tl) => {
                let time_line = build_timeline_tree(&nodes[tl.timeline as usize], nodes);
                let timeline_node_worker = TimeLineNodeWorkerInput::from_wit(tl.server.clone());
                TimeLineOp::TlDurationInCurState(timeline_node_worker, Box::new(time_line))
            }
            TimelineNode::TlLatestEventToState(server_with_event_column_name) => {
                let server =
                    TimeLineNodeWorkerInput::from_wit(server_with_event_column_name.server.clone());
                let event_column_name =
                    EventColumnName(server_with_event_column_name.event_column_name.clone());
                TimeLineOp::TlLatestEventToState(server, event_column_name)
            }
        }
    }
}