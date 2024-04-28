

use uuid::Uuid;

use conversions::Conversion;
use timeline::timeline_node_worker::{TimeLineResultWorker, TimeLineWorkerId, TypedTimeLineResultWorker};
use timeline::timeline_op::TimeLineOp as CoreTimeLineOp;

use crate::bindings::exports::timeline::core::api::Guest;
use crate::bindings::exports::timeline::core::api::*;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
use crate::bindings::timeline::timeline_processor_stub::stub_timeline_processor;

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;

pub mod conversions;

struct Component;

impl Guest for Component {
    fn initialize_timeline(timeline: TimelineOp) -> Result<ExecutionResultWorker, String> {
        let timeline: CoreTimeLineOp = CoreTimeLineOp::from_wit(timeline);

        fn go(core_time_line_op: &CoreTimeLineOp) -> Result<TypedTimeLineResultWorker, String> {
            match core_time_line_op {
                CoreTimeLineOp::EqualTo(worker, left, right) => {
                    let template_id = &worker.template_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();
                    let worker_id = TimeLineWorkerId(format!("{}-tleq-{}", worker_id_prefix, uuid.to_string()));

                    let uri = Uri {
                        value: format!("worker://{template_id}/{}", &worker_id),
                    };

                    let timeline_processor_api = stub_timeline_processor::Api::new(&uri);

                    timeline_processor_api.initialize_equal()


                }
                CoreTimeLineOp::GreaterThan(_, _, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::GreaterThanOrEqual(_, _, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::LessThan(_, _, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::LessThanOrEqual(_, _, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::And(_, _, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::Or(_, _, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::Not(_, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::TlHasExisted(worker, predicate) => {
                    let template_id = &worker.template_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    let worker_id = TimeLineWorkerId(format!("{}-tlhe-{}", worker_id_prefix, uuid.to_string()));

                    let uri = Uri {
                        value: format!("worker://{template_id}/{}", &worker_id),
                    };

                    let core = stub_event_processor::Api::new(&uri);

                    core.initialize_tl_has_existed(
                        &stub_event_processor::WorkerId { name: worker_id.0.clone() },
                        &predicate.to_wit(),
                    )?;

                    let typed_timeline_result_worker = TypedTimeLineResultWorker::tl_has_existed({
                        TimeLineResultWorker {
                            template_id: template_id.clone(),
                            worker_id
                        }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::TlHasExistedWithin(worker, predicate, within) => {
                    let template_id = &worker.template_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    let worker_id = format!("{}-tlhew-{}", worker_id_prefix, uuid.to_string());

                    let uri = Uri {
                        value: format!("worker://{template_id}/{}", worker_id),
                    };

                    let core = stub_event_processor::Api::new(&uri);

                    core.initialize_tl_has_existed_within(
                        &stub_event_processor::WorkerId { name: worker_id.clone() },
                        &predicate.to_wit(),
                        *within
                    )?;

                    Ok(worker_id)
                }
                CoreTimeLineOp::TlLatestEventToState(worker, event_column_name) => {
                    let template_id = &worker.template_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let worker_id = format!("{}-le2s-{}", worker_id_prefix, event_column_name);

                    let uri = Uri {
                        value: format!("worker://{template_id}/{}", worker_id),
                    };

                    let core = stub_event_processor::Api::new(&uri);

                    core.initialize_latest_event_state(
                        &stub_event_processor::WorkerId { name: worker_id.clone() },
                        event_column_name.0.as_str(),
                    )?;

                    Ok(worker_id)
                }
                CoreTimeLineOp::TlDurationWhere(_, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::TlDurationInCurState(_, _) => Err("Not implemented".to_string()),
            }
        }

        go(&timeline)
    }
}
