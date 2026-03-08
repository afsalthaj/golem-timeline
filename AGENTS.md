# AGENTS.md

Durable timeline analytics on [Golem](https://learn.golem.cloud) — a push-based agent graph that processes temporal event streams in real time, built on WebAssembly.

## Build and test

- Build: `golem build` (requires `wasm32-wasip1` target and Golem CLI 1.4.1+)
- Test: `cargo make test` (requires Docker for Kafka)
- Check compilation: `cargo check --workspace`

## Architecture

- `components-rust/timeline-core/` — the single WASM component deployed to Golem, contains all four agent types (TimelineDriver, EventProcessor, TimelineProcessor, Aggregator)
- `common-rust/common-lib/` — shared domain types (`TimeLineOp`, `GolemEventPredicate`, `StateDynamicsTimeLine`)
- `services/` — standalone binaries reusable for cloud deployments (event-generator, kafka-producer, kafka-consumer-feeder)
- `test/integration-harness/` — integration tests that invoke Golem agents via the CLI

## Key conventions

- The WASM component uses `golem-rust` SDK with `#[agent_definition]` / `#[agent_implementation]` macros — not raw WIT exports.
- Agent functions are invoked via `golem agent invoke` using WAVE format. Enum variants are kebab-case (e.g., `string-value("x")`), struct fields are kebab-case (e.g., `{col-name: "x"}`).
- `TimelineOpGraph` is the flat, non-recursive graph encoding sent over the wire (`nodes[0]` = root, children referenced by `NodeIndex`). `TimeLineOp` is the recursive internal representation.
- `setup_node` in the driver uses pre-order depth-first numbering: counter increments before recursion, producing worker names like `{session}-node-{counter}`.
- The push cascade flows leaf → root via `on_child_state_changed`. Point-in-time queries (`get_leaf_result`, `get_derived_result`) are local lookups — no RPC cascade.

## Coding conventions

- Never use raw tuples like `(String, String)` or `(u64, bool)`. Define a named struct — even for two fields. Types should read like domain language, not positional puzzles.
- Prefer newtypes over primitive aliases: `EventColumnName(String)` not `type ColName = String`.
- Keep enum variants self-documenting. If a variant needs more than one field, use a struct — not a tuple variant.
- **Never use `.unwrap()` or `.expect()`** — no exceptions, no justifications. Propagate errors with `?` and handle them only at the boundary (main, test assertions, top-level agent entry points). This rule is absolute.

## Things to watch out for

- Never add dependencies that don't compile to `wasm32-wasip1` inside `components-rust/`. The services and test harness target native.
- Worker names are deterministic from session ID + traversal order. Rerunning tests against the same Golem server without `--clean` will hit stale state.
- `ParentRef` is currently singular (one parent per agent). See the "Compute Reuse" section in README.md for the planned fan-out design.
