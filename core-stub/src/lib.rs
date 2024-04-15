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
    ) -> () {
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
                                            crate::bindings::timeline::core::api::TimelineNode::Primitive(
                                                _,
                                            ) => 1u32,
                                            crate::bindings::timeline::core::api::TimelineNode::NotNode(
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
                                            crate::bindings::timeline::core::api::TimelineNode::Primitive(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::NotNode(
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
                                            ) => case_builder.s32(*inner),
                                            crate::bindings::timeline::core::api::TimelineNode::Primitive(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .enum_value(
                                                        match inner.op {
                                                            crate::bindings::timeline::core::api::TimelinePrimitiveOp::GreaterThan => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelinePrimitiveOp::GreaterThanEqual => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelinePrimitiveOp::LessThan => {
                                                                2u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelinePrimitiveOp::LessThanEqual => {
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
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::NotNode(
                                                inner,
                                            ) => case_builder.s32(*inner),
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExisted(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .enum_value(
                                                        match inner.filter {
                                                            crate::bindings::timeline::core::api::FilterOp::Equal => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::FilterOp::GreaterThan => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::FilterOp::LessThan => {
                                                                2u32
                                                            }
                                                        },
                                                    )
                                                    .item()
                                                    .s32(inner.node)
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
                                                    .enum_value(
                                                        match inner.filtered.filter {
                                                            crate::bindings::timeline::core::api::FilterOp::Equal => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::FilterOp::GreaterThan => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::FilterOp::LessThan => {
                                                                2u32
                                                            }
                                                        },
                                                    )
                                                    .item()
                                                    .s32(inner.filtered.node)
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
                                                    .enum_value(
                                                        match inner.filter {
                                                            crate::bindings::timeline::core::api::FilterOp::Equal => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::FilterOp::GreaterThan => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::FilterOp::LessThan => {
                                                                2u32
                                                            }
                                                        },
                                                    )
                                                    .item()
                                                    .s32(inner.node)
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
                                                    .finish()
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationInCurState(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .enum_value(
                                                        match inner.filter {
                                                            crate::bindings::timeline::core::api::FilterOp::Equal => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::FilterOp::GreaterThan => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::FilterOp::LessThan => {
                                                                2u32
                                                            }
                                                        },
                                                    )
                                                    .item()
                                                    .s32(inner.node)
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
                                                    .finish()
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
        ()
    }
}
