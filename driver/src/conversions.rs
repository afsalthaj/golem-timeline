use std::fmt::Debug;
use timeline::timeline_node_worker::{DerivedTimeLineNode, LeafTimeLineNode, TimeLineResultWorker, TimeLineWorkerId, TypedTimeLineResultWorker};


use crate::bindings::timeline::timeline_processor::api::{TypedTimelineResultWorker as WitTypedTimeLineResultWorker};
use crate::bindings::timeline::timeline_processor::api::LeafTimelineNode as WitLeafTimeLineNode;
use crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode as WitDerivedTimeLineNode;
use crate::bindings::timeline::timeline_processor::api::TimelineResultWorker as WitTimeLineResultWorker;

pub trait Conversion: Clone + Debug {
    type WitType: Clone;
    fn from_wit(input: Self::WitType) -> Self;
    fn to_wit(&self) -> Self::WitType;
}

// FIXME: This is repeated in core module because api::TypedTimeLineResultWorker is different because of binding differences

impl Conversion for TypedTimeLineResultWorker {
    type WitType = WitTypedTimeLineResultWorker;

    fn from_wit(input: Self::WitType) -> Self {
        match input {
            WitTypedTimeLineResultWorker::LeafTimeline(leaf_time_line) => {
                match leaf_time_line {
                    WitLeafTimeLineNode::TlHasExisted(timeline_result_worker) => TypedTimeLineResultWorker::tl_has_existed(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitLeafTimeLineNode::TlHasExistedWithin(timeline_result_worker) => TypedTimeLineResultWorker::tl_has_existed_within(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitLeafTimeLineNode::TlLatestEventToState(timeline_result_worker) => TypedTimeLineResultWorker::tl_event_to_latest_state(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                }
            }

            WitTypedTimeLineResultWorker::DerivedTimeline(derived_timeline) => {
                match derived_timeline {
                    WitDerivedTimeLineNode::EqualTo(timeline_result_worker) => TypedTimeLineResultWorker::equal_to(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitDerivedTimeLineNode::GreaterThan(timeline_result_worker) => TypedTimeLineResultWorker::greater_than(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitDerivedTimeLineNode::GreaterThanOrEqualTo(timeline_result_worker) => TypedTimeLineResultWorker::greater_than_or_equal_to(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitDerivedTimeLineNode::LessThan(timeline_result_worker) => TypedTimeLineResultWorker::less_than(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitDerivedTimeLineNode::LessThanOrEqualTo(timeline_result_worker) => TypedTimeLineResultWorker::less_than_or_equal_to(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitDerivedTimeLineNode::And(timeline_result_worker) => TypedTimeLineResultWorker::and(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitDerivedTimeLineNode::Or(timeline_result_worker) => TypedTimeLineResultWorker::or(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                    WitDerivedTimeLineNode::Not(timeline_result_worker) => TypedTimeLineResultWorker::not(
                        TimeLineResultWorker {
                            worker_id: TimeLineWorkerId(timeline_result_worker.worker_id.clone()),
                            template_id: timeline_result_worker.template_id.clone(),
                        }
                    ),
                }
            }
        }
    }

    fn to_wit(&self) -> Self::WitType {
        match self {
            TypedTimeLineResultWorker::LeafTimeLine(leaf_timeline) => match leaf_timeline {
                LeafTimeLineNode::TLHasExisted { time_line_worker } => {
                    let worker = time_line_worker.clone().worker_id.0;
                    let template_id = time_line_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::LeafTimeline(WitLeafTimeLineNode::TlHasExisted(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },
                LeafTimeLineNode::TLHasExistedWithin { time_line_worker } => {
                    let worker = time_line_worker.clone().worker_id.0;
                    let template_id = time_line_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::LeafTimeline(WitLeafTimeLineNode::TlHasExistedWithin(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },

                LeafTimeLineNode::TLEventToLatestState { time_line_worker } => {
                    let worker = time_line_worker.clone().worker_id.0;
                    let template_id = time_line_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::LeafTimeline(WitLeafTimeLineNode::TlLatestEventToState(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },
            },

            TypedTimeLineResultWorker::DerivedTimeLine(derived_timeline) => match derived_timeline {
                DerivedTimeLineNode::EqualTo { result_worker } => {
                    let worker = result_worker.clone().worker_id.0;
                    let template_id = result_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::DerivedTimeline(WitDerivedTimeLineNode::EqualTo(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },
                DerivedTimeLineNode::GreaterThan { result_worker } => {
                    let worker = result_worker.clone().worker_id.0;
                    let template_id = result_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::DerivedTimeline(WitDerivedTimeLineNode::GreaterThan(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },
                DerivedTimeLineNode::GreaterThanOrEqualTo { result_worker } => {
                    let worker = result_worker.clone().worker_id.0;
                    let template_id = result_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::DerivedTimeline(WitDerivedTimeLineNode::GreaterThanOrEqualTo(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },
                DerivedTimeLineNode::LessThan { result_worker } => {
                    let worker = result_worker.clone().worker_id.0;
                    let template_id = result_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::DerivedTimeline(WitDerivedTimeLineNode::LessThan(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },
                DerivedTimeLineNode::LessThanOrEqualTo { result_worker } => {
                    let worker = result_worker.clone().worker_id.0;
                    let template_id = result_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::DerivedTimeline(WitDerivedTimeLineNode::LessThanOrEqualTo(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },

                DerivedTimeLineNode::And { result_worker } => {
                    let worker = result_worker.clone().worker_id.0;
                    let template_id = result_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::DerivedTimeline(WitDerivedTimeLineNode::And(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        }
                    ))
                },
                DerivedTimeLineNode::Or { result_worker } => {
                    let worker = result_worker.clone().worker_id.0;
                    let template_id = result_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::DerivedTimeline(WitDerivedTimeLineNode::Or(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        }
                    ))
                },
                DerivedTimeLineNode::Not { result_worker } => {
                    let worker = result_worker.clone().worker_id.0;
                    let template_id = result_worker.clone().template_id;
                    WitTypedTimeLineResultWorker::DerivedTimeline(WitDerivedTimeLineNode::Not(
                        WitTimeLineResultWorker {
                            template_id,
                            worker_id: worker,
                        },
                    ))
                },
            }
        }
    }
}
