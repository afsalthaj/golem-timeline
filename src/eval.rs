use crate::timeline::TimeLine;
use crate::timeline_op::TimeLineOp;
use crate::event_stream::EventStream;
use crate::backend::{BackEnd};
use crate::backend::WorkerSink;
use crate::value::Value;
use crate::worker::{Worker, WorkerKey};

trait TimeLineExecution {
     fn run(&self, back_end: BackEnd);
}

impl TimeLineExecution for TimeLineOp {
    fn run(&self, backend: BackEnd){
        match self {
            // Can pre-inspect and avoid timelines of unnecessary events if needed
            TimeLineOp::Leaf(events) => {
                match events {
                    EventStream::InMemoryEvents(in_memory_events) => {
                        // In a usecase like CIRR, the event types are finite
                        for event in in_memory_events.into_iter() {
                            match backend {
                                BackEnd::InMemory(ref sink) => {
                                    let mut sink = sink.lock().unwrap();
                                    let worker_key = WorkerKey {
                                        time_line_op_name: "time_line_op_leaf".to_string(),
                                        identity: event.key.clone()
                                    };
                                    let worker = sink.get_worker_mut(&worker_key);

                                    match worker {
                                        Some(worker) => {
                                            update_timeline(worker, event.time, event.event_type.to_value());
                                        }
                                        None => {
                                            let mut timeline = TimeLine::default();
                                            timeline.add_info(event.time, event.event_type.to_value());
                                            let worker = Worker {
                                                key: worker_key,
                                                timeline
                                            };
                                            sink.add_worker(worker);
                                        }

                                    }
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

fn update_timeline(worker: &mut Worker, time: u64, value: Value) {
    worker.timeline.add_info(time, value);
}