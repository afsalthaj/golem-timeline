# Golem Timeline

[TimeLine Analytics](https://www.cidrdb.org/cidr2023/papers/p22-milner.pdf) backed by Durable Execution Engine provided by [Golem](https://learn.golem.cloud)

Watch the talk from Afsal at [LambdaConf:2024:Estes-Park:Colorado](https://www.youtube.com/watch?v=9WjUBOfgriY)

## Overview

The project implements the TimeLine DSL — a composable language for expressing temporal analytics over event streams.
Each node in a timeline expression maps to a durable Golem agent (worker). Leaf nodes ingest events directly,
while derived nodes fetch results from their children via RPC and combine them.

<img width="754" alt="image" src="https://github.com/user-attachments/assets/4016368e-c5e2-4799-abcb-8d08f7439bc9">

### Architecture

- **`common-rust/common-lib`** — Pure Rust domain library. Contains the recursive `TimeLineOp` DSL,
  `StateDynamicsTimeLine`, `EventTimeLine`, predicates, zip/align logic. No Golem dependencies.

- **`components-rust/timeline-core`** — Single Golem component with 3 agent types:
  - **`EventProcessor`** — Leaf node. Ingests events (`add_event`) and computes leaf timeline operations
    (`tl_latest_event_to_state`, `tl_has_existed`, `tl_has_existed_within`).
  - **`TimelineProcessor`** — Derived node. Fetches results from child agents via RPC and computes
    comparisons (`equal_to`, `greater_than`, ...), boolean logic (`and`, `or`, `not`), and duration operations.
  - **`TimelineDriver`** — Orchestrator. Accepts a `TimelineOpGraph` (non-recursive, flat encoding of the DSL),
    walks the tree, and spawns `EventProcessor` / `TimelineProcessor` agents wired together.

The `TimelineOpGraph` is a non-recursive `Vec<TimelineNode>` with index references — required because
Golem's type system (derived via `#[derive(Schema)]`) does not support recursive types. Internally,
it converts to/from the recursive `TimeLineOp` for computation.

# Summary of examples (mentioned in the talk)
## Connection Induced Rebuffering Ratio


### Actual timeline
```
      |
seek  |                   seek
      |               
buffer|                               ---(buffer)---
play  |         ---(play)--             
t ---------------------------------------------->  
                t1        t2          t3          t10
```

### TimeLine DSL semantics

#### TLHas_Existed(play)

```
     (play)--------------------------------------
-----t1
```

#### Not TLHas_Existed_Within(seek, 5sec)

```
                         
t1----------               t7-------------             
                         
           t2---(seek+5)---t7

```

#### Latest state is buffer (TL_LatestEventToState)

```
            t3-------------(bufer)

-------------  
t1          t3

```

#### And all of it

```

                    t7--------t10
       
t1------t2----------t7
```

#### TL_duration_where:

```

3sec                            /
2sec                          /
1sec                        /
0sec----------------------/
                          t7  t8 t9 t10

```

The summary of the above timeline is as follows:
> User did start playing at some point. After playing user did perform a seek event
> at some point. We extend this event to a configurable 5 seconds. Even after
> extending the seek event to 5 seconds, we can see there still exists 3 seconds
> of buffering, indicating this buffering may not be the direct outcome of seek -
> contributing to the connection induced rebuffering!


## A simple credit card transaction outlier detection

```rust
TL_HasExistedWithin(TL_DurationInCurState(TL_LatestEventToState(col("lat_long")), col(duration) < 10)
```

## QuickStart

### Prerequisites

- Rust with `wasm32-wasip1` target: `rustup target add wasm32-wasip1`
- Golem CLI: download from https://github.com/golemcloud/golem/releases

### Build

```shell
golem build
```

### Run locally

```shell
# Terminal 1: start the Golem server
golem server run

# Terminal 2: deploy the component
golem deploy
```

### Initialize a timeline

Use the REPL or `golem agent invoke` to submit a timeline expression to the driver.
For example, the CIRR expression `EqualTo(TlLatestEventToState("playerStateChange"), "play")`:

```shell
golem agent invoke \
  'timeline-driver("cirr")' \
  'timeline:core/timeline-driver.{initialize-timeline}' \
  '{nodes: [comparison(equal-to, 1, string-value("play")), tl-latest-event-to-state("playerStateChange")]}'
```

This spawns the required EventProcessor and TimelineProcessor agents, wired together.

### Feed events

Once the timeline is initialized, feed events to the leaf EventProcessor agents.
The driver logs which worker names it created — use those to target events:

```shell
golem agent invoke \
  'event-processor("cirr-node-1")' \
  'timeline:core/event-processor.{add-event}' \
  '{time: 1, event: [("playerStateChange", string-value("play"))]}'

golem agent invoke \
  'event-processor("cirr-node-1")' \
  'timeline:core/event-processor.{add-event}' \
  '{time: 5, event: [("playerStateChange", string-value("pause"))]}'
```

### Query results

```shell
golem agent invoke \
  'event-processor("cirr-node-1")' \
  'timeline:core/event-processor.{get-leaf-result}' \
  '3'
```

## Streaming with Pulsar

For real workloads, use a message broker like Pulsar to stream events into the system.
The pattern is:

1. A **producer** writes events to a Pulsar topic (e.g., player state changes).
2. A **feeder** (consumer) reads from the topic and calls `add-event` on the appropriate
   EventProcessor agents via the Golem invocation API or agent-to-agent RPC.

```
┌──────────┐     ┌─────────┐     ┌────────────────────┐
│ Producer │────▶│  Pulsar  │────▶│  Feeder            │
│ (events) │     │  Topic   │     │  (calls add-event  │
└──────────┘     └─────────┘     │   on EventProcessor │
                                  │   agents via Golem) │
                                  └────────────────────┘
```

The feeder can be a standalone Rust binary that uses the Golem REST API, or another
Golem agent that consumes from Pulsar and dispatches events.

## TimeLine DSL Operations

| Operation | Type | Description |
|-----------|------|-------------|
| `TlLatestEventToState(col)` | Leaf | Track latest event value for a column as state |
| `TlHasExisted(predicate)` | Leaf | Has the predicate ever been true? (cumulative OR) |
| `TlHasExistedWithin(predicate, duration)` | Leaf | Has the predicate been true within a time window? |
| `EqualTo(timeline, value)` | Derived | Is the timeline state equal to a constant? |
| `GreaterThan(timeline, value)` | Derived | Is the timeline state greater than a constant? |
| `LessThan(timeline, value)` | Derived | Is the timeline state less than a constant? |
| `And(left, right)` | Derived | Boolean AND of two timelines |
| `Or(left, right)` | Derived | Boolean OR of two timelines |
| `Not(timeline)` | Derived | Negate a boolean timeline |
| `TlDurationWhere(timeline)` | Derived | Cumulative duration where timeline is true |
| `TlDurationInCurState(timeline)` | Derived | Duration in the current state |
