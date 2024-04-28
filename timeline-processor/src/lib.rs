use timeline::event_predicate::EventColumnName;
use timeline::golem_event::GolemEventValue;
use timeline::state_dynamic_timeline::StateDynamicsTimeLine;
use crate::bindings::exports::timeline::timeline_processor::api::{EventValue, Guest, TypedTimelineResultWorker};
use crate::bindings::timeline::event_processor_stub::stub_event_processor;
//use crate::bindings::timeline::timeline_processor_stub::stub_timeline_processor;
use crate::bindings::golem::rpc::types::Uri;

mod bindings;

struct Component;

struct TLEqual {
    child_worker: TypedTimelineResultWorker,
}

struct TLGreaterThan {
    child_worker: TypedTimelineResultWorker,
}

struct TLGreaterThanOrEqualTo {
    child_worker: TypedTimelineResultWorker,
}

struct TLLessThan {
    child_worker: TypedTimelineResultWorker,
}

struct TLLessThanOrEqualTo {
    child_worker: TypedTimelineResultWorker,
}

struct TLAnd {
    child_worker1: TypedTimelineResultWorker,
    child_worker2: TypedTimelineResultWorker,
}

struct TLOr {
    child_worker1: TypedTimelineResultWorker,
    child_worker2: TypedTimelineResultWorker,
}


thread_local! {

}

impl Guest for Component {
    fn initialize_equal(child_url: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {

        let uri = Uri {
            value: format!("worker://some_template/{}", "some_worker"),
        };

        let core = stub_event_processor::Api::new(&uri);

        core.tl_has_existed(
           1
        )?;

        let uri = Uri {
            value: format!("worker://some_template/{}", "some_worker"),
        };

        // let core = stub_timeline_processor::Api::new(&uri);
        //
        // core.initialize_equal(
        //     &child_url,
        //     &current_worker_id,
        //     &event_value
        // )?;

        Ok("Successfully initiated the worker to compute equals".to_string())
    }

    fn initialize_greater_than(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        todo!()
    }

    fn initialize_greater_than_or_equal_to(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        todo!()
    }

    fn initialize_less_than(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        todo!()
    }

    fn initialize_less_than_or_equal_to(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        todo!()
    }

    fn initialize_and(child_worker1: TypedTimelineResultWorker, child_worker2: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        todo!()
    }

    fn initialize_or(child_worker1: TypedTimelineResultWorker, child_worker2: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        todo!()
    }

    fn initialize_not(child_worker: TypedTimelineResultWorker, event_value: EventValue) -> Result<String, String> {
        todo!()
    }
}