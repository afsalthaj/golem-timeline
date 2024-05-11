use crate::bindings::exports::timeline::driver::api::Guest;
use crate::bindings::golem::rpc::types::Uri;

use crate::bindings::timeline::core::api::WorkerDetails;

use crate::bindings::timeline::core_stub::stub_core;

use conversions::Conversion;
use timeline::event_predicate::EventColumnName as DslEventColumnName;
use timeline::timeline_op::SimpleGolemTimelineDsl;
use timeline::timeline_op::TimeLineOpBuilder;

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;

mod builder;
mod conversions;
struct Component;

impl Guest for Component {
    fn run(
        core_component_id: String,
        leaf_node_component_id: String,
        derived_node_component_id: String,
    ) -> Result<WorkerDetails, String> {
        let uri = Uri { value: format!("worker://{core_component_id}/{}", "initialize-timeline") };

        let core = stub_core::Api::new(&uri);

        let dsl = SimpleGolemTimelineDsl::new(
            "cirr".to_string(),
            leaf_node_component_id,
            derived_node_component_id,
        );

        let simple_timeline =
            dsl.not(dsl.latest_event_to_state(DslEventColumnName("playerStateChange".to_string())));

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
