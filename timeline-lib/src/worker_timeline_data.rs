use crate::bindings::timeline::rawevents::api::Event as RawEventRecord;
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
