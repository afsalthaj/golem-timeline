use crate::event_record::RawEventRecord;

pub enum EventStream {
    InMemoryEvents(InMemoryEventStream),
}


pub struct InMemoryEventStream {
    pub events: Vec<RawEventRecord>,
}

impl<'a> IntoIterator for &'a InMemoryEventStream {
    type Item = &'a RawEventRecord;
    type IntoIter = std::slice::Iter<'a, RawEventRecord>;

    fn into_iter(self) -> Self::IntoIter {
        self.events.iter()
    }
}