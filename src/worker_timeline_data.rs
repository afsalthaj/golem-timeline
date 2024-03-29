use crate::event_record::RawEventRecord;
use crate::event_timeline::EventTimeLine;
use crate::timeline::TimeLine;
use crate::worker_timeline::{WorkerKey, WorkerTimeLineData};

pub trait InvokeWorker {
    fn add_event_worker(&mut self, raw_event_record: RawEventRecord, worker: &WorkerKey);
}

pub struct InMemoryWorkerInvoke {
    pub workers: Vec<WorkerTimeLineData>,
}

// Implement methods for InMemoryWorkerSink
impl InMemoryWorkerInvoke {
    pub fn new() -> Self {
        // Initialize with an empty vector of workers
        InMemoryWorkerInvoke {
            workers: Vec::new(),
        }
    }

    pub fn workers(&self) -> &Vec<WorkerTimeLineData> {
        // Return a reference to the vector of workers
        &self.workers
    }

    pub fn get_worker_mut(&mut self, key: &WorkerKey) -> Option<&mut WorkerTimeLineData> {
        self.workers
            .iter_mut()
            .find(|worker| worker.key == key.clone())
    }
}

impl InvokeWorker for InMemoryWorkerInvoke {
    fn add_event_worker(&mut self, event: RawEventRecord, worker_key: &WorkerKey) {
        // For in-memory, see if worker already exist and update the timeline
        let worker = self.get_worker_mut(&worker_key);

        match worker {
            Some(worker) => {
                let event_cloned = event.event;
                worker.timeline.add_info(event.time, event_cloned);
            }
            None => {
                let mut timeline = EventTimeLine::default();
                let event_cloned = event.event;

                timeline.add_event_info(event.time, event_cloned);
                let worker = WorkerTimeLineData {
                    key: worker_key.clone(),
                    timeline: TimeLine::EventTime(timeline),
                };
                self.workers.push(worker);
            }
        }
    }
}
