use std::sync::{Arc, Mutex};
use crate::worker_timeline_data::InMemoryWorkerInvoke;

pub enum BackEnd {
  Golem,
  InMemory(Arc<Mutex<InMemoryWorkerInvoke>>),
}
