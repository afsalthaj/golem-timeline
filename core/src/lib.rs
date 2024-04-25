mod bindings;

pub mod conversions;

mod internals;

use crate::bindings::exports::timeline::core::api::Guest;
use crate::bindings::exports::timeline::core::api::*;
use timeline::timeline_op::{TimeLineOp as CoreTimeLineOp};
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
use crate::bindings::golem::rpc::types::Uri;
use conversions::Conversion;

struct Component;

impl Guest for Component {
    fn initialize_timeline(timeline: TimelineOp) -> Result<String, String> {
       let timeline: CoreTimeLineOp = CoreTimeLineOp::from_wit(timeline);

        match timeline {
            CoreTimeLineOp::Leaf(server) => {
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

           CoreTimeLineOp::EqualTo(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::GreaterThan(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::GreaterThanOrEqual(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::LessThan(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::LessThanOrEqual(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::And(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::Or(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::Not(_, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::TlHasExisted(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::TlHasExistedWithin(_, _, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::TlLatestEventToState(_, _, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::TlDurationWhere(_, _) => Err("Not implemented".to_string()),
           CoreTimeLineOp::TlDurationInCurState(_, _) => Err("Not implemented".to_string()),
       }
    }
}


