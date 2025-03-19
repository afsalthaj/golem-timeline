mod bindings;
mod builder;
mod conversions;

use crate::bindings::exports::timeline::driver_exports::api::{Guest, WorkerDetails};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::timeline::core_client::core_client;
use crate::bindings::timeline::core_client::core_client::GolemRpcUri;
use crate::conversions::Conversion;
use golem_rust::bindings::golem::api::host::{resolve_worker_id, worker_uri};
use std::cell::RefCell;
use timeline_lib::*;

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
    fn run() -> Result<WorkerDetails, String> {
        let worker_name = TimeLineWorkerName("initialize-timeline".to_string());

        let worker_id = resolve_worker_id("timeline:core", &worker_name.0)
            .expect("Failed to resolve worker id");

        let uri = worker_uri(&worker_id);

        let core = core_client::Api::new(&GolemRpcUri { value: uri.value });

        let simple_timeline =
            tl_equal_to(tl_latest_event_to_state(col("playerStateChange")), string_value("play"))
                .with_worker_details(
                    "cirr".to_string(),
                    "timeline:event-processor".to_string(),
                    "timeline:timeline-processor".to_string(),
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
