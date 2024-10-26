use crate::bindings::exports::timeline::driver::api::Guest;
use crate::bindings::golem::rpc::types::Uri;

use crate::bindings::timeline::core::api::WorkerDetails;

use crate::bindings::timeline::core_stub::stub_core;

use conversions::Conversion;
use timeline::*;

#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;

mod builder;
mod conversions;
struct Component;

impl Guest for Component {
    fn run(
        core_component_id: String,
        event_processor_component_id: String,
        timeline_processor_component_id: String,
    ) -> Result<WorkerDetails, String> {
        let uri = Uri { value: format!("urn:worker:{core_component_id}/{}", "initialize-timeline") };

        let core = stub_core::Api::new(&uri);

        // let cirr = tl_and(
        //     tl_and(
        //         tl_and(
        //             tl_equal_to(
        //                 tl_latest_event_to_state(col("playerStateChange")),
        //                 string_value("buffer"),
        //             ),
        //             tl_has_existed(col("playerStateChange").equal_to(string("play"))),
        //         ),
        //         tl_not(tl_has_existed_within(col("userAction").equal_to(string("seek")), 5)),
        //     ),
        //     tl_equal_to(tl_latest_event_to_state(col("cdnChange")), string_value("CDN1")),
        // )
        // .with_worker_details(
        //     "cirr".to_string(),
        //     event_processor_component_id,
        //     timeline_processor_component_id,
        // );

        let simple_timeline =
            tl_equal_to(tl_latest_event_to_state(col("playerStateChange")), string_value("play"))
                .with_worker_details(
                    "cirr".to_string(),
                    event_processor_component_id,
                    timeline_processor_component_id,
                );

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
