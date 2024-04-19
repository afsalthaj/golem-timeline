#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::timeline::core_stub::stub_core::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn initialize_timeline(
        &self,
        timeline: crate::bindings::timeline::core::api::TimelineOp,
    ) -> Result<String, String> {
        let result = self
            .rpc
            .invoke_and_await(
                "timeline:core/api/initialize-timeline",
                &[
                    WitValue::builder()
                        .record()
                        .item()
                        .list_fn(
                            &timeline.nodes,
                            |item, item_builder| {
                                item_builder
                                    .variant_fn(
                                        match &item {
                                            crate::bindings::timeline::core::api::TimelineNode::Leaf(
                                                _,
                                            ) => 0u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineComparison(
                                                _,
                                            ) => 1u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineNegation(
                                                _,
                                            ) => 2u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExisted(
                                                _,
                                            ) => 3u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExistedWithin(
                                                _,
                                            ) => 4u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationWhere(
                                                _,
                                            ) => 5u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationInCurState(
                                                _,
                                            ) => 6u32,
                                        },
                                        match &item {
                                            crate::bindings::timeline::core::api::TimelineNode::Leaf(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineComparison(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineNegation(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExisted(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExistedWithin(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationWhere(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationInCurState(
                                                _,
                                            ) => false,
                                        },
                                        |case_builder| match &item {
                                            crate::bindings::timeline::core::api::TimelineNode::Leaf(
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
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineComparison(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .enum_value(
                                                        match inner.op {
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::GreaterThan => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::GreaterThanEqual => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::LessThan => {
                                                                2u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::LessThanEqual => {
                                                                3u32
                                                            }
                                                        },
                                                    )
                                                    .item()
                                                    .s32(inner.timeline)
                                                    .item()
                                                    .variant_fn(
                                                        match &inner.value {
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
                                                        match &inner.value {
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
                                                        |case_builder| match &inner.value {
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
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.server.worker_id)
                                                    .item()
                                                    .string(&inner.server.template_id)
                                                    .finish()
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineNegation(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .s32(inner.timeline)
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.server.worker_id)
                                                    .item()
                                                    .string(&inner.server.template_id)
                                                    .finish()
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExisted(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .s32(inner.timeline)
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.event_predicate.col_name)
                                                    .item()
                                                    .variant_fn(
                                                        match &inner.event_predicate.value {
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
                                                        match &inner.event_predicate.value {
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
                                                        |case_builder| match &inner.event_predicate.value {
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
                                                    .item()
                                                    .enum_value(
                                                        match inner.event_predicate.op {
                                                            crate::bindings::timeline::core::api::EventPredicateOp::Equal => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::EventPredicateOp::GreaterThan => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::EventPredicateOp::LessThan => {
                                                                2u32
                                                            }
                                                        },
                                                    )
                                                    .finish()
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.server.worker_id)
                                                    .item()
                                                    .string(&inner.server.template_id)
                                                    .finish()
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExistedWithin(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .s32(inner.filtered.timeline)
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.filtered.event_predicate.col_name)
                                                    .item()
                                                    .variant_fn(
                                                        match &inner.filtered.event_predicate.value {
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
                                                        match &inner.filtered.event_predicate.value {
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
                                                        |case_builder| match &inner.filtered.event_predicate.value {
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
                                                    .item()
                                                    .enum_value(
                                                        match inner.filtered.event_predicate.op {
                                                            crate::bindings::timeline::core::api::EventPredicateOp::Equal => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::EventPredicateOp::GreaterThan => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::EventPredicateOp::LessThan => {
                                                                2u32
                                                            }
                                                        },
                                                    )
                                                    .finish()
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.filtered.server.worker_id)
                                                    .item()
                                                    .string(&inner.filtered.server.template_id)
                                                    .finish()
                                                    .finish()
                                                    .item()
                                                    .u64(inner.time)
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationWhere(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.server.worker_id)
                                                    .item()
                                                    .string(&inner.server.template_id)
                                                    .finish()
                                                    .item()
                                                    .s32(inner.timeline)
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationInCurState(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.server.worker_id)
                                                    .item()
                                                    .string(&inner.server.template_id)
                                                    .finish()
                                                    .item()
                                                    .s32(inner.timeline)
                                                    .finish()
                                            }
                                        },
                                    )
                            },
                        )
                        .finish(),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}", "timeline:core/api/initialize-timeline"
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
