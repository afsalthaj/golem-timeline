mod bindings;

pub mod aligned_state_dynamic_timeline;
pub mod backend;
pub mod boundaries;
pub mod event_predicate;
pub mod event_timeline;
pub mod state_dynamic_timeline_point;
pub mod state_dynamics_timeline;
pub mod timeline;
pub mod timeline_execution;
pub mod timeline_op;
pub mod worker_timeline;
pub mod worker_timeline_data;
pub mod zip_result;

use crate::bindings::exports::timeline::core::api::Guest;
use crate::bindings::exports::timeline::core::api::*;

struct Component;

impl Guest for Component {
    fn initialize_timeline(timeline: TimelineOp) {
        dbg!(timeline);
       //let timeline: crate::timeline_op::TimeLineOp = timeline.into();

       // dbg!(timeline.to_string());

        dbg!("Excuted timeline remotely");
    }
}


