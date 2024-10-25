#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
pub struct FutureInitializeLatestEventStateResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeTlHasExistedResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureInitializeTlHasExistedWithinResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureAddEventResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureLatestEventToStateResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureTlHasExistedResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureTlHasExistedWithinResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::Guest
for Component {
    type Api = crate::Api;
    type FutureInitializeLatestEventStateResult = crate::FutureInitializeLatestEventStateResult;
    type FutureInitializeTlHasExistedResult = crate::FutureInitializeTlHasExistedResult;
    type FutureInitializeTlHasExistedWithinResult = crate::FutureInitializeTlHasExistedWithinResult;
    type FutureAddEventResult = crate::FutureAddEventResult;
    type FutureLatestEventToStateResult = crate::FutureLatestEventToStateResult;
    type FutureTlHasExistedResult = crate::FutureTlHasExistedResult;
    type FutureTlHasExistedWithinResult = crate::FutureTlHasExistedWithinResult;
}
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestFutureInitializeLatestEventStateResult
for FutureInitializeLatestEventStateResult {
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
                            "timeline:event-processor/api.{initialize-latest-event-state}"
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
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestFutureInitializeTlHasExistedResult
for FutureInitializeTlHasExistedResult {
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
                            "timeline:event-processor/api.{initialize-tl-has-existed}"
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
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestFutureInitializeTlHasExistedWithinResult
for FutureInitializeTlHasExistedWithinResult {
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
                            "timeline:event-processor/api.{initialize-tl-has-existed-within}"
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
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestFutureAddEventResult
for FutureAddEventResult {
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
                            "timeline:event-processor/api.{add-event}"
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
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestFutureLatestEventToStateResult
for FutureLatestEventToStateResult {
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
        Result<crate::bindings::timeline::event_processor::api::TimelineResult, String>,
    > {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:event-processor/api.{latest-event-to-state}"
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
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestFutureTlHasExistedResult
for FutureTlHasExistedResult {
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
        Result<crate::bindings::timeline::event_processor::api::TimelineResult, String>,
    > {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:event-processor/api.{tl-has-existed}"
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
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestFutureTlHasExistedWithinResult
for FutureTlHasExistedWithinResult {
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
        Result<crate::bindings::timeline::event_processor::api::TimelineResult, String>,
    > {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "timeline:event-processor/api.{tl-has-existed-within}"
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
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestApi
for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_initialize_latest_event_state(
        &self,
        event_col_name: String,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api.{initialize-latest-event-state}",
                &[WitValue::builder().string(&event_col_name)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:event-processor/api.{initialize-latest-event-state}"
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
    fn initialize_latest_event_state(
        &self,
        event_col_name: String,
    ) -> crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureInitializeLatestEventStateResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:event-processor/api.{initialize-latest-event-state}",
                &[WitValue::builder().string(&event_col_name)],
            );
        crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureInitializeLatestEventStateResult::new(FutureInitializeLatestEventStateResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_tl_has_existed(
        &self,
        event_predicate: crate::bindings::timeline::event_processor::api::EventPredicate,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api.{initialize-tl-has-existed}",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .string(&event_predicate.col_name)
                        .item()
                        .variant_fn(
                            match &event_predicate.value {
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
                            match &event_predicate.value {
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
                            |case_builder| match &event_predicate.value {
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
                        )
                        .item()
                        .enum_value(
                            match event_predicate.op {
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::Equal => {
                                    0u32
                                }
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::GreaterThan => {
                                    1u32
                                }
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::LessThan => {
                                    2u32
                                }
                            },
                        )
                        .finish(),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:event-processor/api.{initialize-tl-has-existed}"
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
    fn initialize_tl_has_existed(
        &self,
        event_predicate: crate::bindings::timeline::event_processor::api::EventPredicate,
    ) -> crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureInitializeTlHasExistedResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:event-processor/api.{initialize-tl-has-existed}",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .string(&event_predicate.col_name)
                        .item()
                        .variant_fn(
                            match &event_predicate.value {
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
                            match &event_predicate.value {
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
                            |case_builder| match &event_predicate.value {
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
                        )
                        .item()
                        .enum_value(
                            match event_predicate.op {
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::Equal => {
                                    0u32
                                }
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::GreaterThan => {
                                    1u32
                                }
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::LessThan => {
                                    2u32
                                }
                            },
                        )
                        .finish(),
                ],
            );
        crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureInitializeTlHasExistedResult::new(FutureInitializeTlHasExistedResult {
            future_invoke_result: result,
        })
    }
    fn blocking_initialize_tl_has_existed_within(
        &self,
        event_predicate: crate::bindings::timeline::event_processor::api::EventPredicate,
        time: u64,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api.{initialize-tl-has-existed-within}",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .string(&event_predicate.col_name)
                        .item()
                        .variant_fn(
                            match &event_predicate.value {
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
                            match &event_predicate.value {
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
                            |case_builder| match &event_predicate.value {
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
                        )
                        .item()
                        .enum_value(
                            match event_predicate.op {
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::Equal => {
                                    0u32
                                }
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::GreaterThan => {
                                    1u32
                                }
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::LessThan => {
                                    2u32
                                }
                            },
                        )
                        .finish(),
                    WitValue::builder().u64(time),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:event-processor/api.{initialize-tl-has-existed-within}"
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
    fn initialize_tl_has_existed_within(
        &self,
        event_predicate: crate::bindings::timeline::event_processor::api::EventPredicate,
        time: u64,
    ) -> crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureInitializeTlHasExistedWithinResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:event-processor/api.{initialize-tl-has-existed-within}",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .string(&event_predicate.col_name)
                        .item()
                        .variant_fn(
                            match &event_predicate.value {
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
                            match &event_predicate.value {
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
                            |case_builder| match &event_predicate.value {
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
                        )
                        .item()
                        .enum_value(
                            match event_predicate.op {
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::Equal => {
                                    0u32
                                }
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::GreaterThan => {
                                    1u32
                                }
                                crate::bindings::timeline::event_processor::api::EventPredicateOp::LessThan => {
                                    2u32
                                }
                            },
                        )
                        .finish(),
                    WitValue::builder().u64(time),
                ],
            );
        crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureInitializeTlHasExistedWithinResult::new(FutureInitializeTlHasExistedWithinResult {
            future_invoke_result: result,
        })
    }
    fn blocking_add_event(
        &self,
        event: crate::bindings::timeline::event_processor::api::Event,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api.{add-event}",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .u64(event.time)
                        .item()
                        .list_fn(
                            &event.event,
                            |item, item_builder| {
                                item_builder
                                    .tuple()
                                    .item()
                                    .string(&item.0)
                                    .item()
                                    .variant_fn(
                                        match &item.1 {
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
                                        match &item.1 {
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
                                        |case_builder| match &item.1 {
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
                                    )
                                    .finish()
                            },
                        )
                        .finish(),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:event-processor/api.{add-event}"
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
    fn add_event(
        &self,
        event: crate::bindings::timeline::event_processor::api::Event,
    ) -> crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureAddEventResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:event-processor/api.{add-event}",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .u64(event.time)
                        .item()
                        .list_fn(
                            &event.event,
                            |item, item_builder| {
                                item_builder
                                    .tuple()
                                    .item()
                                    .string(&item.0)
                                    .item()
                                    .variant_fn(
                                        match &item.1 {
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
                                        match &item.1 {
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
                                        |case_builder| match &item.1 {
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
                                    )
                                    .finish()
                            },
                        )
                        .finish(),
                ],
            );
        crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureAddEventResult::new(FutureAddEventResult {
            future_invoke_result: result,
        })
    }
    fn blocking_latest_event_to_state(
        &self,
        t1: u64,
    ) -> Result<
        crate::bindings::timeline::event_processor::api::TimelineResult,
        String,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api.{latest-event-to-state}",
                &[WitValue::builder().u64(t1)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:event-processor/api.{latest-event-to-state}"
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
    fn latest_event_to_state(
        &self,
        t1: u64,
    ) -> crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureLatestEventToStateResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:event-processor/api.{latest-event-to-state}",
                &[WitValue::builder().u64(t1)],
            );
        crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureLatestEventToStateResult::new(FutureLatestEventToStateResult {
            future_invoke_result: result,
        })
    }
    fn blocking_tl_has_existed(
        &self,
        t1: u64,
    ) -> Result<
        crate::bindings::timeline::event_processor::api::TimelineResult,
        String,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api.{tl-has-existed}",
                &[WitValue::builder().u64(t1)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:event-processor/api.{tl-has-existed}"
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
    fn tl_has_existed(
        &self,
        t1: u64,
    ) -> crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureTlHasExistedResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:event-processor/api.{tl-has-existed}",
                &[WitValue::builder().u64(t1)],
            );
        crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureTlHasExistedResult::new(FutureTlHasExistedResult {
            future_invoke_result: result,
        })
    }
    fn blocking_tl_has_existed_within(
        &self,
        t1: u64,
    ) -> Result<
        crate::bindings::timeline::event_processor::api::TimelineResult,
        String,
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api.{tl-has-existed-within}",
                &[WitValue::builder().u64(t1)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "timeline:event-processor/api.{tl-has-existed-within}"
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
    fn tl_has_existed_within(
        &self,
        t1: u64,
    ) -> crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureTlHasExistedWithinResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "timeline:event-processor/api.{tl-has-existed-within}",
                &[WitValue::builder().u64(t1)],
            );
        crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::FutureTlHasExistedWithinResult::new(FutureTlHasExistedWithinResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);
