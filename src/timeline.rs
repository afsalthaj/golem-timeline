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

impl<'t, T: Clone> ZipResult<'t, T> {
    pub fn combine(&'t self, other: &'t ZipResult<T>) -> ZipResult<T> {
       match (self, other) {
           (ZipResult::Singleton(a), ZipResult::Singleton(c)) => {
               ZipResult::Both((a, c))
           }
           _ => panic!("Cannot combine")
       }
    }
}

struct Boundaries<'t, T: Clone> {
    left: Option<TimeLinePoint<ZipResult<'t, T>>>,
    intersection: IntersectionTimeLinePoint<'t, T>,
    right: Option<TimeLinePoint<ZipResult<'t, T>>>
}

struct IntersectionTimeLinePoint<'t, T: Clone> {
    timeline: TimeLinePoint<ZipResult<'t, T>>
}

impl<'t, T: Clone> IntersectionTimeLinePoint<'t, T> {
    pub fn left_boundary(&self) -> u64 {
        self.timeline.t1
    }

    pub fn right_boundary(&self) -> Option<u64> {
        self.timeline.t2
    }

    pub fn get_intersection(left: &'t TimeLinePoint<T>, right: &'t TimeLinePoint<T>) -> IntersectionTimeLinePoint<'t, T> {
        let t1 = left.t1.max(right.t1);

        let t2 = match (left.t2, right.t2) {
            (Some(t2), None) => Some(t2),
            (None, Some(t2)) => Some(t2),
            (Some(t2), Some(t2_other)) => Some(t2.min(t2_other)),
            (None, None) => None
        };

        IntersectionTimeLinePoint {
            timeline: TimeLinePoint {
                t1,
                t2,
                value: ZipResult::Both((&left.value, &right.value))
            }
        }
    }
}

impl<T: std::fmt::Debug + Clone> TimeLine<T> {
    pub fn zip_with<F>(&self, other: &TimeLine<T>, f: F) -> TimeLine<T>
    where F: Fn(&ZipResult<T>) -> T {
        let mut flattened_time_line_points: Vec<TimeLinePoint<ZipResult<T>>> = Vec::new();
        let mut self_iter = self.points.iter().peekable();
        let mut other_iter = other.points.iter().peekable();
        let mut merged_timeline_points: Vec<TimeLinePoint<T>> = vec![];

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
                value: ZipResult::Both((&self_point.value, &other_point.value))
            };

            //dbg!("The intersection is {}", intersection.clone());

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

            //dbg!("The left_ex is {}", left_ex.clone());

            match t2x0 {
                Some(t2x0) => {
                    if Some(t2x0) == self_point.t2 {
                        let right_ex = TimeLinePoint {
                            t1: t2x0,
                            t2: t2x1,
                            value: ZipResult::Singleton(&other_point.value)
                        };
                       // dbg!("The right_ex is {}", right_ex.clone());

                        flattened_time_line_points.push(right_ex);
                    } else {
                        let right_ex = TimeLinePoint {
                            t1: t2x0,
                            t2: t2x1,
                            value: ZipResult::Singleton(&self_point.value)
                        };

                        //dbg!("The right_ex is {}", right_ex.clone());
                        flattened_time_line_points.push(right_ex);
                    }
                }
                None => {}
            }

            flattened_time_line_points.push(intersection);
            flattened_time_line_points.push(left_ex);

            flattened_time_line_points.sort_by(|a, b| a.t1.cmp(&b.t1));
        }

       // dbg!("The flattened time line points are {:?}", flattened_time_line_points.clone());

        for current_timeline in flattened_time_line_points.iter() {
            let last_merged_timeline_points = merged_timeline_points.last_mut();

            match last_merged_timeline_points {
                Some(last) => {

                    if last.t1 == current_timeline.t1 && last.t2 == current_timeline.t2 {
                        dbg!("The last is {:?}", last.clone());
                        dbg!("the current is {:?}", current_timeline.clone());

                        let time_line_point = TimeLinePoint {
                            t1: current_timeline.t1,
                            t2: current_timeline.t2,
                            value: f(&ZipResult::Singleton(&last.value).combine(&current_timeline.value))
                        };

                        dbg!("The result is {}", &time_line_point.clone());
                        *last = time_line_point;
                    } else {
                        let current_time_line_evaluated = TimeLinePoint {
                            t1: current_timeline.t1,
                            t2: current_timeline.t2,
                            value: f(&current_timeline.value)
                        };

                        merged_timeline_points.push(current_time_line_evaluated.clone());
                    }
                }
                None => {
                    let current_time_line_evaluated = TimeLinePoint {
                        t1: current_timeline.t1,
                        t2: current_timeline.t2,
                        value: f(&current_timeline.value)
                    };

                    merged_timeline_points.push(current_time_line_evaluated.clone());

                }
            }
        }

        TimeLine {
            points: merged_timeline_points
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
                    let a0 = a.clone().clone();
                    let b0 = a.clone().clone();
                    Value::ArrayValue(vec![a0, b0])
                }
                ZipResult::Singleton(a) => {
                    let a0 = a.clone().clone();
                    Value::ArrayValue(vec![a0])
                }
            }
        });


        let result2 = timeline2.zip_with(&timeline1, |a| {
            match a {
                ZipResult::Both((a, b)) => {
                    let a0 = a.clone().clone();
                    let b0 = a.clone().clone();
                    Value::ArrayValue(vec![b0, a0])
                }
                ZipResult::Singleton(a) => {
                    let a0 = a.clone().clone();
                    Value::ArrayValue(vec![a0])
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
                    let a0 = a.clone().clone();
                    let b0 = b.clone().clone();
                    Value::ArrayValue(vec![a0, b0])
                }
                ZipResult::Singleton(a) => {
                    let a0 = a.clone().clone();
                    Value::ArrayValue(vec![a0])
                }
            }
        });

        let expected = TimeLine {
            points: vec![
                TimeLinePoint {
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
                TimeLinePoint {
                    t1: 7,
                    t2: Some(
                        8,
                    ),
                    value: Value::ArrayValue(
                        vec![
                            Value::StringValue(
                                "movie".to_string(),
                            ),
                            Value::StringValue(
                                "playing".to_string(),
                            ),
                        ],
                    ),
                },
                TimeLinePoint {
                    t1: 8,
                    t2: Some(
                        9,
                    ),
                    value: Value::ArrayValue(
                        vec![
                            Value::ArrayValue(
                                vec![
                                    Value::StringValue(
                                        "movie".to_string(),
                                    ),
                                ],
                            ),
                            Value::StringValue(
                                "pause".to_string(),
                            ),
                        ],
                    ),
                },
                TimeLinePoint {
                    t1: 9,
                    t2: None,
                    value: Value::ArrayValue(
                        vec![
                            Value::StringValue(
                                "cartoon".to_string(),
                            ),
                            Value::StringValue(
                                "pause".to_string(),
                            ),
                        ],
                    ),
                },
            ],
        };

        dbg!("The timeline is {}", result.clone());
        assert_eq!(result, expected);
    }
}

