use crate::bindings::exports::timeline::timeline_processor::api::{EventValue, Guest, WorkerId};
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
use crate::bindings::golem::rpc::types::Uri;

mod bindings;

struct Component;

impl Guest for Component {
    fn initialize_equal(child_url: WorkerId, current_worker_id: WorkerId, event_value: EventValue) -> Result<String, String> {



        let uri = Uri {
            value: format!("worker://some_template/{}", "some_worker"),
        };

        let core = stub_event_processor::Api::new(&uri);

        core.tl_has_existed(
           1
        )?;

        Ok("".to_string())
    }
}