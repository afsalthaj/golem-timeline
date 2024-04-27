#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
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
    fn initialize_equal(
        &self,
        child_url: crate::bindings::timeline::timeline_processor::api::WorkerId,
        current_worker_id: crate::bindings::timeline::timeline_processor::api::WorkerId,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api/initialize-equal",
                &[
                    WitValue::builder().record().item().string(&child_url.name).finish(),
                    WitValue::builder()
                        .record()
                        .item()
                        .string(&current_worker_id.name)
                        .finish(),
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
                    "Failed to invoke remote {}",
                    "timeline:timeline-processor/api/initialize-equal"
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
