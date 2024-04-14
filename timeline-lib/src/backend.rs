use crate::worker_timeline_data::InMemoryWorkerInvoke;
use std::sync::{Arc, Mutex};

pub enum BackEnd {
    Golem,
    InMemory(Arc<Mutex<InMemoryWorkerInvoke>>),
}
