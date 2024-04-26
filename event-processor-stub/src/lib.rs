#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::timeline::event_processor_stub::stub_event_processor::GuestApi
    for Api
{
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn initialize_latest_event_state(
        &self,
        worker: crate::bindings::timeline::event_processor::api::WorkerId,
        event_col_name: String,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api/initialize-latest-event-state",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .string(&worker.name)
                        .finish(),
                    WitValue::builder().string(&event_col_name),
                ],
            )
            .expect(&format!(
                "Failed to invoke remote {}",
                "timeline:event-processor/api/initialize-latest-event-state"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(ok_value
                    .expect("result ok value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
                Err(err_value) => Err(err_value
                    .expect("result err value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
            }
        })
    }
    fn initialize_tl_has_existed(
        &self,
        worker: crate::bindings::timeline::event_processor::api::WorkerId,
        event_predicate: crate::bindings::timeline::event_processor::api::EventPredicate,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api/initialize-tl-has-existed",
                &[
                    WitValue::builder().record().item().string(&worker.name).finish(),
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
                    "Failed to invoke remote {}",
                    "timeline:event-processor/api/initialize-tl-has-existed"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(ok_value
                    .expect("result ok value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
                Err(err_value) => Err(err_value
                    .expect("result err value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
            }
        })
    }
    fn add_event(
        &self,
        event: crate::bindings::timeline::event_processor::api::Event,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api/add-event",
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
                    "Failed to invoke remote {}",
                    "timeline:event-processor/api/add-event"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(ok_value
                    .expect("result ok value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
                Err(err_value) => Err(err_value
                    .expect("result err value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
            }
        })
    }
    fn latest_event_to_state(
        &self,
        t1: u64,
    ) -> Result<crate::bindings::timeline::event_processor::api::LatestEventToStateResult, String>
    {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api/latest-event-to-state",
                &[WitValue::builder().u64(t1)],
            )
            .expect(&format!(
                "Failed to invoke remote {}",
                "timeline:event-processor/api/latest-event-to-state"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok({
                    let record = ok_value.expect("result ok value not found");
                    crate::bindings::timeline::event_processor::api::LatestEventToStateResult {
                            event_col_name: record
                                .field(0usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            event_results: record
                                .field(1usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    let record = item;
                                    crate::bindings::timeline::event_processor::api::EventStateResult {
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
                                                    .u64()
                                                    .expect("u64 not found"),
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
                }),
                Err(err_value) => Err(err_value
                    .expect("result err value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
            }
        })
    }
    fn tl_has_existed(&self, t1: u64) -> Result<bool, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api/tl-has-existed",
                &[WitValue::builder().u64(t1)],
            )
            .expect(&format!(
                "Failed to invoke remote {}",
                "timeline:event-processor/api/tl-has-existed"
            ));
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => Ok(ok_value
                    .expect("result ok value not found")
                    .bool()
                    .expect("bool not found")),
                Err(err_value) => Err(err_value
                    .expect("result err value not found")
                    .string()
                    .expect("string not found")
                    .to_string()),
            }
        })
    }
}
