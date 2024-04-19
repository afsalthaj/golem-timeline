#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::timeline::raw_events_stub::stub_raw_events::GuestApi
for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn initialize(
        &self,
        worker: crate::bindings::timeline::raw_events::api::WorkerId,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:raw-events/api/initialize",
                &[WitValue::builder().record().item().string(&worker.name).finish()],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}", "timeline:raw-events/api/initialize"
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
    fn add_event(&self, order: crate::bindings::timeline::raw_events::api::Event) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:raw-events/api/add-event",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .u64(order.time)
                        .item()
                        .variant_fn(
                            match &order.event {
                                crate::bindings::timeline::raw_events::api::EventValue::StringValue(
                                    _,
                                ) => 0u32,
                                crate::bindings::timeline::raw_events::api::EventValue::IntValue(
                                    _,
                                ) => 1u32,
                                crate::bindings::timeline::raw_events::api::EventValue::FloatValue(
                                    _,
                                ) => 2u32,
                                crate::bindings::timeline::raw_events::api::EventValue::BoolValue(
                                    _,
                                ) => 3u32,
                            },
                            match &order.event {
                                crate::bindings::timeline::raw_events::api::EventValue::StringValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::raw_events::api::EventValue::IntValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::raw_events::api::EventValue::FloatValue(
                                    _,
                                ) => false,
                                crate::bindings::timeline::raw_events::api::EventValue::BoolValue(
                                    _,
                                ) => false,
                            },
                            |case_builder| match &order.event {
                                crate::bindings::timeline::raw_events::api::EventValue::StringValue(
                                    inner,
                                ) => case_builder.string(&inner),
                                crate::bindings::timeline::raw_events::api::EventValue::IntValue(
                                    inner,
                                ) => case_builder.s64(*inner),
                                crate::bindings::timeline::raw_events::api::EventValue::FloatValue(
                                    inner,
                                ) => case_builder.f64(*inner),
                                crate::bindings::timeline::raw_events::api::EventValue::BoolValue(
                                    inner,
                                ) => case_builder.bool(*inner),
                            },
                        )
                        .finish(),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}", "timeline:raw-events/api/add-event"
                ),
            );
        ()
    }
    fn get_event(&self, time: u64) -> crate::bindings::timeline::raw_events::api::Event {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:raw-events/api/get-event",
                &[WitValue::builder().u64(time)],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}", "timeline:raw-events/api/get-event"
                ),
            );
        ({
            let record = result.tuple_element(0).expect("tuple not found");
            crate::bindings::timeline::raw_events::api::Event {
                time: record
                    .field(0usize)
                    .expect("record field not found")
                    .u64()
                    .expect("u64 not found"),
                event: {
                    let (case_idx, inner) = record
                        .field(1usize)
                        .expect("record field not found")
                        .variant()
                        .expect("variant not found");
                    match case_idx {
                        0u32 => {
                            crate::bindings::timeline::raw_events::api::EventValue::StringValue(
                                inner
                                    .expect("variant case not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        1u32 => {
                            crate::bindings::timeline::raw_events::api::EventValue::IntValue(
                                inner
                                    .expect("variant case not found")
                                    .s64()
                                    .expect("i64 not found"),
                            )
                        }
                        2u32 => {
                            crate::bindings::timeline::raw_events::api::EventValue::FloatValue(
                                inner
                                    .expect("variant case not found")
                                    .f64()
                                    .expect("f64 not found"),
                            )
                        }
                        3u32 => {
                            crate::bindings::timeline::raw_events::api::EventValue::BoolValue(
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
    }
    fn get_events(&self) -> Vec<crate::bindings::timeline::raw_events::api::Event> {
        let result = self
            .rpc
            .invoke_and_await("timeline:raw-events/api/get-events", &[])
            .expect(
                &format!(
                    "Failed to invoke remote {}", "timeline:raw-events/api/get-events"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let record = item;
                crate::bindings::timeline::raw_events::api::Event {
                    time: record
                        .field(0usize)
                        .expect("record field not found")
                        .u64()
                        .expect("u64 not found"),
                    event: {
                        let (case_idx, inner) = record
                            .field(1usize)
                            .expect("record field not found")
                            .variant()
                            .expect("variant not found");
                        match case_idx {
                            0u32 => {
                                crate::bindings::timeline::raw_events::api::EventValue::StringValue(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            1u32 => {
                                crate::bindings::timeline::raw_events::api::EventValue::IntValue(
                                    inner
                                        .expect("variant case not found")
                                        .s64()
                                        .expect("i64 not found"),
                                )
                            }
                            2u32 => {
                                crate::bindings::timeline::raw_events::api::EventValue::FloatValue(
                                    inner
                                        .expect("variant case not found")
                                        .f64()
                                        .expect("f64 not found"),
                                )
                            }
                            3u32 => {
                                crate::bindings::timeline::raw_events::api::EventValue::BoolValue(
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
            .expect("list not found"))
    }
}
