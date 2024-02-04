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


// Each o the below functions invokes a worker
// Each worker is responsible for forgetting past beyond an extent
// This limitation exists in any real world system
// This is more of tracking a StateDynamic event, as a cumulative OR
// Input
// t1: false
// t2: true
// t3: false

// Output
// t1-t2: false
// t2-t3: true
fn tl_has_existed<T>(event_stream: event_predicate: EventPredicate) -> TimeLine<bool> {

}

// This is more of tracking a StateDynamic event, as a cumulative OR
// Input
// Duration: D = 4
// t1: false
// t3: true
// t9: true

// Output
// t1-t3: false
// t3-t7: true
// t7-t9: false
// t9-t13: true
fn tl_has_existed_within<T>(event_predicate: EventPredicate) -> TimeLineStream<bool> {
    unimplemented!("tl_has_existed_within not implemented")
}


// This is more or less making number of events to a very simple
// timeline. Obviously this is corresponding to the events that are state dynamic in nature
// t1 - t10 : CDN2
// t10 - t11 : CDN1
// t11- t12: CDN1

//Output
// t1-t10: CDN2
// t10-t12: CDN1
fn tl_latest_event_to_state<T>(event_predicate: EventPredicate) -> TimeLineStream<bool> {
    unimplemented!("tl_latest_event_to_state not implemented")
}

// A Numerical Timeline of
// the cumulative duration
// where the state was True
// t1 - t3: false
// t3 - t8: true
// t8 - t14: false
// t14 - t20: true


// Output
// t1 - t3: 0
// t3 - t8 : 5
// t8 - t4 : 5
// t14 - t20: 11
fn tl_duration_where<T>(event_predicate: EventPredicate) -> TimeLineStream<u64> {
    unimplemented!("tl_duration_where not implemented")
}

// A Numerical Timeline of
// the duration since the last
// state change

// t1-t3: buffer
// t3-t8: play
// t8-t14: buffer
// t14-t20: pause

// Output
// t1-t3: 3
// t3- t8: 5
// t8-t14: 6
// t14- t20: 6
fn tl_duration_in_cur_state<T>() -> TimeLineStream<u64> {
    unimplemented!("tl_duration_in_cur_state not implemented")
}
