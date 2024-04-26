

use uuid::Uuid;

use conversions::Conversion;
use timeline::timeline_op::TimeLineOp as CoreTimeLineOp;

use crate::bindings::exports::timeline::core::api::Guest;
use crate::bindings::exports::timeline::core::api::*;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::event_processor_stub::stub_event_processor;

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;

pub mod conversions;

struct Component;

impl Guest for Component {
    fn initialize_timeline(timeline: TimelineOp) -> Result<String, String> {
        let timeline: CoreTimeLineOp = CoreTimeLineOp::from_wit(timeline);

        match timeline {
            CoreTimeLineOp::EqualTo(_, _, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::GreaterThan(_, _, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::GreaterThanOrEqual(_, _, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::LessThan(_, _, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::LessThanOrEqual(_, _, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::And(_, _, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::Or(_, _, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::Not(_, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::TlHasExisted(worker, predicate) => {
                let template_id = worker.template_id;
                let worker_id_prefix = worker.worker_id;
                let uuid = Uuid::new_v4();

                let worker_id = format!("{}-tlhe-{}", worker_id_prefix, uuid.to_string());

                let uri = Uri {
                    value: format!("worker://{template_id}/{}", worker_id.clone()),
                };

                let core = stub_event_processor::Api::new(&uri);

                core.initialize_tl_has_existed(
                    &stub_event_processor::WorkerId { name: worker_id },
                    &predicate.to_wit(),
                )
            }
            CoreTimeLineOp::TlHasExistedWithin(_, _, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::TlLatestEventToState(worker, event_column_name) => {
                let template_id = worker.template_id;
                let worker_id_prefix = worker.worker_id;
                let worker_id = format!("{}-le2s-{}", worker_id_prefix, event_column_name);

                let uri = Uri {
                    value: format!("worker://{template_id}/{}", worker_id.clone()),
                };

                let core = stub_event_processor::Api::new(&uri);

                core.initialize_latest_event_state(
                    &stub_event_processor::WorkerId { name: worker_id },
                    event_column_name.0.as_str(),
                )
            }
            CoreTimeLineOp::TlDurationWhere(_, _) => Err("Not implemented".to_string()),
            CoreTimeLineOp::TlDurationInCurState(_, _) => Err("Not implemented".to_string()),
        }
    }
}
