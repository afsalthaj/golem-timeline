echo "Please excuse for this script! We will improve"

# This will build individual wasm files in the target
cargo-component build
# This is basically composing the generated wasms which is the real components in action for golem-timeline
golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/driver.wasm --stub-wasm target/wasm32-wasi/debug/core_stub.wasm --dest-wasm target/wasm32-wasi/debug/driver_with_core.wasm
golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/core.wasm --stub-wasm target/wasm32-wasi/debug/event_processor_stub.wasm --dest-wasm target/wasm32-wasi/debug/core_with_event.wasm
golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/timeline_processor.wasm --stub-wasm target/wasm32-wasi/debug/event_processor_stub.wasm --dest-wasm target/wasm32-wasi/debug/timeline_with_event.wasm
golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/core_with_event.wasm --stub-wasm target/wasm32-wasi/debug/timeline_processor_stub.wasm --dest-wasm target/wasm32-wasi/debug/core_with_event_with_timeline.wasm
golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/timeline_with_event.wasm --stub-wasm target/wasm32-wasi/debug/timeline_processor_stub.wasm --dest-wasm target/wasm32-wasi/debug/timeline_with_event_with_timeline.wasm
