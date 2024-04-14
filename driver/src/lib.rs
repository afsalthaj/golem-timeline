use crate::bindings::exports::timeline::rawevents::api::{Guest, TimelineOp as WitTimelineOp};
use timeline_lib::timeline_op::TimeLineOp as GolemTimeLineOp;

mod bindings;
struct Component;

impl Guest for Component {
    fn get_timelines() -> Vec<String> {
        todo!()
    }

    fn initialize_timeline(timeline: WitTimelineOp) {
        let timeline: GolemTimeLineOp = timeline.into();
    }
}