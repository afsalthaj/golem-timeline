use crate::event_predicate::EventPredicate;
use std::time::{UNIX_EPOCH};
use crate::timeline_point::TimeLinePoint;

pub struct TimeLine<T> {
    // we dont use any backend here, but a mere state of the timeline.
    // Flushing of this vector can involve storing it to postgres if needed
    points: Vec<TimeLinePoint<T>>
}

impl<T> Default for TimeLine<T> {
    fn default() -> Self {
        TimeLine {
            points: vec![]
        }
    }
}

impl<T> TimeLine<T> {
  fn add_point(&mut self, end_time: u64, value: T) -> &mut TimeLine<T> {
      if self.points.is_empty() {
            self.points.push(TimeLinePoint {
                // epoch starting time
                t1: UNIX_EPOCH.elapsed().unwrap().as_secs(),
                t2: end_time,
                value
            });
          self
      } else {
          let last_point = self.points.last().unwrap();
          self.points.push(TimeLinePoint {
              t1: last_point.t2,
              t2: end_time,
              value
          });

          self
      }
  }

}


fn tl_has_existed<T>(event_stream: event_predicate: EventPredicate) -> TimeLine<bool> {

}

fn tl_has_existed_within<T>(event_predicate: EventPredicate) -> TimeLineStream<bool> {
    unimplemented!("tl_has_existed_within not implemented")
}


fn tl_latest_event_to_state<T>(event_predicate: EventPredicate) -> TimeLineStream<bool> {
    unimplemented!("tl_latest_event_to_state not implemented")
}

fn tl_duration_where<T>(event_predicate: EventPredicate) -> TimeLineStream<u64> {
    unimplemented!("tl_duration_where not implemented")
}

fn tl_duration_in_cur_state<T>() -> TimeLineStream<u64> {
    unimplemented!("tl_duration_in_cur_state not implemented")
}
