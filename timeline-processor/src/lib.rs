use crate::bindings::exports::timeline::timeline_processor::api::{EventValue, Guest, WorkerId};

mod bindings;

struct Component;

impl Guest for Component {
    fn initialize_equal(child_url: WorkerId, current_worker_id: WorkerId, event_value: EventValue) -> Result<String, String> {
        todo!()
    }
}