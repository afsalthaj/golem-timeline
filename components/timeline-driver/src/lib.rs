mod bindings;
mod conversions;
mod builder;

use crate::bindings::exports::timeline::driver_exports::api::{Guest, WorkerDetails};
use std::cell::RefCell;
use timeline_lib::*;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::core_client::core_client;
use crate::conversions::Conversion;

/// This is one of any number of data types that our application
/// uses. Golem will take care to persist all application state,
/// whether that state is local to a function being executed or
/// global across the entire program.
struct State {
    total: u64,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        total: 0,
    });
}

struct Component;

impl Guest for Component {
    fn run(
        core_component_id: String,
        event_processor_component_id: String,
        timeline_processor_component_id: String,
    ) -> Result<WorkerDetails, String> {
        let uri =
            Uri { value: format!("urn:worker:{core_component_id}/{}", "initialize-timeline") };

        let core = core_client::Api::new(&uri);

        let simple_timeline =
            tl_equal_to(tl_latest_event_to_state(col("playerStateChange")), string_value("play"))
                .with_worker_details(
                    "cirr".to_string(),
                    event_processor_component_id,
                    timeline_processor_component_id,
                );

        let result = core.blocking_hello_world();

        match core.blocking_initialize_timeline(&simple_timeline.to_wit()) {
            Ok(result) => {
                dbg!("Driver Log: Timeline initialized");
                Ok(result)
            }
            Err(error) => {
                dbg!("Driver Log: Error initializing timeline");
                Err(error)
            }
        }


    }
}

bindings::export!(Component with_types_in bindings);
