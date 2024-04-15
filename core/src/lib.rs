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
use crate::timeline_op::TimeLineOp as CoreTimeLineOp;

struct Component;

impl Guest for Component {
    fn initialize_timeline(timeline: TimelineOp) {
       let timeline: CoreTimeLineOp = timeline.into();

       match timeline {
           CoreTimeLineOp::Leaf() => {}
           CoreTimeLineOp::EqualTo(_, _) => {}
           CoreTimeLineOp::GreaterThan(_, _) => {}
           CoreTimeLineOp::GreaterThanOrEqual(_, _) => {}
           CoreTimeLineOp::LessThan(_, _) => {}
           CoreTimeLineOp::LessThanOrEqual(_, _) => {}
           CoreTimeLineOp::And(_, _) => {}
           CoreTimeLineOp::Or(_, _) => {}
           CoreTimeLineOp::Not(_) => {}
           CoreTimeLineOp::TlHasExisted(_, _) => {}
           CoreTimeLineOp::TlHasExistedWithin(_, _, _) => {}
           CoreTimeLineOp::TlLatestEventToState(_, _) => {}
           CoreTimeLineOp::TlDurationWhere(_, _) => {}
           CoreTimeLineOp::TlDurationInCurState(_, _) => {}
       }

        dbg!("Excuted timeline remotely");
    }
}


