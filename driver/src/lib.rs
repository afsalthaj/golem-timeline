use crate::bindings::exports::timeline::driver::api::Guest;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::core::api::TimelineNode::{TimelineNegation, TlLatestEventToState};
use crate::bindings::timeline::core::api::{Server, TypedTimelineResultWorker, WorkerDetails};
use crate::bindings::timeline::core::api::{
    ServerWithEventColumnName, TimelineNegated, TimelineOp,
};
use crate::bindings::timeline::core_stub::stub_core;
use crate::bindings::timeline::timeline_processor::api::{DerivedTimelineNode, LeafTimelineNode};
use conversions::Conversion;
use std::fmt::format;

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;
mod conversions;
struct Component;

impl Guest for Component {
    fn run(
        core_template_id: String,
        event_processor_template_id: String,
        timeline_processor_template_id: String,
    ) -> Result<WorkerDetails, String> {
        let uri = Uri {
            value: format!("worker://{core_template_id}/{}", "initialize-timeline"),
        };

        let core = stub_core::Api::new(&uri);
        let timeline_op = TimelineOp {
            nodes: vec![
                TimelineNegation(TimelineNegated {
                    server: Server {
                        template_id: timeline_processor_template_id.to_string(),
                        worker_id_prefix: "cirr".to_string(),
                    },
                    timeline: 1,
                }),
                TlLatestEventToState(ServerWithEventColumnName {
                    server: Server {
                        template_id: event_processor_template_id.to_string(),
                        worker_id_prefix: "cirr".to_string(),
                    },
                    event_column_name: "playerStateChange".to_string(),
                }),
            ],
        };

        match core.initialize_timeline(&timeline_op) {
            Ok(result) => {
                dbg!("Driver Log: Timeline initialized");

                // let serializable_result_worker = serde_json::to_value(
                //     timeline::timeline_node_worker::TypedTimeLineResultWorker::from_wit(
                //         result.result_worker,
                //     ),
                // )
                // .map_err(|err| err.to_string())?;
                //
                // let mut event_processors = vec![];
                //
                // for worker in result.event_processor_workers.iter() {
                //     let event_processor =
                //         timeline::timeline_node_worker::TypedTimeLineResultWorker::from_wit(
                //             worker.clone(),
                //         );
                //     event_processors.push(
                //         serde_json::to_value(event_processor).map_err(|err| err.to_string())?,
                //     );
                // }
                //
                // let driver_result: String =
                //     serde_json::Value::Object(serde_json::Map::from_iter(vec![
                //         ("result_worker".to_string(), serializable_result_worker),
                //         (
                //             "event_processors".to_string(),
                //             serde_json::Value::Array(event_processors),
                //         ),
                //     ]))
                //     .to_string();

                Ok(result)
            }
            Err(error) => {
                dbg!("Driver Log: Error initializing timeline");
                Err(error)
            }
        }
    }
}

fn print_typed_time_line_result_worker(
    typed_timeline_result_worker: TypedTimelineResultWorker,
) -> String {
    match typed_timeline_result_worker {
        TypedTimelineResultWorker::LeafTimeline(leaf_time_line_node) => match leaf_time_line_node {
            LeafTimelineNode::TlHasExisted(timeline_result_worker) => {
                format!(
                    "LeafTimeLine: TlHasExisted: Server: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            LeafTimelineNode::TlHasExistedWithin(timeline_result_worker) => {
                format!(
                    "LeafTimeLine: TlHasExistedWithin: Server: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            LeafTimelineNode::TlLatestEventToState(timeline_result_worker) => {
                format!(
                    "LeafTimeLine: TlEventToLatestState: Server: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
        },
        TypedTimelineResultWorker::DerivedTimeline(derived_timeline) => match derived_timeline {
            DerivedTimelineNode::EqualTo(timeline_result_worker) => {
                format!(
                    "DerivedTimeLine: TlHasExisted: Server: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            DerivedTimelineNode::GreaterThan(timeline_result_worker) => {
                format!(
                    "DerivedTimeLine: GreaterThan: Server: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            DerivedTimelineNode::GreaterThanOrEqualTo(timeline_result_worker) => {
                format!(
                    "DerivedTimeLine: GreaterThanOrEqualTo: Server: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            DerivedTimelineNode::LessThan(timeline_result_worker) => {
                format!(
                    "DerivedTimeLine: LessThan: Server: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            DerivedTimelineNode::LessThanOrEqualTo(timeline_result_worker) => {
                format!(
                    "DerivedTimeLine: LessThanOrEqualTo: Server: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            DerivedTimelineNode::And(timeline_result_worker) => {
                format!(
                    "DerivedTimeLine: And: Left: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            DerivedTimelineNode::Or(timeline_result_worker) => {
                format!(
                    "DerivedTimeLine: Or: Left: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
            DerivedTimelineNode::Not(timeline_result_worker) => {
                format!(
                    "DerivedTimeLine: Not: Left: component_id: {}, worker_id: {}",
                    timeline_result_worker.template_id, timeline_result_worker.worker_id,
                )
            }
        },
    }
}
