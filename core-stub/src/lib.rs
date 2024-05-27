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
    ) -> Result<crate::bindings::timeline::core::api::WorkerDetails, String> {
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
                                            crate::bindings::timeline::core::api::TimelineNode::TlLatestEventToState(
                                                _,
                                            ) => 0u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExisted(
                                                _,
                                            ) => 1u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExistedWithin(
                                                _,
                                            ) => 2u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineComparison(
                                                _,
                                            ) => 3u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineNegation(
                                                _,
                                            ) => 4u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationWhere(
                                                _,
                                            ) => 5u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationInCurState(
                                                _,
                                            ) => 6u32,
                                            crate::bindings::timeline::core::api::TimelineNode::TlAnd(
                                                _,
                                            ) => 7u32,
                                        },
                                        match &item {
                                            crate::bindings::timeline::core::api::TimelineNode::TlLatestEventToState(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExisted(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExistedWithin(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineComparison(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TimelineNegation(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationWhere(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationInCurState(
                                                _,
                                            ) => false,
                                            crate::bindings::timeline::core::api::TimelineNode::TlAnd(
                                                _,
                                            ) => false,
                                        },
                                        |case_builder| match &item {
                                            crate::bindings::timeline::core::api::TimelineNode::TlLatestEventToState(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .option_fn(
                                                        inner.server.is_some(),
                                                        |some_builder| {
                                                            some_builder
                                                                .record()
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().worker_id_prefix)
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().template_id)
                                                                .finish()
                                                        },
                                                    )
                                                    .item()
                                                    .string(&inner.event_column_name)
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TlHasExisted(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .record()
                                                    .item()
                                                    .string(&inner.event_predicate.col_name)
                                                    .item()
                                                    .variant_fn(
                                                        match &inner.event_predicate.value {
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
                                                        match &inner.event_predicate.value {
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
                                                        |case_builder| match &inner.event_predicate.value {
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
                                                        match inner.event_predicate.op {
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
                                                    .finish()
                                                    .item()
                                                    .option_fn(
                                                        inner.server.is_some(),
                                                        |some_builder| {
                                                            some_builder
                                                                .record()
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().worker_id_prefix)
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().template_id)
                                                                .finish()
                                                        },
                                                    )
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
                                                    .record()
                                                    .item()
                                                    .string(&inner.filtered.event_predicate.col_name)
                                                    .item()
                                                    .variant_fn(
                                                        match &inner.filtered.event_predicate.value {
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
                                                        match &inner.filtered.event_predicate.value {
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
                                                        |case_builder| match &inner.filtered.event_predicate.value {
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
                                                        match inner.filtered.event_predicate.op {
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
                                                    .finish()
                                                    .item()
                                                    .option_fn(
                                                        inner.filtered.server.is_some(),
                                                        |some_builder| {
                                                            some_builder
                                                                .record()
                                                                .item()
                                                                .string(
                                                                    &inner.filtered.server.as_ref().unwrap().worker_id_prefix,
                                                                )
                                                                .item()
                                                                .string(
                                                                    &inner.filtered.server.as_ref().unwrap().template_id,
                                                                )
                                                                .finish()
                                                        },
                                                    )
                                                    .finish()
                                                    .item()
                                                    .u64(inner.time)
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
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::EqualTo => {
                                                                0u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::GreaterThan => {
                                                                1u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::GreaterThanEqual => {
                                                                2u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::LessThan => {
                                                                3u32
                                                            }
                                                            crate::bindings::timeline::core::api::TimelineConstantComparator::LessThanEqual => {
                                                                4u32
                                                            }
                                                        },
                                                    )
                                                    .item()
                                                    .s32(inner.timeline)
                                                    .item()
                                                    .variant_fn(
                                                        match &inner.value {
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
                                                        match &inner.value {
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
                                                        |case_builder| match &inner.value {
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
                                                    .option_fn(
                                                        inner.server.is_some(),
                                                        |some_builder| {
                                                            some_builder
                                                                .record()
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().worker_id_prefix)
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().template_id)
                                                                .finish()
                                                        },
                                                    )
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
                                                    .option_fn(
                                                        inner.server.is_some(),
                                                        |some_builder| {
                                                            some_builder
                                                                .record()
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().worker_id_prefix)
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().template_id)
                                                                .finish()
                                                        },
                                                    )
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TlDurationWhere(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .option_fn(
                                                        inner.server.is_some(),
                                                        |some_builder| {
                                                            some_builder
                                                                .record()
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().worker_id_prefix)
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().template_id)
                                                                .finish()
                                                        },
                                                    )
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
                                                    .option_fn(
                                                        inner.server.is_some(),
                                                        |some_builder| {
                                                            some_builder
                                                                .record()
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().worker_id_prefix)
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().template_id)
                                                                .finish()
                                                        },
                                                    )
                                                    .item()
                                                    .s32(inner.timeline)
                                                    .finish()
                                            }
                                            crate::bindings::timeline::core::api::TimelineNode::TlAnd(
                                                inner,
                                            ) => {
                                                case_builder
                                                    .record()
                                                    .item()
                                                    .option_fn(
                                                        inner.server.is_some(),
                                                        |some_builder| {
                                                            some_builder
                                                                .record()
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().worker_id_prefix)
                                                                .item()
                                                                .string(&inner.server.as_ref().unwrap().template_id)
                                                                .finish()
                                                        },
                                                    )
                                                    .item()
                                                    .s32(inner.left)
                                                    .item()
                                                    .s32(inner.right)
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
                    Ok({
                        let record = ok_value.expect("result ok value not found");
                        crate::bindings::timeline::core::api::WorkerDetails {
                            event_processor_workers: record
                                .field(0usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    let (case_idx, inner) = item
                                        .variant()
                                        .expect("variant not found");
                                    match case_idx {
                                        0u32 => {
                                            crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline({
                                                let (case_idx, inner) = inner
                                                    .expect("variant case not found")
                                                    .variant()
                                                    .expect("variant not found");
                                                match case_idx {
                                                    0u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    1u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    2u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    _ => unreachable!("invalid variant case index"),
                                                }
                                            })
                                        }
                                        1u32 => {
                                            crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline({
                                                let (case_idx, inner) = inner
                                                    .expect("variant case not found")
                                                    .variant()
                                                    .expect("variant not found");
                                                match case_idx {
                                                    0u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    1u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    2u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    3u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    4u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    5u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    6u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    7u32 => {
                                                        crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not({
                                                            let record = inner.expect("variant case not found");
                                                            crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                                worker_id: record
                                                                    .field(0usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                                template_id: record
                                                                    .field(1usize)
                                                                    .expect("record field not found")
                                                                    .string()
                                                                    .expect("string not found")
                                                                    .to_string(),
                                                            }
                                                        })
                                                    }
                                                    _ => unreachable!("invalid variant case index"),
                                                }
                                            })
                                        }
                                        _ => unreachable!("invalid variant case index"),
                                    }
                                })
                                .expect("list not found"),
                            result_worker: {
                                let (case_idx, inner) = record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .variant()
                                    .expect("variant not found");
                                match case_idx {
                                    0u32 => {
                                        crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::LeafTimeline({
                                            let (case_idx, inner) = inner
                                                .expect("variant case not found")
                                                .variant()
                                                .expect("variant not found");
                                            match case_idx {
                                                0u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExisted({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                1u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlHasExistedWithin({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                2u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::LeafTimelineNode::TlLatestEventToState({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                _ => unreachable!("invalid variant case index"),
                                            }
                                        })
                                    }
                                    1u32 => {
                                        crate::bindings::timeline::timeline_processor::api::TypedTimelineResultWorker::DerivedTimeline({
                                            let (case_idx, inner) = inner
                                                .expect("variant case not found")
                                                .variant()
                                                .expect("variant not found");
                                            match case_idx {
                                                0u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::EqualTo({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                1u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThan({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                2u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::GreaterThanOrEqualTo({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                3u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThan({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                4u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::LessThanOrEqualTo({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                5u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::And({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                6u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Or({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                7u32 => {
                                                    crate::bindings::timeline::timeline_processor::api::DerivedTimelineNode::Not({
                                                        let record = inner.expect("variant case not found");
                                                        crate::bindings::timeline::timeline_processor::api::TimelineResultWorker {
                                                            worker_id: record
                                                                .field(0usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                            template_id: record
                                                                .field(1usize)
                                                                .expect("record field not found")
                                                                .string()
                                                                .expect("string not found")
                                                                .to_string(),
                                                        }
                                                    })
                                                }
                                                _ => unreachable!("invalid variant case index"),
                                            }
                                        })
                                    }
                                    _ => unreachable!("invalid variant case index"),
                                }
                            },
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
}
