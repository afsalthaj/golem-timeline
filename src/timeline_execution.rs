use crate::backend::BackEnd;
use crate::event_stream::EventStream;
use crate::timeline_op::TimeLineOp;
use crate::worker_timeline::WorkerKey;
use crate::worker_timeline_data::InvokeWorker;
use futures::StreamExt;
use futures::{stream, Stream};

pub trait TimeLineExecution {
    // The result is a stream of stream of workers
    // The outer stream represents
    fn run(self, back_end: BackEnd);
}

impl TimeLineExecution for TimeLineOp {
    fn run(self, backend: BackEnd) {
        match self {
            // Can pre-inspect and avoid timelines of unnecessary events if needed
            TimeLineOp::Leaf(events) => {
                match events {
                    EventStream::InMemoryEvents(in_memory_events) => {
                        // In a usecase like CIRR, the event types are finite
                        for event in in_memory_events.events {
                            match backend {
                                BackEnd::InMemory(ref sink) => {
                                    let mut sink = sink.lock().unwrap();
                                    let worker_key = WorkerKey {
                                        time_line_op_name: "time_line_op_leaf".to_string(),
                                        identity: event.key.clone(),
                                    };

                                    sink.add_event_worker(event, &worker_key);
                                }
                                BackEnd::Golem => todo!(),
                            }
                        }
                    }
                }
            }
            _ => todo!(),
        }
    }
}
