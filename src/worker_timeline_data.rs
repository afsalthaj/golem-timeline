use crate::event_record::RawEventRecord;
use crate::state_dynamics_timeline::StateDynamicsTimeLine;
use crate::worker_timeline::{WorkerKey, WorkerTimeLineData};

// Interface to invoke worker and update timeline
pub trait InvokeWorker {
    fn add_worker(&mut self, raw_event_record: &RawEventRecord, worker: &WorkerKey);
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
    fn add_worker(&mut self, event: &RawEventRecord, worker_key: &WorkerKey) {
        // For in-memory, see if worker already exist and update the timeline
        let worker = self.get_worker_mut(&worker_key);

        match worker {
            Some(worker) => {
                worker
                    .timeline
                    .add_state_dynamic_info(event.time, event.event_type.to_value());
            }
            None => {
                let mut timeline = StateDynamicsTimeLine::default();
                timeline.add_state_dynamic_info(event.time, event.event_type.to_value());
                let worker = WorkerTimeLineData {
                    key: worker_key.clone(),
                    timeline,
                };
                self.workers.push(worker);
            }
        }
    }
}
