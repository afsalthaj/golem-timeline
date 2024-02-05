use std::sync::{Arc, Mutex};
use crate::worker_sink::InMemoryWorkerInvoke;

pub enum BackEnd {
  Golem,
  InMemory(Arc<Mutex<InMemoryWorkerInvoke>>),
}
