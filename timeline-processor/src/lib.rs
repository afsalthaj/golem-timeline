use crate::bindings::exports::timeline::timeline_processor::api::{EventValue, Guest, TypedTimelineResultWorker};
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
//use crate::bindings::timeline::timeline_processor_stub::stub_timeline_processor;
use crate::bindings::golem::rpc::types::Uri;

mod bindings;

struct Component;

impl Guest for Component {
    fn initialize_equal(child_url: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {

        let uri = Uri {
            value: format!("worker://some_template/{}", "some_worker"),
        };

        let core = stub_event_processor::Api::new(&uri);

        core.tl_has_existed(
           1
        )?;

        let uri = Uri {
            value: format!("worker://some_template/{}", "some_worker"),
        };

        // let core = stub_timeline_processor::Api::new(&uri);
        //
        // core.initialize_equal(
        //     &child_url,
        //     &current_worker_id,
        //     &event_value
        // )?;

        Ok("Successfully initiated the worker to compute equals".to_string())
    }
}