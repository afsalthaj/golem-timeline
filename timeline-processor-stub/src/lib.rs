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
        child_worker: crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker,
        event_value: crate::bindings::timeline::timeline_processor::api::EventValue,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:timeline-processor/api/initialize-equal",
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
                                                        .record()
                                                        .item()
                                                        .string(&inner.left.worker_id)
                                                        .item()
                                                        .string(&inner.left.template_id)
                                                        .finish()
                                                        .item()
                                                        .record()
                                                        .item()
                                                        .string(&inner.right.worker_id)
                                                        .item()
                                                        .string(&inner.right.template_id)
                                                        .finish()
                                                        .finish()
                                                }
                                                crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or(
                                                    inner,
                                                ) => {
                                                    case_builder
                                                        .record()
                                                        .item()
                                                        .record()
                                                        .item()
                                                        .string(&inner.left.worker_id)
                                                        .item()
                                                        .string(&inner.left.template_id)
                                                        .finish()
                                                        .item()
                                                        .record()
                                                        .item()
                                                        .string(&inner.right.worker_id)
                                                        .item()
                                                        .string(&inner.right.template_id)
                                                        .finish()
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
