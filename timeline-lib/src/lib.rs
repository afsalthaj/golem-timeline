mod bindings;

pub mod aligned_state_dynamic_timeline;
pub mod backend;
pub mod boundaries;
pub mod event_predicate;
pub mod event_record;
pub mod event_stream;
pub mod event_timeline;
pub mod state_dynamic_timeline_point;
pub mod state_dynamics_timeline;
pub mod timeline;
pub mod timeline_execution;
pub mod timeline_op;
pub mod value;
pub mod worker_timeline;
pub mod worker_timeline_data;
pub mod zip_result;

use golem_wasm_rpc::Value;
use crate::bindings::exports::golem::timeline::api::Guest;
use crate::bindings::exports::golem::timeline::api::*;
use golem_wasm_rpc::*;

struct Component;

impl Guest for Component {
    fn get_timelines() -> wit_bindgen::rt::vec::Vec<wit_bindgen::rt::string::String> {
        todo!()
    }

    fn initialize_timeline(timeline_op: TimelineOp) -> () {
        dbg!("Initializing worker timeline");
    }
}


