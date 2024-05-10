use crate::bindings::exports::timeline::core::api::TimelineOp as WitTimeLineOp;
use crate::bindings::exports::timeline::core::api::{
    NodeIndex, ServerWithEventColumnName, ServerWithEventPredicate, ServerWithEventPredicateWithin,
    TimelineConstantComparator, TimelineConstantCompared, TimelineNegated, TimelineNode,
    TimelineWithServer,
};
use crate::conversions::Conversion;
use timeline::timeline_op::TimeLineOp;

pub struct WitValueBuilder {
    nodes: Vec<TimelineNode>,
}

impl WitValueBuilder {
    pub(crate) fn new() -> Self {
        WitValueBuilder { nodes: Vec::new() }
    }

    fn add(&mut self, node: TimelineNode) -> NodeIndex {
        self.nodes.push(node);
        self.nodes.len() as NodeIndex - 1
    }

    // FIXME: Clone is not needed
    pub(crate) fn build(&self) -> WitTimeLineOp {
        WitTimeLineOp {
            nodes: self.nodes.clone(),
        }
    }

    pub(crate) fn build_timeline_op(&mut self, timeline_op: &TimeLineOp) -> NodeIndex {
        match timeline_op {
            TimeLineOp::TlHasExisted(timeline_worker_input, event_predicate) => {
                let server = timeline_worker_input.to_wit();
                let event_predicate = event_predicate.to_wit();

                let timeline_node = TimelineNode::TlHasExisted(ServerWithEventPredicate {
                    server,
                    event_predicate,
                });
                self.add(timeline_node)
            }

            TimeLineOp::TlLatestEventToState(timeline_worker_input, event_column_name) => {
                let server = timeline_worker_input.to_wit();
                let event_column_name = event_column_name.0.clone();

                let timeline_node = TimelineNode::TlLatestEventToState(ServerWithEventColumnName {
                    server,
                    event_column_name,
                });
                self.add(timeline_node)
            }

            TimeLineOp::Not(timeline_worker_input, timeline_op) => {
                let server = timeline_worker_input.to_wit();
                let parent_idx = self.add(TimelineNode::TimelineNegation(TimelineNegated {
                    server,
                    timeline: -1,
                }));

                let child_idx = self.build_timeline_op(timeline_op);

                match &mut self.nodes[parent_idx as usize] {
                    TimelineNode::TimelineNegation(negated) => {
                        negated.timeline = child_idx;
                    }
                    _ => unreachable!(),
                }
                parent_idx
            }

            TimeLineOp::GreaterThan(timeline_worker_input, timeline_op, golem_event_value) => {
                let parent_idx =
                    self.add(TimelineNode::TimelineComparison(TimelineConstantCompared {
                        op: TimelineConstantComparator::GreaterThan,
                        timeline: -1,
                        value: golem_event_value.to_wit(),
                        server: timeline_worker_input.to_wit(),
                    }));

                let child_idx = self.build_timeline_op(timeline_op);

                match &mut self.nodes[parent_idx as usize] {
                    TimelineNode::TimelineComparison(timeline_constant_compared) => {
                        timeline_constant_compared.timeline = child_idx;
                    }
                    _ => unreachable!(),
                }

                parent_idx
            }

            TimeLineOp::GreaterThanOrEqual(
                timeline_worker_input,
                timeline_op,
                golem_event_value,
            ) => {
                let parent_idx =
                    self.add(TimelineNode::TimelineComparison(TimelineConstantCompared {
                        op: TimelineConstantComparator::GreaterThanEqual,
                        timeline: -1,
                        value: golem_event_value.to_wit(),
                        server: timeline_worker_input.to_wit(),
                    }));

                let child_idx = self.build_timeline_op(timeline_op);

                match &mut self.nodes[parent_idx as usize] {
                    TimelineNode::TimelineComparison(timeline_constant_compared) => {
                        timeline_constant_compared.timeline = child_idx;
                    }
                    _ => unreachable!(),
                }

                parent_idx
            }

            TimeLineOp::LessThan(timeline_worker_input, timeline_op, golem_event_value) => {
                let parent_idx =
                    self.add(TimelineNode::TimelineComparison(TimelineConstantCompared {
                        op: TimelineConstantComparator::LessThan,
                        timeline: -1,
                        value: golem_event_value.to_wit(),
                        server: timeline_worker_input.to_wit(),
                    }));

                let child_idx = self.build_timeline_op(timeline_op);

                match &mut self.nodes[parent_idx as usize] {
                    TimelineNode::TimelineComparison(timeline_constant_compared) => {
                        timeline_constant_compared.timeline = child_idx;
                    }
                    _ => unreachable!(),
                }

                parent_idx
            }

            TimeLineOp::LessThanOrEqual(timeline_worker_input, timeline_op, golem_event_value) => {
                let parent_idx =
                    self.add(TimelineNode::TimelineComparison(TimelineConstantCompared {
                        op: TimelineConstantComparator::LessThanEqual,
                        timeline: -1,
                        value: golem_event_value.to_wit(),
                        server: timeline_worker_input.to_wit(),
                    }));

                let child_idx = self.build_timeline_op(timeline_op);

                match &mut self.nodes[parent_idx as usize] {
                    TimelineNode::TimelineComparison(timeline_constant_compared) => {
                        timeline_constant_compared.timeline = child_idx;
                    }
                    _ => unreachable!(),
                }

                parent_idx
            }

            TimeLineOp::EqualTo(timeline_worker_input, timeline_op, golem_event_value) => {
                let parent_idx =
                    self.add(TimelineNode::TimelineComparison(TimelineConstantCompared {
                        op: TimelineConstantComparator::GreaterThan, // FIXME: Add Equal to ConstantOp
                        timeline: -1,
                        value: golem_event_value.to_wit(),
                        server: timeline_worker_input.to_wit(),
                    }));

                let child_idx = self.build_timeline_op(timeline_op);

                match &mut self.nodes[parent_idx as usize] {
                    TimelineNode::TimelineComparison(timeline_constant_compared) => {
                        timeline_constant_compared.timeline = child_idx;
                    }
                    _ => unreachable!(),
                }

                parent_idx
            }

            TimeLineOp::TlDurationInCurState(timeline_worker_input, timeline_op) => {
                let parent_idx = self.add(TimelineNode::TlDurationInCurState(TimelineWithServer {
                    server: timeline_worker_input.to_wit(),
                    timeline: -1,
                }));

                let child_idx = self.build_timeline_op(timeline_op);

                match &mut self.nodes[parent_idx as usize] {
                    TimelineNode::TlDurationInCurState(ref mut timeline) => {
                        timeline.timeline = child_idx;
                    }
                    _ => unreachable!(),
                }

                parent_idx
            }

            TimeLineOp::TlDurationWhere(timeline_worker_input, timeline_op) => {
                let parent_idx = self.add(TimelineNode::TlDurationWhere(TimelineWithServer {
                    server: timeline_worker_input.to_wit(),
                    timeline: -1,
                }));

                let child_idx = self.build_timeline_op(timeline_op);

                match &mut self.nodes[parent_idx as usize] {
                    TimelineNode::TlDurationWhere(timeline) => {
                        timeline.timeline = child_idx;
                    }
                    _ => unreachable!(),
                }

                parent_idx
            }
            TimeLineOp::TlHasExistedWithin(timeline_worker_input, event_predicate, time) => self
                .add(TimelineNode::TlHasExistedWithin(
                    ServerWithEventPredicateWithin {
                        filtered: ServerWithEventPredicate {
                            server: timeline_worker_input.to_wit(),
                            event_predicate: event_predicate.to_wit(),
                        },
                        time: *time,
                    },
                )),
            TimeLineOp::And(_timeline_worker_input, _timeline_op1, _timeline_op2) => {
                unimplemented!("And") //FIXME
            }
            TimeLineOp::Or(_, _, _) => {
                unimplemented!("Or") //FIXME
            }
        }
    }
}
