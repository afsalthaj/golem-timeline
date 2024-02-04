use std::collections::HashMap;
use crate::timeline::TimeLine;
use crate::timeline_op::TimeLineOp;
use crate::value::Value;
use crate::event_record::RawEventRecord;
use crate::event_stream::EventStream;
use crate::event_type::EventType;
use crate::backend::BackEnd;

trait Eval<T> {
    fn eval(&self, back_end: BackEnd) -> TimeLine<T>;
}

impl<T> Eval<T> for TimeLineOp<T> {
    fn eval(&self, backend: BackEnd) -> T {
        match self {
            // Can pre-inspect and avoid timelines of unnecessary events if needed
            TimeLineOp::Leaf(events) => {
                match events {
                    EventStream::InMemoryEvents(in_memory_events) => {
                        // In a usecase like CIRR, the event types are finite
                        for event in in_memory_events.into_iter() {
                            match backend {
                                BackEnd::InMemory(ref in_memory_sink) => {
                                    let in_memory_key = (event.key, "timelineop::leaf".to_string(), event.event_type);
                                    in_memory_sink.add(in_memory_key, event.time, event.event_type.to_value());
                                }
                                BackEnd::Golem => todo!()

                            }
                        }
                    }
                }
            }

            _ => todo!()
        }
    }

}