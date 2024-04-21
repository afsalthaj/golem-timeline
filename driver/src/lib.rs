use crate::bindings::exports::timeline::driver::api::Guest;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::core::api::TimelineNode::Leaf;
use crate::bindings::timeline::core::api::TimelineOp;
use crate::bindings::timeline::core::api::{Server};
use crate::bindings::timeline::core_stub::stub_core;

mod bindings;
struct Component;

impl Guest for Component {
    fn run(core_template_id: String, leaf_template_id: String, event_to_state_tempalte_id: String) -> Result<String, String> {

        let uri = Uri {
            value: format!("worker://{core_template_id}/{}", "initialize-timeline"),
        };

        let core = stub_core::Api::new(&uri);
        let timeline_op = TimelineOp {
            nodes: vec![Leaf(Server {
                template_id : leaf_template_id.to_string(),
                worker_id: "event-processor-worker".to_string(),

            })],
        };

        match core.initialize_timeline(&timeline_op) {
            Ok(result) => {
                dbg!("Driver Log: Timeline initialized");
                Ok(result)
            },
            Err(error) => {
                dbg!("Driver Log: Error initializing timeline");
                Err(error)
            },
        }

    }
}
