# A temporary build script until a few issues with wasm-rpc is fixed
# where we make sure compositions
cargo component build
cargo make compose-timeline-and-event-with-timeline-processor