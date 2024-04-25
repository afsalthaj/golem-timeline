#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
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
    fn initialize(
        &self,
        worker: crate::bindings::timeline::event_processor::api::WorkerId,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api/initialize",
                &[WitValue::builder().record().item().string(&worker.name).finish()],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "timeline:event-processor/api/initialize"
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
    fn get_events(&self) -> Vec<crate::bindings::timeline::event_processor::api::Event> {
        let result = self
            .rpc
            .invoke_and_await("timeline:event-processor/api/get-events", &[])
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "timeline:event-processor/api/get-events"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let record = item;
                crate::bindings::timeline::event_processor::api::Event {
                    time: record
                        .field(0usize)
                        .expect("record field not found")
                        .u64()
                        .expect("u64 not found"),
                    event: record
                        .field(1usize)
                        .expect("record field not found")
                        .list_elements(|item| {
                            let tuple = item;
                            (
                                tuple
                                    .tuple_element(0usize)
                                    .expect("tuple element not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                {
                                    let (case_idx, inner) = tuple
                                        .tuple_element(1usize)
                                        .expect("tuple element not found")
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
                            )
                        })
                        .expect("list not found"),
                }
            })
            .expect("list not found"))
    }
}
