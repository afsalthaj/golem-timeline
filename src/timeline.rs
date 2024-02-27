use crate::event_predicate::EventPredicate;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::timeline_point::TimeLinePoint;
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct TimeLine {
    // we dont use any backend here, but a mere state of the timeline.
    // Flushing of this vector can involve storing it to postgres if needed
    pub points: Vec<TimeLinePoint>
}


impl Default for TimeLine {
    fn default() -> Self {
        TimeLine {
            points: vec![]
        }
    }
}

impl TimeLine {

  // Implement zip with
    // convert to boolean: is it ==  "play-afsal"
    // 8.00 to 8.20 "true"
    // 8.20 to 8.30 "false"
    // 8.30 to 8.40 "true"


    // convert to boolean: is it ==  "play-adam"
    // 8.00 to 8.25 "true"
    // 8.25 to 8.35 "false"
    // 8.25 to 8.39 "true"

    // Step 1: Line up the timelines
    // 8.00 to 8.20 "true" "true"
    // 8.20 to 8.25 "false" "true"
    // 8.25 to 8.30 "false" "false"
    // 8.30 to 8.35 "false" "true"
    // 8.35 to 8.39 "false" "false"

    pub fn zip_with<F>(&self, other: &TimeLine, f: F) -> TimeLine
    where F: Fn(Option<&Value>, Option<&Value>) -> Value {
        let mut merged_points = Vec::new();
        let mut self_iter = self.points.iter().peekable();
        let mut other_iter = other.points.iter().peekable();

        while self_iter.peek().is_some() && other_iter.peek().is_some() {
            let self_point = self_iter.next().unwrap();
            let other_point = other_iter.next().unwrap();

            // Intersection boundary
            let t1 = self_point.t1.max(other_point.t1);

            let t2 = match (self_point.t2, other_point.t2) {
                (Some(t2), None) => Some(t2),
                (None, Some(t2)) => Some(t2),
                (Some(t2), Some(t2_other)) => Some(t2.min(t2_other)),
                (None, None) => None
            };

            // Left Exclusive
            let t1x0 = self_point.t1.min(other_point.t1);
            let t1x1 = t1;

            // Right exclusive
            let t2x0 = t2;
            let t2x1 = match (self_point.t2, other_point.t2) {
                (Some(t2), None) => Some(t2),
                (None, Some(t2)) => Some(t2),
                (Some(t2), Some(t2_other)) => Some(t2.max(t2_other)),
                (None, None) => None
            };

            let intersection = TimeLinePoint {
                t1,
                t2,
                value: f(Some(&self_point.value), Some(&other_point.value))
            };

            // if t1x0 == self_point.t1, then it means t1 is before t2 and the value exists only in self time line.
            let left_ex = if t1x0 == self_point.t1 {
                 TimeLinePoint {
                    t1: t1x0,
                    t2: Some(t1x1),
                    value: f(Some(&self_point.value), None)
                }
            } else {
                // if t1x0 == other_point.t1, then it means t1 is before t2 and the value exists only in self time line.
                TimeLinePoint {
                    t1: t1x0,
                    t2: Some(t1x1),
                    value: f(None, Some(&other_point.value))
                }
            };

            match t2x0 {
                Some(t2x0) => {
                    if Some(t2x0) == self_point.t2 {
                        let right_ex = TimeLinePoint {
                            t1: t2x0,
                            t2: t2x1,
                            value: f(Some(&self_point.value), None)
                        };
                        merged_points.push(right_ex);
                    } else {
                        let right_ex = TimeLinePoint {
                            t1: t2x0,
                            t2: t2x1,
                            value: f(None, Some(&other_point.value))
                        };
                        merged_points.push(right_ex);
                    }
                }
                None => {}
            }

            merged_points.push(intersection);
            merged_points.push(left_ex);
        }

        TimeLine {
            points: merged_points
        }
    }

    // In a state dynamic timeline, the value is valid from t1 to t2
  pub fn add_state_dynamic_info(&mut self, start_time: u64, value: Value) -> &mut TimeLine {
      if self.points.is_empty() {
            self.points.push(TimeLinePoint {
                // epoch starting time
                t1: start_time,
                t2: None,
                value
            });
          self
      } else {
          self.points.last_mut().unwrap().update_t2(start_time);
          self.points.push(TimeLinePoint {
              t1: start_time,
              t2: None,
              value
          });

          self
      }
  }
}

mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_zip_with() {
        let mut timeline1 = TimeLine::default();
        timeline1.add_state_dynamic_info(1708997975, Value::StringValue("playing".to_string()));
        //timeline1.add_state_dynamic_info(1708997980, Value::StringValue("pause".to_string()));

        let mut timeline2 = TimeLine::default();
        timeline2.add_state_dynamic_info(1708997977, Value::StringValue("movie".to_string()));
        //timeline2.add_state_dynamic_info(1708997979, Value::StringValue("cartoon".to_string()));


        // 77 to None "playing" "movie"
        // 75 to 77 "playing"

        // 75 to 77 "playing" (unknown)   (correct)
        // 77 to 79 "playing" "movie"     (correct)
        // 79 to 80 "playing" "cartoon"
        // 80 to Noe "pause" "cartoon"    (correct)
        let result = timeline1.zip_with(&timeline2, |a, b| {
            match (a, b) {
                (Some(a), Some(b)) => {
                    Value::ArrayValue(vec![a.clone(), b.clone()])
                }
                (Some(a), None) => {
                    Value::ArrayValue(vec![a.clone()])
                }
                (None, Some(b)) => {
                    Value::ArrayValue(vec![b.clone()])
                }
                (None, None) => {
                    Value::ArrayValue(vec![])
                }
            }
        });

        dbg!("The timeline is {}", result.clone());
        assert_eq!(result.points.len(), 5);
    }
}

