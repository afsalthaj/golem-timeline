echo "Please excuse for this script! We will improve"

cargo-component clean
# Generate core stub, because core is going to be used from driver module
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen generate -s core/wit -d core-stub
# Generate event processor stub, because core and timeline processor is going to need to communicate with event-processor.
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen generate -s event-processor/wit -d event-processor-stub
# Generate timeline processor stub, because core, timeline processor  (self loop), and event-processor (cyclic dependency) is going to need to communicate with timeline-processor.
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen generate -s timeline-processor/wit -d timeline-processor-stub
# Add core stub as dependency to driver
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen add-stub-dependency --stub-wit-root core-stub/wit --dest-wit-root driver/wit --overwrite --update-cargo-toml
# Add timeline processor stub processor stub as dependency to timeline
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen add-stub-dependency --stub-wit-root timeline-processor-stub/wit --dest-wit-root timeline-processor/wit --overwrite --update-cargo-toml
# Add even processor stub as dependency to core
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen add-stub-dependency --stub-wit-root event-processor-stub/wit --dest-wit-root core/wit --overwrite --update-cargo-toml
# Add timeline processor as dependency to core
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen add-stub-dependency --stub-wit-root timeline-processor-stub/wit --dest-wit-root core/wit --overwrite --update-cargo-toml
# Add event processor stub as dependency to timeline
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen add-stub-dependency --stub-wit-root event-processor-stub/wit --dest-wit-root timeline-processor/wit --overwrite --update-cargo-toml

# This will build individual wasm files in the target
cargo-component build

# Compose these wasms
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/driver.wasm --stub-wasm target/wasm32-wasi/debug/core_stub.wasm --dest-wasm target/wasm32-wasi/debug/driver_with_core.wasm
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/core.wasm --stub-wasm target/wasm32-wasi/debug/event_processor_stub.wasm --dest-wasm target/wasm32-wasi/debug/core_with_event.wasm
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/timeline_processor.wasm --stub-wasm target/wasm32-wasi/debug/event_processor_stub.wasm --dest-wasm target/wasm32-wasi/debug/timeline_with_event.wasm
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/core_with_event.wasm --stub-wasm target/wasm32-wasi/debug/timeline_processor_stub.wasm --dest-wasm target/wasm32-wasi/debug/core_with_event_with_timeline.wasm
/Users/afsalthaj/projects/golem/target/debug/golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/timeline_with_event.wasm --stub-wasm target/wasm32-wasi/debug/timeline_processor_stub.wasm --dest-wasm target/wasm32-wasi/debug/timeline_with_event_with_timeline.wasm
