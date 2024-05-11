use crate::bindings::exports::timeline::driver::api::Guest;
use crate::bindings::golem::rpc::types::Uri;

use crate::bindings::timeline::core::api::WorkerDetails;

use crate::bindings::timeline::core_stub::stub_core;

use conversions::Conversion;
use timeline::event_predicate::EventColumnName as DslEventColumnName;
use timeline::timeline_op::{tl_latest_event_to_state, tl_not};

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;

mod builder;
mod conversions;
struct Component;

impl Guest for Component {
    fn run(
        core_component_id: String,
        even_processor_id: String,
        timeline_processor_id: String,
    ) -> Result<WorkerDetails, String> {
        let uri = Uri { value: format!("worker://{core_component_id}/{}", "initialize-timeline") };

        let core = stub_core::Api::new(&uri);

        let simple_timeline =
            tl_not(tl_latest_event_to_state(DslEventColumnName("playerStateChange".to_string())))
                .with_worker_details("cirr".to_string(), even_processor_id, timeline_processor_id);

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
