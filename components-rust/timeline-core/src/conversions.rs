use common_lib::{EventColumnName, GolemEvent, GolemEventPredicate, GolemEventValue};

use crate::types::*;

impl EventValue {
    pub(crate) fn to_domain(&self) -> GolemEventValue {
        match self {
            EventValue::StringValue(s) => GolemEventValue::StringValue(s.clone()),
            EventValue::IntValue(i) => GolemEventValue::IntValue(*i),
            EventValue::FloatValue(f) => GolemEventValue::FloatValue(*f),
            EventValue::BoolValue(b) => GolemEventValue::BoolValue(*b),
        }
    }

    pub(crate) fn into_domain(self) -> GolemEventValue {
        match self {
            EventValue::StringValue(s) => GolemEventValue::StringValue(s),
            EventValue::IntValue(i) => GolemEventValue::IntValue(i),
            EventValue::FloatValue(f) => GolemEventValue::FloatValue(f),
            EventValue::BoolValue(b) => GolemEventValue::BoolValue(b),
        }
    }

    pub(crate) fn from_domain(v: &GolemEventValue) -> Self {
        match v {
            GolemEventValue::StringValue(s) => EventValue::StringValue(s.clone()),
            GolemEventValue::IntValue(i) => EventValue::IntValue(*i),
            GolemEventValue::FloatValue(f) => EventValue::FloatValue(*f),
            GolemEventValue::BoolValue(b) => EventValue::BoolValue(*b),
        }
    }

    pub(crate) fn from_domain_owned(v: GolemEventValue) -> Self {
        match v {
            GolemEventValue::StringValue(s) => EventValue::StringValue(s),
            GolemEventValue::IntValue(i) => EventValue::IntValue(i),
            GolemEventValue::FloatValue(f) => EventValue::FloatValue(f),
            GolemEventValue::BoolValue(b) => EventValue::BoolValue(b),
        }
    }
}

impl EventPredicate {
    pub(crate) fn to_domain(&self) -> GolemEventPredicate<GolemEventValue> {
        let col = EventColumnName(self.col_name.clone());
        let val = common_lib::EventColumnValue(self.value.to_domain());
        match self.op {
            PredicateOp::Equal => GolemEventPredicate::Equals(col, val),
            PredicateOp::GreaterThan => GolemEventPredicate::GreaterThan(col, val),
            PredicateOp::LessThan => GolemEventPredicate::LessThan(col, val),
        }
    }
}

impl Event {
    pub(crate) fn into_domain(self) -> GolemEvent<GolemEventValue> {
        let map = self
            .event
            .into_iter()
            .map(|(k, v)| (EventColumnName(k), v.into_domain()))
            .collect();
        GolemEvent {
            time: self.time,
            event: map,
        }
    }
}

impl TimelineOpGraph {
    /// Convert the flat graph back to the recursive `TimeLineOp`.
    pub fn to_recursive(&self) -> common_lib::TimeLineOp {
        self.build_node(0)
    }

    fn build_node(&self, idx: usize) -> common_lib::TimeLineOp {
        match &self.nodes[idx] {
            TimelineNode::Comparison(op, child, value) => {
                let child_op = self.build_node(*child as usize);
                let v = value.to_domain();
                match op {
                    CompareOp::EqualTo => common_lib::TimeLineOp::EqualTo(Box::new(child_op), v),
                    CompareOp::GreaterThan => {
                        common_lib::TimeLineOp::GreaterThan(Box::new(child_op), v)
                    }
                    CompareOp::GreaterThanOrEqual => {
                        common_lib::TimeLineOp::GreaterThanOrEqual(Box::new(child_op), v)
                    }
                    CompareOp::LessThan => common_lib::TimeLineOp::LessThan(Box::new(child_op), v),
                    CompareOp::LessThanOrEqual => {
                        common_lib::TimeLineOp::LessThanOrEqual(Box::new(child_op), v)
                    }
                }
            }
            TimelineNode::Negation(child) => {
                common_lib::TimeLineOp::Not(Box::new(self.build_node(*child as usize)))
            }
            TimelineNode::And(l, r) => common_lib::TimeLineOp::And(
                Box::new(self.build_node(*l as usize)),
                Box::new(self.build_node(*r as usize)),
            ),
            TimelineNode::Or(l, r) => common_lib::TimeLineOp::Or(
                Box::new(self.build_node(*l as usize)),
                Box::new(self.build_node(*r as usize)),
            ),
            TimelineNode::TlHasExisted(pred) => {
                common_lib::TimeLineOp::TlHasExisted(pred.to_domain())
            }
            TimelineNode::TlHasExistedWithin(pred, dur) => {
                common_lib::TimeLineOp::TlHasExistedWithin(pred.to_domain(), *dur)
            }
            TimelineNode::TlLatestEventToState(col) => {
                common_lib::TimeLineOp::TlLatestEventToState(EventColumnName(col.clone()))
            }
            TimelineNode::TlDurationWhere(child) => {
                common_lib::TimeLineOp::TlDurationWhere(Box::new(self.build_node(*child as usize)))
            }
            TimelineNode::TlDurationInCurState(child) => {
                common_lib::TimeLineOp::TlDurationInCurState(Box::new(
                    self.build_node(*child as usize),
                ))
            }
        }
    }
}

/// Build a `TimelineOpGraph` from a recursive `TimeLineOp`.
pub fn to_graph(op: &common_lib::TimeLineOp) -> TimelineOpGraph {
    let mut nodes = Vec::new();
    build_graph_node(op, &mut nodes);
    TimelineOpGraph { nodes }
}

fn build_graph_node(op: &common_lib::TimeLineOp, nodes: &mut Vec<TimelineNode>) -> NodeIndex {
    match op {
        common_lib::TimeLineOp::EqualTo(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(CompareOp::EqualTo, -1, EventValue::from_domain(v)));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] { *c = child_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::GreaterThan(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(CompareOp::GreaterThan, -1, EventValue::from_domain(v)));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] { *c = child_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::GreaterThanOrEqual(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(CompareOp::GreaterThanOrEqual, -1, EventValue::from_domain(v)));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] { *c = child_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::LessThan(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(CompareOp::LessThan, -1, EventValue::from_domain(v)));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] { *c = child_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::LessThanOrEqual(child, v) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Comparison(CompareOp::LessThanOrEqual, -1, EventValue::from_domain(v)));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Comparison(_, ref mut c, _) = nodes[parent_idx] { *c = child_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::Not(child) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Negation(-1));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::Negation(ref mut c) = nodes[parent_idx] { *c = child_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::And(l, r) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::And(-1, -1));
            let l_idx = build_graph_node(l, nodes);
            let r_idx = build_graph_node(r, nodes);
            if let TimelineNode::And(ref mut li, ref mut ri) = nodes[parent_idx] { *li = l_idx; *ri = r_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::Or(l, r) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::Or(-1, -1));
            let l_idx = build_graph_node(l, nodes);
            let r_idx = build_graph_node(r, nodes);
            if let TimelineNode::Or(ref mut li, ref mut ri) = nodes[parent_idx] { *li = l_idx; *ri = r_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::TlHasExisted(pred) => {
            let idx = nodes.len();
            nodes.push(TimelineNode::TlHasExisted(predicate_to_api(pred)));
            idx as NodeIndex
        }
        common_lib::TimeLineOp::TlHasExistedWithin(pred, dur) => {
            let idx = nodes.len();
            nodes.push(TimelineNode::TlHasExistedWithin(predicate_to_api(pred), *dur));
            idx as NodeIndex
        }
        common_lib::TimeLineOp::TlLatestEventToState(col) => {
            let idx = nodes.len();
            nodes.push(TimelineNode::TlLatestEventToState(col.0.clone()));
            idx as NodeIndex
        }
        common_lib::TimeLineOp::TlDurationWhere(child) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::TlDurationWhere(-1));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::TlDurationWhere(ref mut c) = nodes[parent_idx] { *c = child_idx; }
            parent_idx as NodeIndex
        }
        common_lib::TimeLineOp::TlDurationInCurState(child) => {
            let parent_idx = nodes.len();
            nodes.push(TimelineNode::TlDurationInCurState(-1));
            let child_idx = build_graph_node(child, nodes);
            if let TimelineNode::TlDurationInCurState(ref mut c) = nodes[parent_idx] { *c = child_idx; }
            parent_idx as NodeIndex
        }
    }
}

pub(crate) fn predicate_to_api(pred: &GolemEventPredicate<GolemEventValue>) -> EventPredicate {
    match pred {
        GolemEventPredicate::Equals(col, val) => EventPredicate {
            col_name: col.0.clone(),
            value: EventValue::from_domain(&val.0),
            op: PredicateOp::Equal,
        },
        GolemEventPredicate::GreaterThan(col, val) => EventPredicate {
            col_name: col.0.clone(),
            value: EventValue::from_domain(&val.0),
            op: PredicateOp::GreaterThan,
        },
        GolemEventPredicate::LessThan(col, val) => EventPredicate {
            col_name: col.0.clone(),
            value: EventValue::from_domain(&val.0),
            op: PredicateOp::LessThan,
        },
        _ => panic!("Compound predicates (And/Or) not yet supported in API encoding"),
    }
}
