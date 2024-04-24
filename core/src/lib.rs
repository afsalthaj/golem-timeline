mod bindings;

pub mod event_predicate;
pub mod event_timeline;
pub mod state_dynamic_timeline_point;
pub mod state_dynamics_timeline;
pub mod timeline_op;

mod internals;

use crate::bindings::exports::timeline::core::api::Guest;
use crate::bindings::exports::timeline::core::api::*;
use crate::timeline_op::{TimeLineOp as CoreTimeLineOp, TimeLineOp};
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
use crate::bindings::golem::rpc::types::Uri;



struct Component;

impl Guest for Component {
    fn initialize_timeline(timeline: TimelineOp) -> Result<String, String> {
       let timeline: CoreTimeLineOp = timeline.into();

        match timeline {
           TimeLineOp::Leaf(server) => {
               let template_id = server.template_id;
               let worker_id = server.worker_id;

               let uri = Uri {
                   value: format!("worker://{template_id}/{}", worker_id.clone()),
               };

               let core = stub_event_processor::Api::new(&uri);

               core.initialize(&stub_event_processor::WorkerId{
                     name: worker_id,
                })
           },

           TimeLineOp::EqualTo(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::GreaterThan(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::GreaterThanOrEqual(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::LessThan(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::LessThanOrEqual(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::And(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::Or(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::Not(_, _) => Err("Not implemented".to_string()),
           TimeLineOp::TlHasExisted(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::TlHasExistedWithin(_, _, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::TlLatestEventToState(_, _, _) => Err("Not implemented".to_string()),
           TimeLineOp::TlDurationWhere(_, _) => Err("Not implemented".to_string()),
           TimeLineOp::TlDurationInCurState(_, _) => Err("Not implemented".to_string()),
       }
    }
}


