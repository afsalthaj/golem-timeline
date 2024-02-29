pub struct EventTimeLine<T> {
    pub points: Vec<EventTimeLinePoint<T>>,
}

impl<T> EventTimeLine<T> {
    pub fn new() -> EventTimeLine<T> {
        EventTimeLine { points: Vec::new() }
    }

    pub fn add_event_info(&mut self, t1: u64, value: T) -> &mut EventTimeLine<T> {
        self.points.push(EventTimeLinePoint { t1, value });
        self
    }
}

pub struct EventTimeLinePoint<T> {
    pub t1: u64,
    pub value: T,
}
