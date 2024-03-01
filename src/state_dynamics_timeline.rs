use crate::aligned_state_dynamic_timeline::AlignedStateDynamicsTimeLine;
use crate::boundaries::Boundaries;
use crate::event_predicate::string;
use crate::event_predicate::EventPredicate;
use crate::event_timeline::EventTimeLine;
use crate::state_dynamic_timeline_point::StateDynamicsTimeLinePoint;
use crate::value::Value;
use crate::zip_result::ZipResult;
use std::fmt::Debug;
use std::ops::Neg;

#[derive(Clone, Debug, PartialEq)]
pub struct StateDynamicsTimeLine<T> {
    // we dont use any backend here, but a mere state of the timeline.
    // Flushing of this vector can involve storing it to postgres if needed
    pub points: Vec<StateDynamicsTimeLinePoint<T>>,
}

impl<T> Default for StateDynamicsTimeLine<T> {
    fn default() -> Self {
        StateDynamicsTimeLine { points: vec![] }
    }
}

impl StateDynamicsTimeLine<bool> {
    pub fn negate(&self) -> StateDynamicsTimeLine<bool> {
        let mut negated_timeline = StateDynamicsTimeLine::default();

        for point in &self.points {
            let negated_value = !point.value;
            negated_timeline.add_state_dynamic_info(point.t1, negated_value);
        }

        negated_timeline
    }

    pub fn tl_duration_where(&self) -> EventTimeLine<u64> {
        let mut event_time_line = EventTimeLine::new();
        let mut duration = 0;

        // 1: false
        for point in &self.points {
            dbg! {point};
            let start = point.t1;
            let end = point.t2;

            match end {
                Some(end) => {
                    if point.value {
                        let mut i = 0;
                        while (start + i) < end {
                            event_time_line.add_event_info(start + i, duration);
                            duration = duration + 1;
                            i += 1;
                        }
                    } else {
                        let mut i = 0;
                        while (start + i) < end {
                            event_time_line.add_event_info(start + i, duration);
                            i += 1;
                        }
                    }
                }

                None => {
                    if point.value {
                        event_time_line.add_event_info(start, duration);
                    }
                }
            }
        }

        event_time_line
    }

    pub fn and(&self, that: StateDynamicsTimeLine<bool>) -> StateDynamicsTimeLine<bool> {
        self.zip_with(&that, |a| match a {
            ZipResult::Both((a, b)) => **a && **b,
            ZipResult::Singleton(a) => **a,
        })
    }

    pub fn or(&self, that: StateDynamicsTimeLine<bool>) -> StateDynamicsTimeLine<bool> {
        self.zip_with(&that, |a| match a {
            ZipResult::Both((a, b)) => **a || **b,
            ZipResult::Singleton(a) => **a,
        })
    }
}

impl<T: Debug + Clone + Eq + Ord> StateDynamicsTimeLine<T> {
    // This turned out to be a mere conversion of events to state
    pub fn tl_latest_event_to_state(
        event_time_line: &EventTimeLine<T>,
    ) -> StateDynamicsTimeLine<T> {
        let mut state_dynamics_time_line = StateDynamicsTimeLine::default();

        for point in &event_time_line.points {
            state_dynamics_time_line.add_state_dynamic_info(point.t1, point.value.clone());
        }

        state_dynamics_time_line
    }

    pub fn tl_has_existed(
        event_time_line: &EventTimeLine<T>,
        event_predicate: EventPredicate<T>,
    ) -> StateDynamicsTimeLine<bool> {
        let mut state_dynamics_time_line = StateDynamicsTimeLine::default();

        for event_time_line_point in &event_time_line.points {
            if event_predicate.evaluate(&event_time_line_point.value) {
                state_dynamics_time_line.add_state_dynamic_info(event_time_line_point.t1, true);
                break;
            } else {
                state_dynamics_time_line.add_state_dynamic_info(event_time_line_point.t1, false);
            }
        }

        state_dynamics_time_line
    }

    pub fn tl_has_existed_within(
        event_time_line: &EventTimeLine<T>,
        event_predicate: EventPredicate<T>,
        seconds: u64,
    ) -> StateDynamicsTimeLine<bool> {
        let mut state_dynamics_time_line = StateDynamicsTimeLine::default();

        let mut previous_true_point: Option<u64> = None;

        for event_time_line_point in &event_time_line.points {
            let is_predicate_true = event_predicate.evaluate(&event_time_line_point.value);

            match previous_true_point {
                Some(t) if event_time_line_point.t1 > t + seconds => {
                    state_dynamics_time_line.add_state_dynamic_info(t + seconds, false);
                    previous_true_point = None;
                }
                _ => {}
            }

            if is_predicate_true {
                state_dynamics_time_line.add_state_dynamic_info(event_time_line_point.t1, true);
                previous_true_point = Some(event_time_line_point.t1);
            } else {
                state_dynamics_time_line.add_state_dynamic_info(event_time_line_point.t1, false);
                previous_true_point = None;
            }
        }

        // If the last known value is a true, then add an extra data point that expires at t + seconds
        match previous_true_point {
            Some(t) => {
                state_dynamics_time_line.add_state_dynamic_info(t + seconds, false);
            }
            None => {}
        }

        state_dynamics_time_line
    }

    pub fn beginning(&self) -> Option<u64> {
        self.points.first().map(|point| point.t1)
    }

    pub fn equal_to(&self, constant: T) -> StateDynamicsTimeLine<bool> {
        let mut state_dynamics_time_line = StateDynamicsTimeLine::default();

        for point in &self.points {
            let is_equal = point.value == constant;
            state_dynamics_time_line.add_state_dynamic_info(point.t1, is_equal);
        }

        state_dynamics_time_line
    }

    pub fn greater_than(&self, constant: T) -> StateDynamicsTimeLine<bool> {
        let mut state_dynamics_time_line = StateDynamicsTimeLine::default();

        for point in &self.points {
            let is_greater_than = point.value > constant;
            state_dynamics_time_line.add_state_dynamic_info(point.t1, is_greater_than);
        }

        state_dynamics_time_line
    }

    pub fn less_than(&self, constant: T) -> StateDynamicsTimeLine<bool> {
        let mut state_dynamics_time_line = StateDynamicsTimeLine::default();

        for point in &self.points {
            let is_less_than = point.value < constant;
            state_dynamics_time_line.add_state_dynamic_info(point.t1, is_less_than);
        }

        state_dynamics_time_line
    }

    pub fn zip_with<F>(&self, other: &StateDynamicsTimeLine<T>, f: F) -> StateDynamicsTimeLine<T>
    where
        F: Fn(&ZipResult<T>) -> T,
    {
        let mut flattened_time_line_points: Vec<StateDynamicsTimeLinePoint<ZipResult<T>>> =
            Vec::new();
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
                .collect::<Vec<StateDynamicsTimeLinePoint<ZipResult<T>>>>();

            flattened_time_line_points.extend(zipped_result);
        }

        if let Some(removed_time_lines) = &aligned_time_lines.removed_points_timeline2 {
            let zipped_result = removed_time_lines
                .points
                .iter()
                .map(|point| point.to_zip_result())
                .collect::<Vec<StateDynamicsTimeLinePoint<ZipResult<T>>>>();

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
    pub fn add_state_dynamic_info(
        &mut self,
        start_time: u64,
        value: T,
    ) -> &mut StateDynamicsTimeLine<T> {
        if self.points.is_empty() {
            self.points.push(StateDynamicsTimeLinePoint {
                // epoch starting time
                t1: start_time,
                t2: None,
                value,
            });
            self
        } else {
            self.points.last_mut().unwrap().update_t2(start_time);
            self.points.push(StateDynamicsTimeLinePoint {
                t1: start_time,
                t2: None,
                value,
            });

            self
        }
    }
}

fn merge_result<F, T: Clone>(
    flattened_time_line_points: &Vec<StateDynamicsTimeLinePoint<ZipResult<T>>>,
    f: F,
) -> Vec<StateDynamicsTimeLinePoint<T>>
where
    F: Fn(&ZipResult<T>) -> T,
{
    let mut merged_timeline_points: Vec<StateDynamicsTimeLinePoint<T>> = vec![];

    for current_timeline in flattened_time_line_points.iter() {
        let last_merged_timeline_points = merged_timeline_points.last_mut();

        match last_merged_timeline_points {
            Some(last) => {
                if last.t1 == current_timeline.t1 && last.t2 == current_timeline.t2 {
                    let time_line_point = StateDynamicsTimeLinePoint {
                        t1: current_timeline.t1,
                        t2: current_timeline.t2,
                        value: f(
                            &ZipResult::Singleton(&last.value).combine(&current_timeline.value)
                        ),
                    };

                    *last = time_line_point;
                } else {
                    let current_time_line_evaluated = StateDynamicsTimeLinePoint {
                        t1: current_timeline.t1,
                        t2: current_timeline.t2,
                        value: f(&current_timeline.value),
                    };

                    merged_timeline_points.push(current_time_line_evaluated.clone());
                }
            }
            None => {
                let current_time_line_evaluated = StateDynamicsTimeLinePoint {
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
    use crate::event_predicate;
    use crate::event_predicate::EventValue;
    use crate::event_timeline::EventTimeLinePoint;
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
                StateDynamicsTimeLinePoint {
                    t1: 5,
                    t2: Some(7),
                    value: Value::ArrayValue(vec![Value::StringValue("playing".to_string())]),
                },
                StateDynamicsTimeLinePoint {
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
                StateDynamicsTimeLinePoint {
                    t1: 5,
                    t2: Some(7),
                    value: Value::ArrayValue(vec![Value::StringValue("playing".to_string())]),
                },
                StateDynamicsTimeLinePoint {
                    t1: 7,
                    t2: Some(8),
                    value: Value::ArrayValue(vec![
                        Value::StringValue("movie".to_string()),
                        Value::StringValue("playing".to_string()),
                    ]),
                },
                StateDynamicsTimeLinePoint {
                    t1: 8,
                    t2: Some(9),
                    value: Value::ArrayValue(vec![
                        Value::StringValue("movie".to_string()),
                        Value::StringValue("pause".to_string()),
                    ]),
                },
                StateDynamicsTimeLinePoint {
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
                StateDynamicsTimeLinePoint {
                    t1: 1,
                    t2: Some(2),
                    value: Value::ArrayValue(vec![Value::StringValue("playing".to_string())]),
                },
                StateDynamicsTimeLinePoint {
                    t1: 2,
                    t2: Some(3),
                    value: Value::ArrayValue(vec![
                        Value::StringValue("movie".to_string()),
                        Value::StringValue("playing".to_string()),
                    ]),
                },
                StateDynamicsTimeLinePoint {
                    t1: 3,
                    t2: Some(4),
                    value: Value::ArrayValue(vec![
                        Value::StringValue("playing".to_string()),
                        Value::StringValue("cartoon".to_string()),
                    ]),
                },
                StateDynamicsTimeLinePoint {
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
                StateDynamicsTimeLinePoint {
                    t1: 1,
                    t2: Some(2),
                    value: Value::ArrayValue(vec![Value::StringValue("pause".to_string())]),
                },
                StateDynamicsTimeLinePoint {
                    t1: 2,
                    t2: Some(3),
                    value: Value::ArrayValue(vec![Value::StringValue("playing".to_string())]),
                },
                StateDynamicsTimeLinePoint {
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

    #[test]
    fn test_tl_has_existed() {
        let mut event_time_line = EventTimeLine::new();
        event_time_line.add_event_info(1, "pause".to_string());
        event_time_line.add_event_info(2, "playing".to_string());
        event_time_line.add_event_info(3, "pause".to_string());

        let event_predicate = event_predicate::col("event").equal_to::<String>(string("playing"));

        let result = StateDynamicsTimeLine::tl_has_existed(&event_time_line, event_predicate);

        let expected = StateDynamicsTimeLine {
            points: vec![
                StateDynamicsTimeLinePoint {
                    t1: 1,
                    t2: Some(2),
                    value: false,
                },
                StateDynamicsTimeLinePoint {
                    t1: 2,
                    t2: None,
                    value: true,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    // Input t1------(pause)-----t2~~~~~(playing)~~~~~>
    // Expiration time: 1 seconds, predicate: if playing
    // t1------(false)-----t2----(true)-------t3~~~(false)~~~~>
    #[test]
    fn test_tl_has_existed_within_scenario1() {
        let mut event_time_line = EventTimeLine::new();
        event_time_line.add_event_info(1, "pause".to_string());
        event_time_line.add_event_info(2, "playing".to_string());

        let event_predicate = event_predicate::col("event").equal_to::<String>(string("playing"));

        let result =
            StateDynamicsTimeLine::tl_has_existed_within(&event_time_line, event_predicate, 1);

        let expected = StateDynamicsTimeLine {
            points: vec![
                StateDynamicsTimeLinePoint {
                    t1: 1,
                    t2: Some(2),
                    value: false,
                },
                StateDynamicsTimeLinePoint {
                    t1: 2,
                    t2: Some(3),
                    value: true,
                },
                StateDynamicsTimeLinePoint {
                    t1: 3,
                    t2: None,
                    value: false,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    // Input t1------(pause)-----t2----------(playing)------t5~~~~~~(playing)~~~~>
    // Expiration time: 2 seconds, predicate: if playing
    // t1--------(false)-----t2----(true)---t4----(false)---t5----(true)---t7~~~~(false)~~~~~~>
    #[test]
    fn test_tl_has_existed_within_scenario2() {
        let mut event_time_line = EventTimeLine::new();
        event_time_line.add_event_info(1, "pause".to_string());
        event_time_line.add_event_info(2, "playing".to_string());
        event_time_line.add_event_info(5, "playing".to_string());

        let event_predicate = event_predicate::col("event").equal_to::<String>(string("playing"));

        let result =
            StateDynamicsTimeLine::tl_has_existed_within(&event_time_line, event_predicate, 2);

        let expected = StateDynamicsTimeLine {
            points: vec![
                StateDynamicsTimeLinePoint {
                    t1: 1,
                    t2: Some(2),
                    value: false,
                },
                StateDynamicsTimeLinePoint {
                    t1: 2,
                    t2: Some(4),
                    value: true,
                },
                StateDynamicsTimeLinePoint {
                    t1: 4,
                    t2: Some(5),
                    value: false,
                },
                StateDynamicsTimeLinePoint {
                    t1: 5,
                    t2: Some(7),
                    value: true,
                },
                StateDynamicsTimeLinePoint {
                    t1: 7,
                    t2: None,
                    value: false,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_negatable() {
        let mut timeline = StateDynamicsTimeLine::default();
        timeline.add_state_dynamic_info(1, true);
        timeline.add_state_dynamic_info(2, false);
        timeline.add_state_dynamic_info(3, true);

        let result = timeline.negate();

        let expected = StateDynamicsTimeLine {
            points: vec![
                StateDynamicsTimeLinePoint {
                    t1: 1,
                    t2: Some(2),
                    value: false,
                },
                StateDynamicsTimeLinePoint {
                    t1: 2,
                    t2: Some(3),
                    value: true,
                },
                StateDynamicsTimeLinePoint {
                    t1: 3,
                    t2: None,
                    value: false,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn tl_duration_where() {
        let mut timeline = StateDynamicsTimeLine::default();
        timeline.add_state_dynamic_info(1, true);
        timeline.add_state_dynamic_info(3, false);
        timeline.add_state_dynamic_info(5, true);
        timeline.add_state_dynamic_info(7, true);
        let result = timeline.tl_duration_where();

        let expected = EventTimeLine {
            points: vec![
                EventTimeLinePoint { t1: 1, value: 0 },
                EventTimeLinePoint { t1: 2, value: 1 },
                EventTimeLinePoint { t1: 3, value: 2 },
                EventTimeLinePoint { t1: 4, value: 2 },
                EventTimeLinePoint { t1: 5, value: 2 },
                EventTimeLinePoint { t1: 6, value: 3 },
                EventTimeLinePoint { t1: 7, value: 4 },
            ],
        };

        assert_eq!(result, expected);
    }
}
