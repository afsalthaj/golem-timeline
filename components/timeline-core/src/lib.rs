mod bindings;
mod builder;
mod conversions;

use crate::bindings::exports::timeline::core_exports::api::{
    Guest, TimelineOp, TypedTimelineResultWorker, WorkerDetails,
};
use crate::bindings::golem::rpc::types::Uri;
use crate::conversions::Conversion;
use timeline_lib::TimeLineOp as CoreTimeLineOp;
use timeline_lib::{
    TimeLineResultWorker, TimeLineWorkerIdPrefix, TimeLineWorkerName, TypedTimeLineResultWorker,
};

use crate::bindings::timeline::event_processor_client::event_processor_client;
use crate::bindings::timeline::timeline_processor_client::timeline_processor_client;
use crate::bindings::timeline::timeline_processor_client::timeline_processor_client::GolemRpcUri;
use golem_rust::bindings::golem::api::host::{resolve_worker_id, worker_uri};
use std::cell::RefCell;
use uuid::Uuid;

/// This is one of any number of data types that our application
/// uses. Golem will take care to persist all application state,
/// whether that state is local to a function being executed or
/// global across the entire program.
struct State {
    total: u64,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        total: 0,
    });
}

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
                    let (component_name, worker_id_prefix) = worker
                        .clone()
                        .map(|w| (w.component_name, w.worker_name_prefix))
                        .ok_or("No worker id for timeline found")?;

                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_name =
                        TimeLineWorkerName(format!("{}-tleq-{}", worker_id_prefix, uuid));

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let timeline_processor_api =
                        timeline_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(left, event_processors)?;

                    // Culprit
                    timeline_processor_api
                        .blocking_initialize_equal(&child_worker.to_wit(), &right.to_wit())?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::equal_to({
                        TimeLineResultWorker {
                            component_name: component_name.clone(),
                            worker_name: worker_name,
                        }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::GreaterThan(worker, timeline, value) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_name = format!("{}-tlgt-{}", worker_id_prefix, uuid);

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.as_str())
                            .expect("Failed to resolve worker ID");

                    let target_uri: golem_rust::bindings::golem::api::host::Uri =
                        worker_uri(&worker_id);

                    let timeline_processor_api =
                        timeline_processor_client::Api::new(&GolemRpcUri {
                            value: target_uri.value,
                        });

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(timeline, event_processors)?;

                    // We initialise this node into some worker along with the information about child worker that it needs to fetch the result from
                    timeline_processor_api.blocking_initialize_greater_than(
                        &child_worker.to_wit(),
                        &value.to_wit(),
                    )?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::greater_than({
                        TimeLineResultWorker {
                            component_name: component_name.clone(),
                            worker_name: TimeLineWorkerName(worker_name),
                        }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::GreaterThanOrEqual(worker, timeline, value) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_name = format!("{}-tlgteq-{}", worker_id_prefix, uuid);

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let timeline_processor_api =
                        timeline_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(timeline, event_processors)?;

                    // We initialise this node into some worker along with the information about child worker that it needs to fetch the result from
                    timeline_processor_api.blocking_initialize_greater_than_or_equal_to(
                        &child_worker.to_wit(),
                        &value.to_wit(),
                    )?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker =
                        TypedTimeLineResultWorker::greater_than_or_equal_to({
                            TimeLineResultWorker {
                                component_name: component_name.clone(),
                                worker_name: TimeLineWorkerName(worker_name),
                            }
                        });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::LessThan(worker, timeline, event_value) => {
                    let (component_id, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_name =
                        TimeLineWorkerName(format!("{}-tllt-{}", worker_id_prefix, uuid));

                    let worker_id =
                        resolve_worker_id(component_id.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let timeline_processor_api =
                        timeline_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(timeline, event_processors)?;

                    // We initialise this node into some worker along with the information about child worker that it needs to fetch the result from
                    timeline_processor_api.blocking_initialize_less_than(
                        &child_worker.to_wit(),
                        &event_value.to_wit(),
                    )?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::greater_than({
                        TimeLineResultWorker { component_name: component_id.clone(), worker_name }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::LessThanOrEqual(worker, timeline, event_value) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    // Connecting to the worker that should compute equal
                    let worker_name =
                        TimeLineWorkerName(format!("{}-tllteq-{}", worker_id_prefix, uuid));

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let timeline_processor_api =
                        timeline_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    // Specifying the worker the timeline-equal worker should fetch the results from to compare with a constant
                    let child_worker = go(timeline, event_processors)?;

                    timeline_processor_api.blocking_initialize_less_than_or_equal_to(
                        &child_worker.to_wit(),
                        &event_value.to_wit(),
                    )?;

                    // The worker in which the comparison with a constant actually executes
                    let typed_timeline_result_worker =
                        TypedTimeLineResultWorker::less_than_or_equal_to({
                            TimeLineResultWorker {
                                component_name: component_name.clone(),
                                worker_name,
                            }
                        });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::And(worker, left, right) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    let worker_name =
                        TimeLineWorkerName(format!("{}-tl-and-{}", worker_id_prefix, uuid));

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let core =
                        timeline_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    let left_worker = go(left, event_processors)?;
                    let right_worker = go(right, event_processors)?;

                    // We initialise this node into some worker along with the information about children workers that it needs to fetch the result from and apply and logic
                    core.blocking_initialize_and(&left_worker.to_wit(), &right_worker.to_wit())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::and({
                        TimeLineResultWorker { component_name: component_name.clone(), worker_name }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::Or(worker, left, right) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    let worker_name =
                        TimeLineWorkerName(format!("{}-tl-and-{}", worker_id_prefix, uuid));

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let core =
                        timeline_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    let left_worker = go(left, event_processors)?;
                    let right_worker = go(right, event_processors)?;

                    // We initialise this node into some worker along with the information about children workers that it needs to fetch the result from and apply or logic
                    core.blocking_initialize_or(&left_worker.to_wit(), &right_worker.to_wit())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::or({
                        TimeLineResultWorker { component_name: component_name.clone(), worker_name }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::Not(worker, timeline) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    let worker_name =
                        TimeLineWorkerName(format!("{}-tl-not-{}", worker_id_prefix, uuid));

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let core =
                        timeline_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    let child_worker = go(timeline, event_processors)?;

                    // We initialise this node into some worker along with the information about child worker that it needs to fetch the result from and apply not logic
                    core.blocking_initialize_not(&child_worker.to_wit())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::not({
                        TimeLineResultWorker { component_name: component_name.clone(), worker_name }
                    });

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::TlHasExisted(worker, predicate) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    let worker_name =
                        TimeLineWorkerName(format!("{}-tlhe-{}", worker_id_prefix, uuid));

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let core = event_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    core.blocking_initialize_tl_has_existed(&predicate.to_wit())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker = TypedTimeLineResultWorker::tl_has_existed({
                        TimeLineResultWorker { component_name: component_name.clone(), worker_name }
                    });

                    event_processors.push(typed_timeline_result_worker.to_wit());

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::TlHasExistedWithin(worker, predicate, within) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let uuid = Uuid::new_v4();

                    let worker_name =
                        TimeLineWorkerName(format!("{}-tlhew-{}", worker_id_prefix, uuid));

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let core = event_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    // The result of this node will be available in this worker
                    core.blocking_initialize_tl_has_existed_within(&predicate.to_wit(), *within)?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker =
                        TypedTimeLineResultWorker::tl_has_existed_within({
                            TimeLineResultWorker {
                                component_name: component_name.clone(),
                                worker_name,
                            }
                        });

                    event_processors.push(typed_timeline_result_worker.to_wit());

                    Ok(typed_timeline_result_worker)
                }
                CoreTimeLineOp::TlLatestEventToState(worker, event_column_name) => {
                    let (component_name, worker_id_prefix) = worker.clone().map_or(
                        ("default".to_string(), TimeLineWorkerIdPrefix("default".to_string())),
                        |w| (w.component_name, w.worker_name_prefix),
                    );
                    let worker_name = TimeLineWorkerName(format!(
                        "{}-le2s-{}",
                        worker_id_prefix, event_column_name
                    ));

                    let worker_id =
                        resolve_worker_id(component_name.as_str(), worker_name.0.as_str())
                            .expect("Failed to resolve worker ID");

                    let uri = worker_uri(&worker_id);

                    let core = event_processor_client::Api::new(&GolemRpcUri { value: uri.value });

                    core.blocking_initialize_latest_event_state(event_column_name.0.as_str())?;

                    // The result of this node will be available in this worker
                    let typed_timeline_result_worker =
                        TypedTimeLineResultWorker::tl_event_to_latest_state({
                            TimeLineResultWorker {
                                component_name: component_name.clone(),
                                worker_name,
                            }
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

    fn hello_world() -> String {
        "afsal".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
