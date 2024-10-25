#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
pub struct FutureInitializeEqualResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeGreaterThanResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeGreaterThanOrEqualToResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeLessThanResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeLessThanOrEqualToResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeAndResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeOrResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeNotResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetTimelineResultResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::Guest
for Component {
    type Api = crate::Api;
    type FutureInitializeEqualResult = crate::FutureInitializeEqualResult;
    type FutureInitializeGreaterThanResult = crate::FutureInitializeGreaterThanResult;
    type FutureInitializeGreaterThanOrEqualToResult = crate::FutureInitializeGreaterThanOrEqualToResult;
    type FutureInitializeLessThanResult = crate::FutureInitializeLessThanResult;
    type FutureInitializeLessThanOrEqualToResult = crate::FutureInitializeLessThanOrEqualToResult;
    type FutureInitializeAndResult = crate::FutureInitializeAndResult;
    type FutureInitializeOrResult = crate::FutureInitializeOrResult;
    type FutureInitializeNotResult = crate::FutureInitializeNotResult;
    type FutureGetTimelineResultResult = crate::FutureGetTimelineResultResult;
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureInitializeEqualResult
for FutureInitializeEqualResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{initialize-equal}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureInitializeGreaterThanResult
for FutureInitializeGreaterThanResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{initialize-greater-than}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureInitializeGreaterThanOrEqualToResult
for FutureInitializeGreaterThanOrEqualToResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{initialize-greater-than-or-equal-to}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureInitializeLessThanResult
for FutureInitializeLessThanResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{initialize-less-than}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureInitializeLessThanOrEqualToResult
for FutureInitializeLessThanOrEqualToResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{initialize-less-than-or-equal-to}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureInitializeAndResult
for FutureInitializeAndResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{initialize-and}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureInitializeOrResult
for FutureInitializeOrResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{initialize-or}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureInitializeNotResult
for FutureInitializeNotResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{initialize-not}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestFutureGetTimelineResultResult
for FutureGetTimelineResultResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(
        &self,
    ) -> Option<
        Result<
            crate::bindings::timeline::timeline_processor::api::TimelineResult,
            String,
        >,
    > {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:timeline-processor/api.{get-timeline-result}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok({
                                let record = ok_value.expect("result ok value not found");
                                crate::bindings::timeline::event_processor::api::TimelineResult {
                                    results: record
                                        .field(0usize)
                                        .expect("record field not found")
                                        .list_elements(|item| {
                                            let record = item;
                                            crate::bindings::timeline::event_processor::api::TimelineResultPoint {
                                                time_period: {
                                                    let record = record
                                                        .field(0usize)
                                                        .expect("record field not found");
                                                    crate::bindings::timeline::event_processor::api::TimePeriod {
                                                        t1: record
                                                            .field(0usize)
                                                            .expect("record field not found")
                                                            .u64()
                                                            .expect("u64 not found"),
                                                        t2: record
                                                            .field(1usize)
                                                            .expect("record field not found")
                                                            .option()
                                                            .expect("option not found")
                                                            .map(|inner| inner.u64().expect("u64 not found")),
                                                    }
                                                },
                                                value: {
                                                    let (case_idx, inner) = record
                                                        .field(1usize)
                                                        .expect("record field not found")
                                                        .variant()
                                                        .expect("variant not found");
                                                    match case_idx {
                                                        0u32 => {
                                                            crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                                                inner
                                                                    .expect("variant case not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            )
                                                        }
                                                        1u32 => {
                                                            crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                                                inner
                                                                    .expect("variant case not found")
                                                                    .s64()
                                                                    .expect("i64 not found"),
                                                            )
                                                        }
                                                        2u32 => {
                                                            crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                                                inner
                                                                    .expect("variant case not found")
                                                                    .f64()
                                                                    .expect("f64 not found"),
                                                            )
                                                        }
                                                        3u32 => {
                                                            crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                                                inner
                                                                    .expect("variant case not found")
                                                                    .bool()
                                                                    .expect("bool not found"),
                                                            )
                                                        }
                                                        _ => unreachable!("invalid variant case index"),
                                                    }
                                                },
                                            }
                                        })
                                        .expect("list not found"),
                                }
                            })
                        }
                        Err(err_value) => {
                            Err(
                                err_value
                                    .expect("result err value not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                    }
                })
            })
    }
}
impl crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::GuestApi
for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_initialize_equal(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{initialize-equal}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{initialize-equal}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn initialize_equal(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeEqualResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{initialize-equal}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeEqualResult::new(FutureInitializeEqualResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_greater_than(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{initialize-greater-than}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{initialize-greater-than}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn initialize_greater_than(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeGreaterThanResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{initialize-greater-than}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeGreaterThanResult::new(FutureInitializeGreaterThanResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_greater_than_or_equal_to(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{initialize-greater-than-or-equal-to}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{initialize-greater-than-or-equal-to}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn initialize_greater_than_or_equal_to(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeGreaterThanOrEqualToResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{initialize-greater-than-or-equal-to}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeGreaterThanOrEqualToResult::new(FutureInitializeGreaterThanOrEqualToResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_less_than(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{initialize-less-than}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{initialize-less-than}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn initialize_less_than(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeLessThanResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{initialize-less-than}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeLessThanResult::new(FutureInitializeLessThanResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_less_than_or_equal_to(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{initialize-less-than-or-equal-to}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{initialize-less-than-or-equal-to}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn initialize_less_than_or_equal_to(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeLessThanOrEqualToResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{initialize-less-than-or-equal-to}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &event_value {
                                crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(inner),
                                crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        ),
                ],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeLessThanOrEqualToResult::new(FutureInitializeLessThanOrEqualToResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_and(
        &self,
        child_worker1: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        child_worker2: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{initialize-and}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{initialize-and}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn initialize_and(
        &self,
        child_worker1: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        child_worker2: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeAndResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{initialize-and}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                ],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeAndResult::new(FutureInitializeAndResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_or(
        &self,
        child_worker1: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        child_worker2: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{initialize-or}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{initialize-or}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn initialize_or(
        &self,
        child_worker1: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        child_worker2: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeOrResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{initialize-or}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker1 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker2 {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                ],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeOrResult::new(FutureInitializeOrResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_not(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{initialize-not}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{initialize-not}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn initialize_not(
        &self,
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeNotResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{initialize-not}",
                &[
                    WitValue::builder()
                        .variant_fn(
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => 1u32,
                            },
                            match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    _,
                                ) => false,
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &child_worker {
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => 2u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                                crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline(
                                    inner,
                                ) => {
                                    case_builder
                                        .variant_fn(
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => 0u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => 1u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => 2u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => 3u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => 4u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => 5u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => 6u32,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => 7u32,
                                            },
                                            match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    _,
                                                ) => false,
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    _,
                                                ) => false,
                                            },
                                            |case_builder| match &inner {
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .string(&inner.worker_id)
                                                        .item()
                                                        .string(&inner.template_id)
                                                        .finish()
                                                }
                                            },
                                        )
                                }
                            },
                        ),
                ],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureInitializeNotResult::new(FutureInitializeNotResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_timeline_result(
        &self,
        t1: u64,
    ) -> Result<
        crate::bindings::timeline::timeline_processor::api::TimelineResult,
        String,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api.{get-timeline-result}",
                &[WitValue::builder().u64(t1)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:timeline-processor/api.{get-timeline-result}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok({
                        let record = ok_value.expect("result ok value not found");
                        crate::bindings::timeline::event_processor::api::TimelineResult {
                            results: record
                                .field(0usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    let record = item;
                                    crate::bindings::timeline::event_processor::api::TimelineResultPoint {
                                        time_period: {
                                            let record = record
                                                .field(0usize)
                                                .expect("record field not found");
                                            crate::bindings::timeline::event_processor::api::TimePeriod {
                                                t1: record
                                                    .field(0usize)
                                                    .expect("record field not found")
                                                    .u64()
                                                    .expect("u64 not found"),
                                                t2: record
                                                    .field(1usize)
                                                    .expect("record field not found")
                                                    .option()
                                                    .expect("option not found")
                                                    .map(|inner| inner.u64().expect("u64 not found")),
                                            }
                                        },
                                        value: {
                                            let (case_idx, inner) = record
                                                .field(1usize)
                                                .expect("record field not found")
                                                .variant()
                                                .expect("variant not found");
                                            match case_idx {
                                                0u32 => {
                                                    crate::bindings::timeline::event_processor::api::EventValue::StringValue(
                                                        inner
                                                            .expect("variant case not found")
                                                            .string()
                                                            .expect("string not found")
                                                            .to_string(),
                                                    )
                                                }
                                                1u32 => {
                                                    crate::bindings::timeline::event_processor::api::EventValue::IntValue(
                                                        inner
                                                            .expect("variant case not found")
                                                            .s64()
                                                            .expect("i64 not found"),
                                                    )
                                                }
                                                2u32 => {
                                                    crate::bindings::timeline::event_processor::api::EventValue::FloatValue(
                                                        inner
                                                            .expect("variant case not found")
                                                            .f64()
                                                            .expect("f64 not found"),
                                                    )
                                                }
                                                3u32 => {
                                                    crate::bindings::timeline::event_processor::api::EventValue::BoolValue(
                                                        inner
                                                            .expect("variant case not found")
                                                            .bool()
                                                            .expect("bool not found"),
                                                    )
                                                }
                                                _ => unreachable!("invalid variant case index"),
                                            }
                                        },
                                    }
                                })
                                .expect("list not found"),
                        }
                    })
                }
                Err(err_value) => {
                    Err(
                        err_value
                            .expect("result err value not found")
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
            }
        })
    }
    fn get_timeline_result(
        &self,
        t1: u64,
    ) -> crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureGetTimelineResultResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:timeline-processor/api.{get-timeline-result}",
                &[WitValue::builder().u64(t1)],
            );
        crate::bindings::exports::timeline::timeline_processor_stub::stub_timeline_processor::FutureGetTimelineResultResult::new(FutureGetTimelineResultResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);
