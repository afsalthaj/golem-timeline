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
use timeline::timeline_op::TimeLineOp;

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
        let uri = Uri { value: format!("worker://{core_template_id}/{}", "initialize-timeline") };

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
                Ok(result)
            }
            Err(error) => {
                dbg!("Driver Log: Error initializing timeline");
                Err(error)
            }
        }
    }
}
