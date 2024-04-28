# A temporary build script until a few issues with wasm-rpc is fixed
cargo compose build
cargo make compose-driver
cargo make compose-core-with-event-processor
cargo make compose-timeline-processor-with-event-processor
cargo make compose-timeline-and-event-with-timeline-processor

