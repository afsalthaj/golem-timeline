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
    fn latest_event_to_state(
        &self,
        event: crate::bindings::timeline::event_processor::api::Event,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:event-processor/api/latest-event-to-state",
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
                    "timeline:event-processor/api/latest-event-to-state"
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
}
