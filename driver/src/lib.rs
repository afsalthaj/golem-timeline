use crate::bindings::exports::timeline::driver::api::Guest;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::core::api::{Server, TypedTimelineResultWorker};
use crate::bindings::timeline::core::api::TimelineNode::{TlLatestEventToState, TimelineNegation};
use crate::bindings::timeline::core::api::{ServerWithEventColumnName, TimelineOp, TimelineNegated};
use crate::bindings::timeline::core_stub::stub_core;
use crate::bindings::timeline::timeline_processor::api::{DerivedTimelineNode, LeafTimelineNode};

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;
struct Component;

impl Guest for Component {
    fn run(
        core_template_id: String,
        event_processor_template_id: String,
        timeline_processor_template_id: String,
    ) -> Result<String, String> {
        let uri = Uri {
            value: format!("worker://{core_template_id}/{}", "initialize-timeline"),
        };

        let core = stub_core::Api::new(&uri);
        let timeline_op = TimelineOp {
            nodes: vec![ TimelineNegation(TimelineNegated{
                server: Server {
                    template_id: timeline_processor_template_id.to_string(),
                    worker_id_prefix: "cirr".to_string(),
                },
                timeline: 1
            }),  TlLatestEventToState(ServerWithEventColumnName {
                server: Server {
                    template_id: event_processor_template_id.to_string(),
                    worker_id_prefix: "cirr".to_string(),
                },
                event_column_name: "playerStateChange".to_string(),
            })],
        };

        match core.initialize_timeline(&timeline_op) {
            Ok(result) => {
                dbg!("Driver Log: Timeline initialized");
                Ok(print_typed_time_line_result_worker(result))
            }
            Err(error) => {
                dbg!("Driver Log: Error initializing timeline");
                Err(error)
            }
        }
    }
}

fn print_typed_time_line_result_worker(typed_timeline_result_worker: TypedTimelineResultWorker) -> String {
    match typed_timeline_result_worker {
        TypedTimelineResultWorker::LeafTimeline(leaf_time_line_node) => {
            match leaf_time_line_node {
                LeafTimelineNode::TlHasExisted(timeline_result_worker) => {
                    format!(
                        "LeafTimeLine: TlHasExisted: Server: template_id: {}, worker_id: {}",
                        timeline_result_worker.template_id,
                        timeline_result_worker.worker_id,
                    )
                },
                LeafTimelineNode::TlHasExistedWithin(timeline_result_worker) => {
                    format!(
                        "LeafTimeLine: TlHasExistedWithin: Server: template_id: {}, worker_id: {}",
                        timeline_result_worker.template_id,
                        timeline_result_worker.worker_id,
                    )
                },
                LeafTimelineNode::TlLatestEventToState(timeline_result_worker) => {
                    format!(
                        "LeafTimeLine: TlEventToLatestState: Server: template_id: {}, worker_id: {}",
                        timeline_result_worker.template_id,
                        timeline_result_worker.worker_id,
                    )
                },
            }
        }
        TypedTimelineResultWorker::DerivedTimeline(derived_timeline) => {
           match derived_timeline {
               DerivedTimelineNode::EqualTo(timeline_result_worker) => {
                   format!(
                       "DerivedTimeLine: TlHasExisted: Server: template_id: {}, worker_id: {}",
                       timeline_result_worker.template_id,
                       timeline_result_worker.worker_id,
                   )
               },
                DerivedTimelineNode::GreaterThan(timeline_result_worker) => {
                     format!(
                          "DerivedTimeLine: GreaterThan: Server: template_id: {}, worker_id: {}",
                          timeline_result_worker.template_id,
                          timeline_result_worker.worker_id,
                     )
                },
                DerivedTimelineNode::GreaterThanOrEqualTo(timeline_result_worker) => {
                     format!(
                          "DerivedTimeLine: GreaterThanOrEqualTo: Server: template_id: {}, worker_id: {}",
                          timeline_result_worker.template_id,
                          timeline_result_worker.worker_id,
                     )
                },
                DerivedTimelineNode::LessThan(timeline_result_worker) => {
                     format!(
                          "DerivedTimeLine: LessThan: Server: template_id: {}, worker_id: {}",
                          timeline_result_worker.template_id,
                          timeline_result_worker.worker_id,
                     )
                },
               DerivedTimelineNode::LessThanOrEqualTo(timeline_result_worker) => {
                     format!(
                          "DerivedTimeLine: LessThanOrEqualTo: Server: template_id: {}, worker_id: {}",
                          timeline_result_worker.template_id,
                          timeline_result_worker.worker_id,
                     )
                },
                DerivedTimelineNode::And(timeline_result_worker) => {
                     format!(
                          "DerivedTimeLine: And: Left: template_id: {}, worker_id: {}",
                          timeline_result_worker.template_id,
                          timeline_result_worker.worker_id,
                     )
                },
                DerivedTimelineNode::Or(timeline_result_worker) => {
                     format!(
                          "DerivedTimeLine: Or: Left: template_id: {}, worker_id: {}",
                          timeline_result_worker.template_id,
                          timeline_result_worker.worker_id,
                     )
                },
                DerivedTimelineNode::Not(timeline_result_worker) => {
                     format!(
                          "DerivedTimeLine: Not: Left: template_id: {}, worker_id: {}",
                          timeline_result_worker.template_id,
                          timeline_result_worker.worker_id,
                     )
                },

           }
        }
    }
}
