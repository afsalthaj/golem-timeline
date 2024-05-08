use std::fmt::Debug;
use timeline::timeline_node_worker::{
    TimeLineResultWorker, TimeLineWorkerId, TypedTimeLineResultWorker,
};

use crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode as WitDerivedTimeLineNode;
use crate::bindings::timeline::timeline_processor::api::LeafTimelineNode as WitLeafTimeLineNode;
use crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker as WitTypedTimeLineResultWorker;

pub trait Conversion: Clone + Debug {
    type WitType: Clone;
    fn from_wit(input: Self::WitType) -> Self;
}

// FIXME: This is repeated in core module because api::TypedTimeLineResultWorker is different because of binding differences

impl Conversion for TypedTimeLineResultWorker {
    type WitType = WitTypedTimeLineResultWorker;

    fn from_wit(input: Self::WitType) -> Self {
        match input {
            WitTypedTimeLineResultWorker::LeafTimeline(leaf_time_line) => match leaf_time_line {
                WitLeafTimeLineNode::TlHasExisted(timeline_result_worker) => {
                    TypedTimeLineResultWorker::tl_has_existed(TimeLineResultWorker {
                        worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                        component_id: timeline_result_worker.template_id.clone(),
                    })
                }
                WitLeafTimeLineNode::TlHasExistedWithin(timeline_result_worker) => {
                    TypedTimeLineResultWorker::tl_has_existed_within(TimeLineResultWorker {
                        worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                        component_id: timeline_result_worker.template_id.clone(),
                    })
                }
                WitLeafTimeLineNode::TlLatestEventToState(timeline_result_worker) => {
                    TypedTimeLineResultWorker::tl_event_to_latest_state(TimeLineResultWorker {
                        worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                        component_id: timeline_result_worker.template_id.clone(),
                    })
                }
            },

            WitTypedTimeLineResultWorker::DerivedTimeline(derived_timeline) => {
                match derived_timeline {
                    WitDerivedTimeLineNode::EqualTo(timeline_result_worker) => {
                        TypedTimeLineResultWorker::equal_to(TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            component_id: timeline_result_worker.template_id.clone(),
                        })
                    }
                    WitDerivedTimeLineNode::GreaterThan(timeline_result_worker) => {
                        TypedTimeLineResultWorker::greater_than(TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            component_id: timeline_result_worker.template_id.clone(),
                        })
                    }
                    WitDerivedTimeLineNode::GreaterThanOrEqualTo(timeline_result_worker) => {
                        TypedTimeLineResultWorker::greater_than_or_equal_to(TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            component_id: timeline_result_worker.template_id.clone(),
                        })
                    }
                    WitDerivedTimeLineNode::LessThan(timeline_result_worker) => {
                        TypedTimeLineResultWorker::less_than(TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            component_id: timeline_result_worker.template_id.clone(),
                        })
                    }
                    WitDerivedTimeLineNode::LessThanOrEqualTo(timeline_result_worker) => {
                        TypedTimeLineResultWorker::less_than_or_equal_to(TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            component_id: timeline_result_worker.template_id.clone(),
                        })
                    }
                    WitDerivedTimeLineNode::And(timeline_result_worker) => {
                        TypedTimeLineResultWorker::and(TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            component_id: timeline_result_worker.template_id.clone(),
                        })
                    }
                    WitDerivedTimeLineNode::Or(timeline_result_worker) => {
                        TypedTimeLineResultWorker::or(TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            component_id: timeline_result_worker.template_id.clone(),
                        })
                    }
                    WitDerivedTimeLineNode::Not(timeline_result_worker) => {
                        TypedTimeLineResultWorker::not(TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            component_id: timeline_result_worker.template_id.clone(),
                        })
                    }
                }
            }
        }
    }
}
