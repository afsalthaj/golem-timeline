use std::fmt::Debug;

use crate::state_dynamic_timeline::StateDynamicsTimeLine;

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
pub struct AlignedStateDynamicsTimeLine<T> {
    pub time_line1: StateDynamicsTimeLine<T>,
    pub time_line2: StateDynamicsTimeLine<T>,
    pub removed_points_timeline1: Option<StateDynamicsTimeLine<T>>,
    pub removed_points_timeline2: Option<StateDynamicsTimeLine<T>>,
}

impl<T: Clone + Debug + PartialEq + PartialOrd> AlignedStateDynamicsTimeLine<T> {
    pub fn from_left_and_right(
        left: &mut StateDynamicsTimeLine<T>,
        right: &mut StateDynamicsTimeLine<T>,
    ) -> AlignedStateDynamicsTimeLine<T> {
        if &left.beginning() <= &right.beginning() {
            let boundary = right.beginning().map_or(None, |t| left.boundary(t));

            let new_points = boundary.map_or(left.points.clone(), |boundary| {
                left.points.split_off(&boundary)
            });

            AlignedStateDynamicsTimeLine {
                time_line1: StateDynamicsTimeLine { points: new_points },
                time_line2: right.clone(),
                removed_points_timeline1: Some(StateDynamicsTimeLine {
                    points: left.points.clone(),
                }),
                removed_points_timeline2: None,
            }
        } else {
            let boundary = left.beginning().map_or(None, |t| right.boundary(t));

            let new_points = boundary.map_or(right.points.clone(), |boundary| {
                right.points.split_off(&boundary)
            });

            AlignedStateDynamicsTimeLine {
                time_line1: left.clone(),
                time_line2: StateDynamicsTimeLine { points: new_points },
                removed_points_timeline1: None,
                removed_points_timeline2: Some(StateDynamicsTimeLine {
                    points: right.points.clone(),
                }),
            }
        }
    }
}
