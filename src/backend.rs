use std::sync::{Arc, Mutex};
use crate::worker::{Worker, WorkerKey};

pub enum BackEnd {
  Golem,
  InMemory(Arc<Mutex<InMemoryWorkerSink>>),
}

pub struct InMemoryWorkerSink {
  pub workers: Vec<Worker>,
}

// Implement methods for InMemoryWorkerSink
impl InMemoryWorkerSink {
  fn new() -> Self {
    // Initialize with an empty vector of workers
    InMemoryWorkerSink { workers: Vec::new() }
  }
}
// Define a trait for an in-memory sink
pub trait WorkerSink {
  fn add_worker(&mut self, worker: Worker);
  fn workers(&self) -> &Vec<Worker>;
  fn get_worker_mut(&mut self, worker_key: &WorkerKey) -> Option<&mut Worker>;
}

impl WorkerSink for InMemoryWorkerSink {
  fn add_worker(&mut self, worker: Worker) {
    // Add the worker to the vector
    self.workers.push(worker);
  }

  fn workers(&self) -> &Vec<Worker> {
    // Return a reference to the vector of workers
    &self.workers
  }

  fn get_worker_mut(&mut self, key: &WorkerKey) -> Option<&mut Worker> {
    self.workers.iter_mut().find(|worker| worker.key == key.clone())  }
}