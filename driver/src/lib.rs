use crate::bindings::exports::timeline::driver::api::Guest;
use crate::bindings::golem::rpc::types::Uri;

use crate::bindings::timeline::core::api::WorkerDetails;

use crate::bindings::timeline::core_stub::stub_core;

use conversions::Conversion;
use timeline::event_predicate::EventColumnName as DslEventColumnName;
use timeline::timeline_node_worker::TimeLineNodeWorkerInput as DslTimeLineNodeWorkerInput;
use timeline::timeline_node_worker::TimeLineWorkerIdPrefix as DslTimeLineWorkerIdPrefix;
use timeline::timeline_op::TimeLineOp as DslTimeLineOp;

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;
mod builder;
mod conversions;
struct Component;

impl Guest for Component {
    fn run(
        core_template_id: String,
        event_processor_component_id: String,
        timeline_processor_component_id: String,
    ) -> Result<WorkerDetails, String> {
        let uri = Uri { value: format!("worker://{core_template_id}/{}", "initialize-timeline") };

        let core = stub_core::Api::new(&uri);
        let simple_timeline = DslTimeLineOp::Not(
            DslTimeLineNodeWorkerInput {
                worker_id_prefix: DslTimeLineWorkerIdPrefix("not".to_string()),
                component_id: timeline_processor_component_id,
            },
            Box::new(DslTimeLineOp::TlLatestEventToState(
                DslTimeLineNodeWorkerInput {
                    worker_id_prefix: DslTimeLineWorkerIdPrefix("cdn-change".to_string()),
                    component_id: event_processor_component_id,
                },
                DslEventColumnName("cdnChange".to_string()),
            )),
        );

        match core.initialize_timeline(&simple_timeline.to_wit()) {
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
