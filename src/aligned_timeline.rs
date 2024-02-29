use crate::timeline::TimeLine;
use std::fmt::Debug;

// Aligning two timelines is an important step before you start zipping timelines
// In real world, if the align returns empty timelines probably it's a good idea to wait for the
// stream to get some data in each worker to get a fuller timeline

// Input: Two misaligned timelines
// Timeline1: t1------------t2-----------t4
// Timeline2:                    t3--------------t5
// Output: Aligned timeline returns (t2 - t4) and (t3 - t5)

// Input:  Two misaligned timelines
// TimeLine1:                  t3-----------t5
// TimeLine2: t1----------t2---------t4
// Output: AlignedTimeLine returns (t3 - t5) and (t2 -t4)
pub struct AlignedTimeLine<T> {
    pub time_line1: TimeLine<T>,
    pub time_line2: TimeLine<T>,
    pub removed_points_timeline1: Option<TimeLine<T>>,
    pub removed_points_timeline2: Option<TimeLine<T>>,
}

impl<T: Clone + Debug> AlignedTimeLine<T> {
    pub fn from_left_and_right(
        left: &mut TimeLine<T>,
        right: &mut TimeLine<T>,
    ) -> AlignedTimeLine<T> {
        if &left.beginning() <= &right.beginning() {
            let mut n = 0;

            for time_line_point in &left.points {
                if !time_line_point.contains(right.beginning().unwrap()) {
                    n += 1;
                } else {
                    break;
                }
            }

            let new_points = left.points.split_off(n);

            AlignedTimeLine {
                time_line1: TimeLine { points: new_points },
                time_line2: right.clone(),
                removed_points_timeline1: Some(TimeLine {
                    points: left.points.clone(),
                }),
                removed_points_timeline2: None,
            }
        } else {
            let mut n = 0;

            for time_line_point in &right.points {
                if !time_line_point.contains(left.beginning().unwrap()) {
                    n += 1;
                } else {
                    break;
                }
            }

            let new_points = right.points.split_off(n);

            AlignedTimeLine {
                time_line1: left.clone(),
                time_line2: TimeLine { points: new_points },
                removed_points_timeline1: None,
                removed_points_timeline2: Some(TimeLine {
                    points: right.points.clone(),
                }),
            }
        }
    }
}
