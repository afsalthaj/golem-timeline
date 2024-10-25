use crate::bindings::exports::timeline::timeline_processor::api::{
    DerivedTimelineNode, LeafTimelineNode, TimelineResultWorker, TypedTimelineResultWorker,
};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::event_processor::api::TimelineResult;
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
use crate::bindings::timeline::timeline_processor_stub::stub_timeline_processor;

pub(crate) trait WorkerExt {
    fn get_worker_info(&self) -> WorkerInfo;
}

pub struct WorkerInfo {
    worker_id: String,
    template_id: String,
}

impl WorkerInfo {
    pub fn get_uri(&self) -> Uri {
        Uri { value: format!("worker://{}/{}", self.template_id, self.worker_id) }
    }
}

impl WorkerExt for TimelineResultWorker {
    fn get_worker_info(&self) -> WorkerInfo {
        WorkerInfo { worker_id: self.worker_id.clone(), template_id: self.template_id.clone() }
    }
}

impl WorkerExt for TypedTimelineResultWorker {
    // FIXME: Fix the data structure of TypedTimeLineResultWorker as a product of TimeLineResultWorker and enum of timeline type
    fn get_worker_info(&self) -> WorkerInfo {
        match self {
            TypedTimelineResultWorker::DerivedTimeline(timeline) => match timeline {
                DerivedTimelineNode::EqualTo(result_worker) => result_worker.get_worker_info(),
                DerivedTimelineNode::GreaterThan(result_worker) => result_worker.get_worker_info(),
                DerivedTimelineNode::GreaterThanOrEqualTo(result_worker) => {
                    result_worker.get_worker_info()
                }
                DerivedTimelineNode::LessThan(result_worker) => result_worker.get_worker_info(),
                DerivedTimelineNode::LessThanOrEqualTo(result_worker) => {
                    result_worker.get_worker_info()
                }
                DerivedTimelineNode::And(result_worker) => result_worker.get_worker_info(),
                DerivedTimelineNode::Or(result_worker) => result_worker.get_worker_info(),
                DerivedTimelineNode::Not(result_worker) => result_worker.get_worker_info(),
            },
            TypedTimelineResultWorker::LeafTimeline(timeline) => match timeline {
                LeafTimelineNode::TlHasExisted(result_worker) => result_worker.get_worker_info(),
                LeafTimelineNode::TlHasExistedWithin(result_worker) => {
                    result_worker.get_worker_info()
                }
                LeafTimelineNode::TlLatestEventToState(result_worker) => {
                    result_worker.get_worker_info()
                }
            },
        }
    }
}

pub(crate) trait WorkerResultExt {
    fn get_timeline_result(&self, t1: u64) -> Result<TimelineResult, String>;
}

impl WorkerResultExt for TypedTimelineResultWorker {
    fn get_timeline_result(&self, t1: u64) -> Result<TimelineResult, String> {
        match self {
            TypedTimelineResultWorker::DerivedTimeline(_) => {
                let api = stub_timeline_processor::Api::new(&self.get_worker_info().get_uri());
                api.blocking_get_timeline_result(t1)
            }
            TypedTimelineResultWorker::LeafTimeline(leaf_node) => match leaf_node {
                LeafTimelineNode::TlHasExisted(worker) => {
                    let api = stub_event_processor::Api::new(&worker.get_worker_info().get_uri());
                    api.blocking_tl_has_existed_within(t1)
                }
                LeafTimelineNode::TlHasExistedWithin(worker) => {
                    let api = stub_event_processor::Api::new(&worker.get_worker_info().get_uri());
                    api.blocking_tl_has_existed_within(t1)
                }
                LeafTimelineNode::TlLatestEventToState(worker) => {
                    let api = stub_event_processor::Api::new(&worker.get_worker_info().get_uri());
                    api.blocking_latest_event_to_state(t1)
                }
            },
        }
    }
}
