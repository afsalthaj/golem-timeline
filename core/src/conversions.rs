use std::fmt::{Debug, Display};
use timeline::event_predicate::{EventColumnName, EventColumnValue, GolemEventPredicate};
use timeline::golem_event::{GolemEvent, GolemEventValue};
use timeline::timeline_node_worker::TimeLineNodeWorker;
use timeline::timeline_op::TimeLineOp;
use crate::bindings::exports::timeline::core::api::{EventPredicateOp, TimelineOp as WitTimeLineOp};
use crate::bindings::timeline::event_processor::api::EventValue as WitEventValue;
use crate::bindings::exports::timeline::core::api::Server as WitTimeLineNodeWorker;
use crate::bindings::exports::timeline::core::api::EventPredicate as WitEventPredicate;
use crate::bindings::timeline::event_processor::api::Event as WitEvent;


pub trait Conversion: Clone + Debug {
    type WitType: Clone;
    fn from_wit(input: Self::WitType) -> Self;
    fn to_wit(&self) -> Self::WitType;
}


// Golem Event Value conversion
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


// Golem Event conversion
impl Conversion for GolemEvent {
    type WitType = WitEvent;

    fn from_wit(input: Self::WitType) -> Self {
        GolemEvent {
            time: input.time,
            event: input.event.into_iter().map(|(k, v)| (k, GolemEventValue::from_wit(v))).collect()
        }
    }

    fn to_wit(&self) -> Self::WitType {
        WitEvent {
            time: self.time,
            event: self.event.iter().map(|(k, v)| (k.clone(), v.to_wit())).collect()
        }
    }
}

// Timeline Node Worker conversion
impl Conversion for TimeLineNodeWorker {
    type WitType = WitTimeLineNodeWorker;

    fn from_wit(input: Self::WitType) -> Self {
        TimeLineNodeWorker {
            worker_id: input.worker_id,
            template_id: input.template_id,
        }
    }

    fn to_wit(&self) -> Self::WitType {
        WitTimeLineNodeWorker {
            worker_id: self.worker_id.clone(),
            template_id: self.template_id.clone(),
        }
    }

}

// Event Predicate conversion
impl Conversion for GolemEventPredicate<GolemEventValue> {
    type WitType = WitEventPredicate;

    fn from_wit(input: Self::WitType) -> Self {
        let event_column = EventColumnName(input.col_name.clone());
        let event_value = EventColumnValue::from(GolemEventValue::from_wit(input.value.clone()));
        match input.op {
            EventPredicateOp::Equal => GolemEventPredicate::Equals(event_column, event_value),
            EventPredicateOp::GreaterThan => GolemEventPredicate::GreaterThan(event_column, event_value),
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


// TimeLineOp conversion
impl Conversion for TimeLineOp {
    type WitType = WitTimeLineOp;

    fn from_wit(input: Self::WitType) -> Self {
        assert!(!input.nodes.is_empty());
        internals::build_timeline_tree(&input.nodes[0], &input.nodes)
    }

    fn to_wit(&self) -> Self::WitType {
       panic!("Conversion from TimeLineOp to corresponding WIT type hasn't been done yet")
    }

}

mod internals {
    use timeline::event_predicate::GolemEventPredicate;
    use timeline::golem_event::GolemEventValue;
    use timeline::timeline_node_worker::TimeLineNodeWorker;
    use timeline::timeline_op::TimeLineOp;
    use crate::bindings::exports::timeline::core::api::{TimelineConstantComparator, TimelineNode as WitTimeLineNode};
    use super::Conversion;

    pub(crate) fn build_timeline_tree(node: &crate::bindings::exports::timeline::core::api::TimelineNode, nodes: &[crate::bindings::exports::timeline::core::api::TimelineNode]) -> TimeLineOp {
        match node {
           WitTimeLineNode::TimelineComparison(timeline_constant_compared) => {
                let time_line = build_timeline_tree(&nodes[timeline_constant_compared.timeline as usize], nodes);
                let golem_event_value: GolemEventValue = GolemEventValue::from_wit(timeline_constant_compared.value.clone());
                let timeline_node_worker = TimeLineNodeWorker::from_wit(timeline_constant_compared.server.clone());

                match timeline_constant_compared.op {
                    TimelineConstantComparator::GreaterThan => TimeLineOp::GreaterThan(timeline_node_worker, Box::new(time_line),golem_event_value),
                    TimelineConstantComparator::GreaterThanEqual => TimeLineOp::GreaterThanOrEqual(timeline_node_worker, Box::new(time_line),golem_event_value),
                    TimelineConstantComparator::LessThan => TimeLineOp::LessThan(timeline_node_worker, Box::new(time_line),golem_event_value),
                    TimelineConstantComparator::LessThanEqual => TimeLineOp::LessThanOrEqual(timeline_node_worker, Box::new(time_line),golem_event_value),
                }
            }
           WitTimeLineNode::TimelineNegation(timeline_negation) => {
                let time_line = build_timeline_tree(&nodes[timeline_negation.timeline as usize], nodes);
                let timeline_node_worker: TimeLineNodeWorker = TimeLineNodeWorker::from_wit(timeline_negation.server.clone());

                TimeLineOp::Not(timeline_node_worker, Box::new(time_line))
            }
           WitTimeLineNode::TlHasExisted(timeline_with_event_predicate) => {
                let time_line = build_timeline_tree(&nodes[timeline_with_event_predicate.timeline as usize], nodes);
                let server: TimeLineNodeWorker = TimeLineNodeWorker::from_wit(timeline_with_event_predicate.server.clone());
                let filter = GolemEventPredicate::from_wit(timeline_with_event_predicate.event_predicate.clone());
                TimeLineOp::TlHasExisted(server, Box::new(time_line), filter)
            }

           WitTimeLineNode::TlHasExistedWithin(tl_has_existed_within) => {
                let time_line = build_timeline_tree(&nodes[tl_has_existed_within.filtered.timeline as usize], nodes);
                let max_duration = tl_has_existed_within.time;
                let server: TimeLineNodeWorker = TimeLineNodeWorker::from_wit(tl_has_existed_within.filtered.server.clone());

               let filter = GolemEventPredicate::from_wit(tl_has_existed_within.filtered.event_predicate.clone());

                TimeLineOp::TlHasExistedWithin(server, Box::new(time_line), filter, max_duration)
            }
           WitTimeLineNode::TlDurationWhere(tl) => {
                let time_line = build_timeline_tree(&nodes[tl.timeline.clone() as usize], nodes);

                TimeLineOp::TlDurationWhere(TimeLineNodeWorker::from_wit(tl.server.clone()), Box::new(time_line))
            }
           WitTimeLineNode::TlDurationInCurState(tl) => {
                let time_line = build_timeline_tree(&nodes[tl.timeline as usize], nodes);
                let timeline_node_worker = TimeLineNodeWorker::from_wit(tl.server.clone());
                TimeLineOp::TlDurationInCurState(timeline_node_worker, Box::new(time_line))
            }
           WitTimeLineNode::Leaf(server) => {
               let timeline_node_worker = TimeLineNodeWorker::from_wit(server.clone());

               TimeLineOp::Leaf(timeline_node_worker)
           },
        }
    }


}

