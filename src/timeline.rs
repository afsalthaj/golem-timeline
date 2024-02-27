use crate::event_predicate::EventPredicate;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::timeline_point::TimeLinePoint;
use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct TimeLine<T> {
    // we dont use any backend here, but a mere state of the timeline.
    // Flushing of this vector can involve storing it to postgres if needed
    pub points: Vec<TimeLinePoint<T>>
}


impl<T> Default for TimeLine<T> {
    fn default() -> Self {
        TimeLine {
            points: vec![]
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum ZipResult<'t, T: Clone> {
    Both((&'t T, &'t T)),
    Singleton(&'t T)
}

impl<T: Clone> ZipResult<T> {
    pub fn combine(&self, other: &ZipResult<T>) -> ZipResult<T> {
       match (self, other) {
           (ZipResult::Singleton(a), ZipResult::Singleton(c)) => {
               ZipResult::Both((a, c))
           }
           _ => panic!("Cannot combine")
       }
    }
}

impl<T: std::fmt::Debug + Clone> TimeLine<T> {

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

    //         timeline1.add_state_dynamic_info(5, Value::StringValue("playing".to_string()));
    //         timeline1.add_state_dynamic_info(8, Value::StringValue("pause".to_string()));
    //
    //         let mut timeline2 = TimeLine::default();
    //         timeline2.add_state_dynamic_info(7, Value::StringValue("movie".to_string()));
    //         timeline2.add_state_dynamic_info(9, Value::StringValue("cartoon".to_string()));
    pub fn zip_with<F>(&self, other: &TimeLine<T>, f: F) -> TimeLine<T>
    where F: Fn(&ZipResult<T>) -> Value {
        let mut flattened_time_line_points = Vec::new();
        let mut self_iter = self.points.iter().peekable();
        let mut other_iter = other.points.iter().peekable();

        while self_iter.peek().is_some() && other_iter.peek().is_some() {
            let self_point = self_iter.next().unwrap();
            let other_point = other_iter.next().unwrap();

            dbg!("The self point is {}", self_point.clone());
            dbg!("The other point is {}", other_point.clone());
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
                value: ZipResult::Both((&self_point.value, &other_point.value))
            };

            dbg!("The intersection is {}", intersection.clone());

            // if t1x0 == self_point.t1, then it means t1 is before t2 and the value exists only in self time line.
            let left_ex = if t1x0 == self_point.t1 {
                 TimeLinePoint {
                    t1: t1x0,
                    t2: Some(t1x1),
                    value: ZipResult::Singleton(&self_point.value)
                }
            } else {
                // if t1x0 == other_point.t1, then it means t1 is before t2 and the value exists only in self time line.
                TimeLinePoint {
                    t1: t1x0,
                    t2: Some(t1x1),
                    value: ZipResult::Singleton(&other_point.value)
                }
            };

            dbg!("The left_ex is {}", left_ex.clone());

            match t2x0 {
                Some(t2x0) => {
                    if Some(t2x0) == self_point.t2 {
                        let right_ex = TimeLinePoint {
                            t1: t2x0,
                            t2: t2x1,
                            value: ZipResult::Singleton(&other_point.value)
                        };
                        dbg!("The right_ex is {}", right_ex.clone());

                        flattened_time_line_points.push(right_ex);
                    } else {
                        let right_ex = TimeLinePoint {
                            t1: t2x0,
                            t2: t2x1,
                            value: ZipResult::Singleton(&self_point.value)
                        };

                        dbg!("The right_ex is {}", right_ex.clone());
                        flattened_time_line_points.push(right_ex);
                    }
                }
                None => {}
            }

            flattened_time_line_points.push(intersection);
            flattened_time_line_points.push(left_ex);

            flattened_time_line_points.sort_by(|a, b| a.t1.cmp(&b.t1));

            let mut merged_timeline_points = vec![];

            for time_line_points in flattened_time_line_points.windows(2) {
                let left = &time_line_points.get(0);
                let right = &time_line_points.get(1);

                match (left, right) {
                    (Some(left), Some(right)) => {
                        if left.t1 == right.t1 && left.t2 == right.t2 {
                            let time_line_point = TimeLinePoint {
                                t1: left.t1,
                                t2: left.t2,
                                value: f(&left.value.combine(&right.value))
                            };
                            merged_timeline_points.push(time_line_point);
                        } else {
                            let time_line_point_left = TimeLinePoint {
                                t1: left.t1,
                                t2: left.t2,
                                value: f(&left.value)
                            };

                            let time_line_point_right = TimeLinePoint {
                                t1: right.t1,
                                t2: right.t2,
                                value: f(&right.value)
                            };

                            merged_timeline_points.push(time_line_point_left);
                            merged_timeline_points.push(time_line_point_right);
                        }
                    }
                    _ => {}
                }
            }
        }

        TimeLine {
            points: flattened_time_line_points
        }
    }

    // In a state dynamic timeline, the value is valid from t1 to t2
  pub fn add_state_dynamic_info(&mut self, start_time: u64, value: T) -> &mut TimeLine<T> {
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
    fn test_zip_with_simple() {
        let mut timeline1 = TimeLine::default();
        timeline1.add_state_dynamic_info(5, Value::StringValue("playing".to_string()));
        //timeline1.add_state_dynamic_info(1708997980, Value::StringValue("pause".to_string()));

        let mut timeline2 = TimeLine::default();
        timeline2.add_state_dynamic_info(7, Value::StringValue("movie".to_string()));
        //timeline2.add_state_dynamic_info(1708997979, Value::StringValue("cartoon".to_string()));


        // 77 to None "playing" "movie"
        // 75 to 77 "playing"

        // 75 to 77 "playing" (unknown)   (correct)
        // 77 to 79 "playing" "movie"     (correct)
        // 79 to 80 "playing" "cartoon"
        // 80 to Noe "pause" "cartoon"    (correct)
        let result1 = timeline1.zip_with(&timeline2, |a| {
            match a {
                ZipResult::Both((a, b)) => {
                    Value::ArrayValue(vec![**a, **b])
                }
                ZipResult::Singleton(a) => {
                    Value::ArrayValue(vec![**a])
                }
            }
        });


        let result2 = timeline2.zip_with(&timeline1, |a| {
            match a {
                ZipResult::Both((a, b)) => {
                    Value::ArrayValue(vec![**b, **a])
                }
                ZipResult::Singleton(a) => {
                    Value::ArrayValue(vec![**a])
                }
            }
        });

        let expected = TimeLine { points: vec![
            // From 77 to future, they are playing a movie
            TimeLinePoint {
                t1: 7,
                t2: None,
                value: Value::ArrayValue(
                    vec![
                        Value::StringValue(
                            "playing".to_string(),
                        ),
                        Value::StringValue(
                            "movie".to_string(),
                        ),
                    ],
                ),
            },
            TimeLinePoint {
                // From 75 to 77, they are playing something, but no information about whethr it was a movie or not
                t1: 5,
                t2: Some(
                    7,
                ),
                value: Value::ArrayValue(
                    vec![
                        Value::StringValue(
                            "playing".to_string(),
                        ),
                    ],
                ),
            },
        ]};

        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }

    //  t1-----------t3~~~~~~~~~~~~~~~~~~
    //            t2-----------t4~~~~~~~~~~~~~~
    #[test]
    fn test_zip_with_complex() {
        let mut timeline1 = TimeLine::default();
        timeline1.add_state_dynamic_info(5, Value::StringValue("playing".to_string()));
        timeline1.add_state_dynamic_info(8, Value::StringValue("pause".to_string()));

        let mut timeline2 = TimeLine::default();
        timeline2.add_state_dynamic_info(7, Value::StringValue("movie".to_string()));
        timeline2.add_state_dynamic_info(9, Value::StringValue("cartoon".to_string()));


        // 5 to 7 "playing"
        // 7 to 8 "playing" "movie"
        // 8 to 9 "pause" "movie"
        let result = timeline2.zip_with(&timeline1, |a| {
            match a {
                ZipResult::Both((a, b)) => {
                    Value::ArrayValue(vec![**b, **a])
                }
                ZipResult::Singleton(a) => {
                    Value::ArrayValue(vec![**a])
                }
            }
        });

        let expected = TimeLine { points: vec![
            // From 77 to future, they are playing a movie
            TimeLinePoint {
                t1: 7,
                t2: None,
                value: Value::ArrayValue(
                    vec![
                        Value::StringValue(
                            "playing".to_string(),
                        ),
                        Value::StringValue(
                            "movie".to_string(),
                        ),
                    ],
                ),
            },
            TimeLinePoint {
                // From 75 to 77, they are playing something, but no information about whethr it was a movie or not
                t1: 5,
                t2: Some(
                    7,
                ),
                value: Value::ArrayValue(
                    vec![
                        Value::StringValue(
                            "playing".to_string(),
                        ),
                    ],
                ),
            },
        ]};

       // dbg!("The timeline is {}", result.clone());
        assert_eq!(result, expected);
    }
}

