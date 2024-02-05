use std::sync::{Arc, Mutex};
use crate::worker_sink::InMemoryWorkerSink;

pub enum BackEnd {
  Golem,
  InMemory(Arc<Mutex<InMemoryWorkerSink>>),
}
