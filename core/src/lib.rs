use uuid::Uuid;

use conversions::Conversion;
use timeline::timeline_node_worker::{
    TimeLineResultWorker, TimeLineWorkerId, TypedTimeLineResultWorker,
};
use timeline::timeline_op::TimeLineOp as CoreTimeLineOp;

use crate::bindings::exports::timeline::core::api::TimelineOp;
use crate::bindings::exports::timeline::core::api::{
    Guest, TypedTimelineResultWorker, WorkerDetails,
};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
use crate::bindings::timeline::timeline_processor_stub::stub_timeline_processor;

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;

pub mod conversions;

pub mod builder;

struct Component;

impl Guest for Component {
    fn initialize_timeline(timeline: TimelineOp) -> Result<WorkerDetails, String> {
        let timeline: CoreTimeLineOp = CoreTimeLineOp::from_wit(timeline);
        let mut event_processor_workers = vec![];

        fn go(
            core_time_line_op: &CoreTimeLineOp,
            event_processors: &mut Vec<TypedTimelineResultWorker>,
        ) -> Result<TypedTimeLineResultWorker, String> {
            match core_time_line_op {
                CoreTimeLineOp::EqualTo(worker, left, right) => {
                    let component_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_id =
                        TimeLineWorkerId(format!("{}-tleq-{}", worker_id_prefix, uuid.to_string()));

                    let uri = Uri { value: format!("worker://{component_id}/{}", &worker_id) };

                    let timeline_processor_api = stub_timeline_processor::Api::new(&uri);

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(left, event_processors)?;

                    timeline_processor_api
                        .initialize_equal(&child_worker.to_wit(), &right.to_wit())?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::equal_to({
                        TimeLineResultWorker { component_id: component_id.clone(), worker_id }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::GreaterThan(worker, timeline, value) => {
                    let component_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_id =
                        TimeLineWorkerId(format!("{}-tlgt-{}", worker_id_prefix, uuid.to_string()));

                    let uri = Uri { value: format!("worker://{component_id}/{}", &worker_id) };

                    let timeline_processor_api = stub_timeline_processor::Api::new(&uri);

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(timeline, event_processors)?;

                    // We initialise this node into some worker along with the information about child worker that it needs to fetch the result from
                    timeline_processor_api
                        .initialize_greater_than(&child_worker.to_wit(), &value.to_wit())?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::greater_than({
                        TimeLineResultWorker { component_id: component_id.clone(), worker_id }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::GreaterThanOrEqual(worker, timeline, value) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_id = TimeLineWorkerId(format!(
                        "{}-tlgteq-{}",
                        worker_id_prefix,
                        uuid.to_string()
                    ));

                    let uri = Uri { value: format!("worker://{template_id}/{}", &worker_id) };

                    let timeline_processor_api = stub_timeline_processor::Api::new(&uri);

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(timeline, event_processors)?;

                    // We initialise this node into some worker along with the information about child worker that it needs to fetch the result from
                    timeline_processor_api.initialize_greater_than_or_equal_to(
                        &child_worker.to_wit(),
                        &value.to_wit(),
                    )?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker =
                        TypedTimeLineResultWorker::greater_than_or_equal_to({
                            TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                        });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::LessThan(worker, timeline, event_value) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_id =
                        TimeLineWorkerId(format!("{}-tllt-{}", worker_id_prefix, uuid.to_string()));

                    let uri = Uri { value: format!("worker://{template_id}/{}", &worker_id) };

                    let timeline_processor_api = stub_timeline_processor::Api::new(&uri);

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(timeline, event_processors)?;

                    // We initialise this node into some worker along with the information about child worker that it needs to fetch the result from
                    timeline_processor_api
                        .initialize_less_than(&child_worker.to_wit(), &event_value.to_wit())?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::greater_than({
                        TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::LessThanOrEqual(worker, timeline, event_value) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_id = TimeLineWorkerId(format!(
                        "{}-tllteq-{}",
                        worker_id_prefix,
                        uuid.to_string()
                    ));

                    let uri = Uri { value: format!("worker://{template_id}/{}", &worker_id) };

                    let timeline_processor_api = stub_timeline_processor::Api::new(&uri);

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(timeline, event_processors)?;

                    timeline_processor_api.initialize_less_than_or_equal_to(
                        &child_worker.to_wit(),
                        &event_value.to_wit(),
                    )?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker =
                        TypedTimeLineResultWorker::less_than_or_equal_to({
                            TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                        });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::And(worker, left, right) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    let worker_id = TimeLineWorkerId(format!(
                        "{}-tl-and-{}",
                        worker_id_prefix,
                        uuid.to_string()
                    ));

                    let uri = Uri { value: format!("worker://{template_id}/{}", &worker_id) };

                    let core = stub_timeline_processor::Api::new(&uri);

                    let left_worker = go(left, event_processors)?;
                    let right_worker = go(right, event_processors)?;

                    // We initialise this node into some worker along with the information about children workers that it needs to fetch the result from and apply and logic
                    core.initialize_and(&left_worker.to_wit(), &right_worker.to_wit())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::and({
                        TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::Or(worker, left, right) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    let worker_id = TimeLineWorkerId(format!(
                        "{}-tl-and-{}",
                        worker_id_prefix,
                        uuid.to_string()
                    ));

                    let uri = Uri { value: format!("worker://{template_id}/{}", &worker_id) };

                    let core = stub_timeline_processor::Api::new(&uri);

                    let left_worker = go(left, event_processors)?;
                    let right_worker = go(right, event_processors)?;

                    // We initialise this node into some worker along with the information about children workers that it needs to fetch the result from and apply or logic
                    core.initialize_or(&left_worker.to_wit(), &right_worker.to_wit())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::or({
                        TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::Not(worker, timeline) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    let worker_id = TimeLineWorkerId(format!(
                        "{}-tl-not-{}",
                        worker_id_prefix,
                        uuid.to_string()
                    ));

                    let uri = Uri { value: format!("worker://{template_id}/{}", &worker_id) };

                    let core = stub_timeline_processor::Api::new(&uri);

                    let child_worker = go(timeline, event_processors)?;

                    // We initialise this node into some worker along with the information about child worker that it needs to fetch the result from and apply not logic
                    core.initialize_not(&child_worker.to_wit())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::not({
                        TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::TlHasExisted(worker, predicate) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    let worker_id =
                        TimeLineWorkerId(format!("{}-tlhe-{}", worker_id_prefix, uuid.to_string()));

                    let uri = Uri { value: format!("worker://{template_id}/{}", &worker_id) };

                    let core = stub_event_processor::Api::new(&uri);

                    core.initialize_tl_has_existed(&predicate.to_wit())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::tl_has_existed({
                        TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                    });

                    event_processors.push(typed_timeline_result_worker.to_wit());

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::TlHasExistedWithin(worker, predicate, within) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let uuid = Uuid::new_v4();

                    let worker_id = TimeLineWorkerId(format!(
                        "{}-tlhew-{}",
                        worker_id_prefix,
                        uuid.to_string()
                    ));

                    let uri = Uri { value: format!("worker://{template_id}/{}", worker_id) };

                    let core = stub_event_processor::Api::new(&uri);

                    // The result of this node will be available in this worker
                    core.initialize_tl_has_existed_within(&predicate.to_wit(), *within)?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker =
                        TypedTimeLineResultWorker::tl_has_existed_within({
                            TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                        });

                    event_processors.push(typed_timeline_result_worker.to_wit());

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::TlLatestEventToState(worker, event_column_name) => {
                    let template_id = &worker.component_id;
                    let worker_id_prefix = &worker.worker_id_prefix;
                    let worker_id = TimeLineWorkerId(format!(
                        "{}-le2s-{}",
                        worker_id_prefix, event_column_name
                    ));

                    let uri = Uri { value: format!("worker://{template_id}/{}", worker_id) };

                    let core = stub_event_processor::Api::new(&uri);

                    core.initialize_latest_event_state(event_column_name.0.as_str())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker =
                        TypedTimeLineResultWorker::tl_has_existed_within({
                            TimeLineResultWorker { component_id: template_id.clone(), worker_id }
                        });

                    event_processors.push(typed_timeline_result_worker.to_wit());

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::TlDurationWhere(_, _) => Err("Not implemented".to_string()),
                CoreTimeLineOp::TlDurationInCurState(_, _) => Err("Not implemented".to_string()),
            }
        }

        let result_worker = go(&timeline, &mut event_processor_workers)
            .map(|typed_worker_info| typed_worker_info.to_wit())?;

        Ok(WorkerDetails { result_worker, event_processor_workers })
    }
}
