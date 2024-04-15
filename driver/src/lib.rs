use crate::bindings::exports::timeline::driver::api::Guest;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::core::api::TimelineNode::Leaf;
use crate::bindings::timeline::core::api::TimelineOp;
use crate::bindings::timeline::core::api::WorkerId;
use crate::bindings::timeline::core_stub::stub_core;

mod bindings;
struct Component;

impl Guest for Component {
    fn run(value: String) {
        let template_id = value;

        let uri = Uri {
            value: format!("worker://{template_id}/{}", "raw-events"),
        };

        let core = stub_core::Api::new(&uri);
        let timeline_op = TimelineOp {
            nodes: vec![Leaf(WorkerId {
                name : "leaf".to_string(),
            })],
        };

        core.initialize_timeline(&timeline_op);

        dbg!("Timeline initialized");
    }
}
