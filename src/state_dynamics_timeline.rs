use crate::aligned_state_dynamic_timeline::AlignedStateDynamicsTimeLine;
use crate::boundaries::Boundaries;
use crate::state_dynamic_timeline_point::StateDynamicsTimeLineSlice;
use crate::value::Value;
use crate::zip_result::ZipResult;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct StateDynamicsTimeLine<T> {
    // we dont use any backend here, but a mere state of the timeline.
    // Flushing of this vector can involve storing it to postgres if needed
    pub points: Vec<StateDynamicsTimeLineSlice<T>>,
}

impl<T> Default for StateDynamicsTimeLine<T> {
    fn default() -> Self {
        StateDynamicsTimeLine { points: vec![] }
    }
}

impl<T: std::fmt::Debug + Clone> StateDynamicsTimeLine<T> {
    pub fn beginning(&self) -> Option<u64> {
        self.points.first().map(|point| point.t1)
    }

    pub fn zip_with<F>(&self, other: &StateDynamicsTimeLine<T>, f: F) -> StateDynamicsTimeLine<T>
    where
        F: Fn(&ZipResult<T>) -> T,
    {
        let mut flattened_time_line_points: Vec<StateDynamicsTimeLineSlice<ZipResult<T>>> = Vec::new();
        let mut self_cloned = self.clone();
        let mut right_cloned = other.clone();

        let aligned_time_lines =
            AlignedStateDynamicsTimeLine::from_left_and_right(&mut self_cloned, &mut right_cloned);

        let mut self_iter = aligned_time_lines.time_line1.points.iter().peekable();
        let mut other_iter = aligned_time_lines.time_line2.points.iter().peekable();

        if let Some(removed_time_lines) = &aligned_time_lines.removed_points_timeline1 {
            let zipped_result = removed_time_lines
                .points
                .iter()
                .map(|point| point.to_zip_result())
                .collect::<Vec<StateDynamicsTimeLineSlice<ZipResult<T>>>>();

            flattened_time_line_points.extend(zipped_result);
        }

        if let Some(removed_time_lines) = &aligned_time_lines.removed_points_timeline2 {
            let zipped_result = removed_time_lines
                .points
                .iter()
                .map(|point| point.to_zip_result())
                .collect::<Vec<StateDynamicsTimeLineSlice<ZipResult<T>>>>();

            flattened_time_line_points.extend(zipped_result);
        }

        while self_iter.peek().is_some() && other_iter.peek().is_some() {
            let self_point = self_iter.next().unwrap();
            let other_point = other_iter.next().unwrap();

            let Boundaries {
                left: left_ex,
                intersection,
                right: right_ex,
            } = Boundaries::get_boundaries(self_point, other_point);

            flattened_time_line_points.push(intersection);

            if let Some(left_ex) = left_ex {
                flattened_time_line_points.push(left_ex);
            }
            if let Some(right_ex) = right_ex {
                flattened_time_line_points.push(right_ex);
            }

            flattened_time_line_points.sort_by(|a, b| a.t1.cmp(&b.t1));
        }

        let merged_timeline_points = merge_result(&flattened_time_line_points, f);

        StateDynamicsTimeLine {
            points: merged_timeline_points,
        }
    }

    // In a state dynamic timeline, the value is valid from t1 to t2
    pub fn add_state_dynamic_info(&mut self, start_time: u64, value: T) -> &mut StateDynamicsTimeLine<T> {
        if self.points.is_empty() {
            self.points.push(StateDynamicsTimeLineSlice {
                // epoch starting time
                t1: start_time,
                t2: None,
                value,
            });
            self
        } else {
            self.points.last_mut().unwrap().update_t2(start_time);
            self.points.push(StateDynamicsTimeLineSlice {
                t1: start_time,
                t2: None,
                value,
            });

            self
        }
    }
}

fn merge_result<F, T: Clone>(
    flattened_time_line_points: &Vec<StateDynamicsTimeLineSlice<ZipResult<T>>>,
    f: F,
) -> Vec<StateDynamicsTimeLineSlice<T>>
where
    F: Fn(&ZipResult<T>) -> T,
{
    let mut merged_timeline_points: Vec<StateDynamicsTimeLineSlice<T>> = vec![];

    for current_timeline in flattened_time_line_points.iter() {
        let last_merged_timeline_points = merged_timeline_points.last_mut();

        match last_merged_timeline_points {
            Some(last) => {
                if last.t1 == current_timeline.t1 && last.t2 == current_timeline.t2 {
                    let time_line_point = StateDynamicsTimeLineSlice {
                        t1: current_timeline.t1,
                        t2: current_timeline.t2,
                        value: f(
                            &ZipResult::Singleton(&last.value).combine(&current_timeline.value)
                        ),
                    };

                    *last = time_line_point;
                } else {
                    let current_time_line_evaluated = StateDynamicsTimeLineSlice {
                        t1: current_timeline.t1,
                        t2: current_timeline.t2,
                        value: f(&current_timeline.value),
                    };

                    merged_timeline_points.push(current_time_line_evaluated.clone());
                }
            }
            None => {
                let current_time_line_evaluated = StateDynamicsTimeLineSlice {
                    t1: current_timeline.t1,
                    t2: current_timeline.t2,
                    value: f(&current_timeline.value),
                };

                merged_timeline_points.push(current_time_line_evaluated.clone());
            }
        }
    }

    merged_timeline_points
}

// ~~ represents `forever`
// -- denotes a finite boundary
mod tests {
    use super::*;
    use crate::value::Value;

    // t1~~~~(playing)~~~~~~~~~~~~>
    //       t2~~~~(movie)~~~~~~~~~~>
    // Expected Result:
    //   t1 - t2     : playing
    //   t2 -> future: playing a movie
    #[test]
    fn test_zip_with_scenario1() {
        let mut timeline1 = StateDynamicsTimeLine::default();
        timeline1.add_state_dynamic_info(5, Value::StringValue("playing".to_string()));

        let mut timeline2 = StateDynamicsTimeLine::default();
        timeline2.add_state_dynamic_info(7, Value::StringValue("movie".to_string()));

        let result1 = timeline1.zip_with(&timeline2, |a| match a {
            ZipResult::Both((a, b)) => {
                let a0 = a.clone().clone();
                let b0 = b.clone().clone();
                Value::ArrayValue(vec![a0, b0])
            }
            ZipResult::Singleton(a) => {
                let a0 = a.clone().clone();
                Value::ArrayValue(vec![a0])
            }
        });

        let result2 = timeline2.zip_with(&timeline1, |a| match a {
            ZipResult::Both((a, b)) => {
                let a0 = a.clone().clone();
                let b0 = b.clone().clone();
                Value::ArrayValue(vec![b0, a0])
            }
            ZipResult::Singleton(a) => {
                let a0 = a.clone().clone();
                Value::ArrayValue(vec![a0])
            }
        });

        let expected = StateDynamicsTimeLine {
            points: vec![
                StateDynamicsTimeLineSlice {
                    t1: 5,
                    t2: Some(7),
                    value: Value::ArrayValue(vec![Value::StringValue("playing".to_string())]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 7,
                    t2: None,
                    value: Value::ArrayValue(vec![
                        Value::StringValue("playing".to_string()),
                        Value::StringValue("movie".to_string()),
                    ]),
                },
            ],
        };

        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }

    // t1-------(playing)---t3~~~~(pause)~~~~~>
    //      t2----(movie)---------t4~~~~(cartoon)~~~~~>
    // Expected Result:
    //   t1 - t2    : playing
    //   t2 - t3    : playing a movie
    //   t3 - t4    : paused movie
    //   t4 - future: paused cartoon
    #[test]
    fn test_zip_with_scenario2() {
        let mut timeline1 = StateDynamicsTimeLine::default();
        timeline1.add_state_dynamic_info(5, Value::StringValue("playing".to_string()));
        timeline1.add_state_dynamic_info(8, Value::StringValue("pause".to_string()));

        let mut timeline2 = StateDynamicsTimeLine::default();
        timeline2.add_state_dynamic_info(7, Value::StringValue("movie".to_string()));
        timeline2.add_state_dynamic_info(9, Value::StringValue("cartoon".to_string()));

        let result = timeline2.zip_with(&timeline1, |a| match a {
            ZipResult::Both((a, b)) => {
                let a0 = a.clone().clone();
                let b0 = b.clone().clone();
                match (a0, b0) {
                    (Value::ArrayValue(a), Value::ArrayValue(b)) => {
                        Value::ArrayValue(a.iter().chain(b.iter()).cloned().collect())
                    }
                    (Value::ArrayValue(a), value) => {
                        Value::ArrayValue(a.iter().chain(&vec![value]).cloned().collect())
                    }
                    (value, Value::ArrayValue(b)) => {
                        Value::ArrayValue(vec![value].iter().chain(b.iter()).cloned().collect())
                    }
                    (value1, value2) => Value::ArrayValue(vec![value1, value2]),
                }
            }
            ZipResult::Singleton(a) => {
                let a0 = a.clone().clone();
                Value::ArrayValue(vec![a0])
            }
        });

        let expected = StateDynamicsTimeLine {
            points: vec![
                StateDynamicsTimeLineSlice {
                    t1: 5,
                    t2: Some(7),
                    value: Value::ArrayValue(vec![Value::StringValue("playing".to_string())]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 7,
                    t2: Some(8),
                    value: Value::ArrayValue(vec![
                        Value::StringValue("movie".to_string()),
                        Value::StringValue("playing".to_string()),
                    ]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 8,
                    t2: Some(9),
                    value: Value::ArrayValue(vec![
                        Value::StringValue("movie".to_string()),
                        Value::StringValue("pause".to_string()),
                    ]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 9,
                    t2: None,
                    value: Value::ArrayValue(vec![
                        Value::StringValue("cartoon".to_string()),
                        Value::StringValue("pause".to_string()),
                    ]),
                },
            ],
        };

        assert_eq!(result, expected);
    }

    // t1-------(playing)----------t4~~~~(pause)~~~~~>
    //      t2----(movie)---t3~~~~(cartoon)~~~~~>
    // Expected Result:
    //   t1 - t2    : playing
    //   t2 - t3    : playing a movie
    //   t3 - t4    : playing a cartoon
    //   t4 - future: paused cartoon
    #[test]
    fn test_zip_with_scenario3() {
        let mut timeline1 = StateDynamicsTimeLine::default();
        timeline1.add_state_dynamic_info(1, Value::StringValue("playing".to_string()));
        timeline1.add_state_dynamic_info(4, Value::StringValue("pause".to_string()));

        let mut timeline2 = StateDynamicsTimeLine::default();
        timeline2.add_state_dynamic_info(2, Value::StringValue("movie".to_string()));
        timeline2.add_state_dynamic_info(3, Value::StringValue("cartoon".to_string()));

        let result = timeline2.zip_with(&timeline1, |a| match a {
            ZipResult::Both((a, b)) => {
                let a0 = a.clone().clone();
                let b0 = b.clone().clone();
                match (a0, b0) {
                    (Value::ArrayValue(a), Value::ArrayValue(b)) => {
                        Value::ArrayValue(a.iter().chain(b.iter()).cloned().collect())
                    }
                    (Value::ArrayValue(a), value) => {
                        Value::ArrayValue(a.iter().chain(&vec![value]).cloned().collect())
                    }
                    (value, Value::ArrayValue(b)) => {
                        Value::ArrayValue(vec![value].iter().chain(b.iter()).cloned().collect())
                    }
                    (value1, value2) => Value::ArrayValue(vec![value1, value2]),
                }
            }
            ZipResult::Singleton(a) => {
                let a0 = a.clone().clone();
                Value::ArrayValue(vec![a0])
            }
        });

        let expected = StateDynamicsTimeLine {
            points: vec![
                StateDynamicsTimeLineSlice {
                    t1: 1,
                    t2: Some(2),
                    value: Value::ArrayValue(vec![Value::StringValue("playing".to_string())]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 2,
                    t2: Some(3),
                    value: Value::ArrayValue(vec![
                        Value::StringValue("movie".to_string()),
                        Value::StringValue("playing".to_string()),
                    ]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 3,
                    t2: Some(4),
                    value: Value::ArrayValue(vec![
                        Value::StringValue("playing".to_string()),
                        Value::StringValue("cartoon".to_string()),
                    ]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 4,
                    t2: None,
                    value: Value::ArrayValue(vec![
                        Value::StringValue("cartoon".to_string()),
                        Value::StringValue("pause".to_string()),
                    ]),
                },
            ],
        };

        assert_eq!(result, expected);
    }

    // FIX this test - this is because we don't allign timeline2 to the correct segment of timeline1
    // t1-----(pause)------t2~~~~~(playing)~~~~~>
    //                         t3~~~~~(movie)~~~~>
    // Expected Result:
    //   t1 - t2    : pause
    //   t2 - t3    : playing
    //   t2 - future: playing a movie
    #[test]
    fn test_zip_with_scenario4() {
        let mut timeline1 = StateDynamicsTimeLine::default();
        timeline1.add_state_dynamic_info(1, Value::StringValue("pause".to_string()));
        timeline1.add_state_dynamic_info(2, Value::StringValue("playing".to_string()));

        let mut timeline2 = StateDynamicsTimeLine::default();
        timeline2.add_state_dynamic_info(3, Value::StringValue("movie".to_string()));

        let result = timeline1.zip_with(&timeline2, |a| match a {
            ZipResult::Both((a, b)) => {
                let a0 = a.clone().clone();
                let b0 = b.clone().clone();
                match (a0, b0) {
                    (Value::ArrayValue(a), Value::ArrayValue(b)) => {
                        Value::ArrayValue(a.iter().chain(b.iter()).cloned().collect())
                    }
                    (Value::ArrayValue(a), value) => {
                        Value::ArrayValue(a.iter().chain(&vec![value]).cloned().collect())
                    }
                    (value, Value::ArrayValue(b)) => {
                        Value::ArrayValue(vec![value].iter().chain(b.iter()).cloned().collect())
                    }
                    (value1, value2) => Value::ArrayValue(vec![value1, value2]),
                }
            }
            ZipResult::Singleton(a) => {
                let a0 = a.clone().clone();
                Value::ArrayValue(vec![a0])
            }
        });

        let expected = StateDynamicsTimeLine {
            points: vec![
                StateDynamicsTimeLineSlice {
                    t1: 1,
                    t2: Some(2),
                    value: Value::ArrayValue(vec![Value::StringValue("pause".to_string())]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 2,
                    t2: Some(3),
                    value: Value::ArrayValue(vec![Value::StringValue("playing".to_string())]),
                },
                StateDynamicsTimeLineSlice {
                    t1: 3,
                    t2: None,
                    value: Value::ArrayValue(vec![
                        Value::StringValue("playing".to_string()),
                        Value::StringValue("movie".to_string()),
                    ]),
                },
            ],
        };

        assert_eq!(result, expected);
    }
}
