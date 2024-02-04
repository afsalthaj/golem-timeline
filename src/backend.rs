use std::collections::HashMap;
use crate::event_stream::EventStream;
use crate::event_type::EventType;
use crate::timeline::TimeLine;
use crate::timeline_op::TimeLineOp;
use crate::value::Value;

pub enum BackEnd {
  Golem,
  InMemory(Box<dyn InMemorySink>)
}

// Every timeline or event is associated with some sort of a key
type EntityKey = String;
type TimeLineOpHash = String;
type InMemoryKey = (EntityKey, TimeLineOpHash, EventType);

pub trait InMemorySink {
  fn add(mut self, key: InMemoryKey, time: u64, value: Value);
}

struct InMemorySinkDefault {
  pub timelines:  HashMap<InMemoryKey, TimeLine<Value>>
}

impl InMemorySink for InMemorySinkDefault {
  fn add(mut self, key: InMemoryKey, time: u64, value: Value) {
    let timeline = self.timelines.entry(key).or_insert_with(|| {
         TimeLine::default()
    });
     timeline.add_info(time, value);

  }
}