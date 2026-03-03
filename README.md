# Durable Timeline Analytics For Data Engineering

This is inspired from [TimeLine Analytics](https://www.cidrdb.org/cidr2023/papers/p22-milner.pdf) and it's extention, but most importantly,
system being backed by new agentic runtime [Golem](https://learn.golem.cloud) that's also a durable execution engine.

Watch the talk from Afsal at [LambdaConf:2024:Estes-Park:Colorado](https://www.youtube.com/watch?v=9WjUBOfgriY) or refer presentation slides [here](https://github.com/afsalthaj/golem-timeline-presentation/blob/main/presentation_last.pdf)

## Timeline Query DSL

A text-based DSL for expressing temporal analytics over event streams. Write a query, deploy it,
and the system materializes a push-based agent graph that processes events in real time.

### CIRR — Connection Induced Rebuffering Ratio

A user is experiencing connection-induced rebuffering when:
1. They started playing at some point (`has_existed`)
2. There was no recent seek event (`!has_existed_within` — rules out seek-induced buffering)
3. The current player state is `"buffer"` (`latest_event_to_state == "buffer"`)

All three must be true simultaneously, and we measure how long that condition holds.

```javascript
duration_where(
  has_existed(playerStateChange == "play")
  && !has_existed_within(playerStateChange == "seek", 5)
  && latest_event_to_state(playerStateChange) == "buffer"
)
```

With cross-session aggregation per CDN:

```javascript
duration_where(
  has_existed(playerStateChange == "play")
  && !has_existed_within(playerStateChange == "seek", 5)
  && latest_event_to_state(playerStateChange) == "buffer"
) | aggregate(group_by=cdn, count, sum, avg)
```


### More examples

**Time spent idle per region:**
```javascript
duration_in_cur_state(
  latest_event_to_state(status) == "idle"
) | aggregate(group_by=region, count, avg, max)
```

**Credit card location anomaly — location changed within 10 minutes:**
```javascript
has_existed_within(
  lat_long > 0, 600
)
```

**User engaged: played and never errored:**
```
has_existed(playerStateChange == "play") && !has_existed(error == "fatal")
```

## Overview

The project is a merging the ideas from the TimeLine DSL — a composable language for expressing temporal analytics over event streams, into Golem's Runtime.
Each node in a timeline expression maps to a durable Golem agent (worker). The architecture is **fully push-based**:
leaf nodes ingest events and push state changes upward through the agent tree. Derived nodes recompute
incrementally on each notification and cascade changes to their parents. Point-in-time queries are local
lookups on precomputed state — no cascading RPC required at query time.

### Architecture

```
                          ┌─────────────────┐
                          │ TimelineDriver   │  (1) Walks the DSL tree, spawns agents,
                          │ (orchestrator)   │      wires ParentRef / AggregatorRef
                          └────────┬────────┘
                                   │ spawns & wires
              ┌────────────────────┼────────────────────┐
              ▼                    ▼                     ▼
   ┌──────────────────┐ ┌──────────────────┐  ┌──────────────────┐
   │ EventProcessor   │ │ EventProcessor   │  │ EventProcessor   │
   │ (leaf: has_exist) │ │ (leaf: latest)   │  │ (leaf: within)   │
   └────────┬─────────┘ └────────┬─────────┘  └────────┬─────────┘
            │ push                │ push                │ push
            ▼                    ▼                     ▼
   ┌──────────────────┐ ┌──────────────────┐
   │TimelineProcessor │ │TimelineProcessor │  (2) Receives on_child_state_changed,
   │ (derived: And)   │ │ (derived: EqualTo│      recomputes, pushes to parent
   └────────┬─────────┘ └────────┬─────────┘
            │ push                │ push
            └────────┬───────────┘
                     ▼
          ┌──────────────────┐       ┌──────────────────┐
          │TimelineProcessor │──────▶│   Aggregator     │  (3) Root pushes deltas
          │ (root: Duration) │ delta │ (cross-session)  │      to aggregator
          └──────────────────┘       └──────────────────┘
```

- **`common-rust/common-lib`** — Pure Rust domain library. Contains the recursive `TimeLineOp` DSL,
  `StateDynamicsTimeLine`, `EventTimeLine`, predicates, zip/align logic. No Golem dependencies.

- **`components-rust/timeline-core`** — Single Golem component with 4 agent types:
  - **`EventProcessor`** — Leaf node. Ingests events (`add_event`) and computes leaf timeline operations
    (`tl_latest_event_to_state`, `tl_has_existed`, `tl_has_existed_within`). On state change, pushes
    the new value to its parent via `notify_parent`.
  - **`TimelineProcessor`** — Derived node. Receives state change notifications from children via
    `on_child_state_changed`, recomputes its own state (comparisons, boolean logic, duration tracking),
    and cascades the change upward to its parent. Duration operations use a compact `DurationState`
    (Climbing/Flat) to track cumulative time without per-tick storage.
  - **`TimelineDriver`** — Orchestrator. Accepts a `TimelineOpGraph` (non-recursive, flat encoding of the DSL),
    walks the tree, spawns `EventProcessor` / `TimelineProcessor` agents, and wires them together
    by setting `ParentRef` on each child so push notifications flow upward.
  - **`Aggregator`** — Cross-session accumulator. Receives delta-based updates from root
    `TimelineProcessor` nodes, maintaining running Count / Sum / Avg / Min / Max with O(1) memory.

The `TimelineOpGraph` is a non-recursive `Vec<TimelineNode>` with index references — required because
Golem's type system (derived via `#[derive(Schema)]`) does not support recursive types. Internally,
it converts to/from the recursive `TimeLineOp` for computation.


### Push-based data flow

1. **Event ingestion** — An event arrives at an `EventProcessor` via `add_event`.
2. **Leaf computation** — The leaf evaluates its operation (e.g., "has `status == error` ever been true?")
   and records the result in its local `StateDynamicsTimeLine`.
3. **Parent notification** — If the state changed, the leaf calls `on_child_state_changed` on its parent
   `TimelineProcessor` (identified by the `ParentRef` wired during initialization).
4. **Derived recomputation** — The `TimelineProcessor` updates its own state and, if changed, pushes
   upward to *its* parent. This cascade continues until the root node is reached.
5. **Aggregator update** — If the root node has an `AggregatorRef`, it computes the delta between the
   old and new numeric value and calls `on_delta` on the `Aggregator` agent. The aggregator maintains
   only running accumulators (sum, count), so each session contributes O(1) memory.
6. **Query** — `get_leaf_result(t)` or `get_derived_result(t)` performs a local point lookup on the
   precomputed `StateDynamicsTimeLine`. No RPC cascade is needed.

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

### Initialize a timeline with cross-session aggregation

To aggregate CIRR duration across CDN sessions, pass an `AggregationConfig` alongside the timeline.
For example, to group by the `cdn` column in event data and compute Count, Sum, and Avg:

```shell
golem agent invoke \
  'timeline-driver("cirr-cdn-x-session-1")' \
  'timeline:core/timeline-driver.{initialize-timeline}' \
  '{nodes: [tl-duration-where(1), and(2, 3), comparison(equal-to, 4, string-value("buffer")), tl-latest-event-to-state("playerStateChange"), tl-has-existed(col-name: "playerStateChange", value: string-value("play"), op: equal))]}' \
  '{group-by-column: "cdn", aggregations: [count, sum, avg]}'
```

Each session's root node pushes deltas to the shared `aggregator-cdn-x` agent.
Query the aggregated metrics across all sessions for that CDN:

```shell
golem agent invoke \
  'aggregator("aggregator-cdn-x")' \
  'timeline:core/aggregator.{get-aggregation-result}'
```

This returns `{ count, sum, avg, min, max }` — the running aggregate of CIRR duration
across all sessions grouped under `cdn-x`.

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

## Deploying CIRR at a Streaming Company

This section walks through a realistic end-to-end deployment of CIRR at a hypothetical
streaming platform (think Disney+, Netflix, etc.) where player telemetry events flow through
Pulsar or Kafka.

### End-to-end architecture

```
┌────────────────┐     ┌───────────────┐     ┌────────────────────────────────────┐
│  Video Players │────▶│  Pulsar/Kafka  │────▶│  Feeder (Pulsar Consumer)          │
│  (millions of  │     │  Topic:        │     │                                    │
│   sessions)    │     │  player-events │     │  1. Extract session_id from msg    │
└────────────────┘     └───────────────┘     │  2. If new session:                │
                                              │     → initialize_timeline(sess_id) │
                                              │  3. Route event to leaf agents     │
                                              │     using naming convention:       │
                                              │     "{sess_id}-node-{N}"           │
                                              └──────────────┬─────────────────────┘
                                                             │
                                              ┌──────────────▼─────────────────────┐
                                              │         Golem Cloud (K8s)           │
                                              │                                    │
                                              │  ┌──────────┐   ┌──────────┐       │
                                              │  │  Leaf     │   │  Leaf    │       │
                                              │  │  Agents   │──▶│ Derived  │──▶ ...│
                                              │  │(per sess) │   │  Agents  │       │
                                              │  └──────────┘   └────┬─────┘       │
                                              │                      │              │
                                              │               ┌──────▼──────┐       │
                                              │               │ Aggregator  │       │
                                              │               │ (per CDN)   │       │
                                              │               └─────────────┘       │
                                              └────────────────────────────────────┘
```

### Step 1: Define the CIRR workflow

The CIRR expression is defined once for the entire platform:

```
TlDurationWhere(
  And(
    And(
      TlHasExisted(playerStateChange == "play"),
      Not(TlHasExistedWithin(userAction == "seek", 5))
    ),
    EqualTo(TlLatestEventToState("playerStateChange"), "buffer")
  )
)
```

This is the same expression for every session — Afsal watching a movie and John watching
a movie both use this exact tree. The only difference is the session ID prefix in agent names.

### Step 2: Bootstrap — Discover the agent topology

The `TimelineDriver` uses a depth-first counter to name agents. For the CIRR tree, the
traversal produces these nodes:

```
setup_node(TlDurationWhere)          → counter=1  "{sid}-node-1"  TimelineProcessor
  setup_node(And)                    → counter=2  "{sid}-node-2"  TimelineProcessor
    setup_node(And)        [left]    → counter=3  "{sid}-node-3"  TimelineProcessor
      setup_node(TlHasExisted) [L]   → counter=4  "{sid}-node-4"  EventProcessor ★
      setup_node(Not)          [R]   → counter=5  "{sid}-node-5"  TimelineProcessor
        setup_node(TlHasExistedWithin)→ counter=6  "{sid}-node-6"  EventProcessor ★
    setup_node(EqualTo)    [right]   → counter=7  "{sid}-node-7"  TimelineProcessor
      setup_node(TlLatestEventToState)→ counter=8  "{sid}-node-8"  EventProcessor ★
```

The ★ markers are the **leaf EventProcessor agents** — the ones that receive events.
This gives us the static routing table:

| Leaf worker | Operation | Listens for column | Matching events |
|---|---|---|---|
| `{sid}-node-4` | TlHasExisted | `playerStateChange` | `playerStateChange == "play"` |
| `{sid}-node-6` | TlHasExistedWithin | `userAction` | `userAction == "seek"` (within 5s) |
| `{sid}-node-8` | TlLatestEventToState | `playerStateChange` | Any `playerStateChange` value |

**This table is the same for every CIRR session.** The node numbering is deterministic
because `setup_node` always traverses depth-first with a monotonic counter.

> **Note:** The system does not currently return this plan as a structured object.
> One practical approach: call `initialize_timeline` for a single bootstrap session,
> observe the worker names created (e.g., from logs or the return string), and extract
> the naming pattern. Replace the concrete session ID with a `{sid}` placeholder —
> that becomes your static routing template for all future sessions.

### Step 3: Build the Pulsar/Kafka consumer (feeder)

The feeder is a standalone process (not a Golem agent) that bridges the message broker
and Golem. Here is the event routing logic:

```
                    ┌──────────────────────────────────────┐
                    │          Feeder (Consumer)            │
                    │                                      │
                    │  Event arrives from Pulsar:           │
                    │  { session_id: "sess-42",            │
                    │    time: 7,                           │
                    │    event: [("playerStateChange",      │
                    │             "buffer")] }              │
                    │                                      │
                    │  1. session_id = "sess-42"           │
                    │                                      │
                    │  2. First event for this session?    │
                    │     YES → call initialize_timeline   │
                    │           on TimelineDriver("sess-42")│
                    │     NO  → skip (already initialized) │
                    │                                      │
                    │  3. Column is "playerStateChange"    │
                    │     → route to node-4 AND node-8     │
                    │                                      │
                    │     If column were "userAction"      │
                    │     → route to node-6 only           │
                    └────────────┬──────────┬──────────────┘
                                 │          │
            ┌────────────────────▼┐   ┌─────▼──────────────────┐
            │  EventProcessor     │   │  EventProcessor         │
            │  "sess-42-node-4"   │   │  "sess-42-node-8"       │
            │  (TlHasExisted)     │   │  (TlLatestEventToState)  │
            └─────────────────────┘   └──────────────────────────┘
```

Note that **one event can fan out to multiple leaves**. A `playerStateChange` event
must be sent to both `node-4` (which checks "has play ever existed?") and `node-8`
(which tracks the latest state). The feeder is responsible for this fan-out.

The feeder only needs to track **which sessions have been initialized** (a simple
`HashSet<SessionId>`, not a full plan per session). The routing logic is static
and identical for every session:

```rust
// Static routing table — derived once from the CIRR workflow
fn route_event(session_id: &str, column: &str) -> Vec<String> {
    match column {
        "playerStateChange" => vec![
            format!("{session_id}-node-4"),
            format!("{session_id}-node-8"),
        ],
        "userAction" => vec![
            format!("{session_id}-node-6"),
        ],
        _ => vec![], // unknown column, ignore
    }
}
```

### Step 4: Runtime event flow — Afsal and John watch movies

```
Timeline: 8 PM Friday

  Afsal starts "The Mandalorian"           John starts "Moana"
  session_id = "afsal-mando-1"             session_id = "john-moana-1"
          │                                         │
          ▼                                         ▼
  ┌─── Pulsar Topic: player-events ─────────────────────────────────┐
  │ {sid:"afsal-mando-1", time:1, playerStateChange:"play"}         │
  │ {sid:"john-moana-1",  time:1, playerStateChange:"play"}         │
  │ {sid:"afsal-mando-1", time:5, playerStateChange:"buffer"}       │
  │ {sid:"john-moana-1",  time:3, userAction:"seek"}                │
  │ {sid:"afsal-mando-1", time:7, userAction:"seek"}                │
  │ ...                                                             │
  └──────────────────────────┬──────────────────────────────────────┘
                             │
                      ┌──────▼───────┐
                      │   Feeder     │
                      └──────┬───────┘
                             │
              ┌──────────────┼──────────────┐
              ▼                             ▼
  Afsal's agent tree:              John's agent tree:
  afsal-mando-1-node-1 (root)     john-moana-1-node-1 (root)
  afsal-mando-1-node-2            john-moana-1-node-2
  ...                             ...
  afsal-mando-1-node-8            john-moana-1-node-8
              │                             │
              ▼                             ▼
  ┌───────────────────┐         ┌───────────────────┐
  │ aggregator-cdn-a  │         │ aggregator-cdn-b  │
  │ (Akamai)          │         │ (Cloudfront)      │
  └───────────────────┘         └───────────────────┘
```

**At any given instant**, most of these agents are **suspended** (not in memory).
Only the agents currently processing a push notification are active. When Afsal's
`playerStateChange:"buffer"` event arrives:

1. Feeder calls `add_event` on `afsal-mando-1-node-4` and `afsal-mando-1-node-8`
2. `node-4` wakes (~1ms), evaluates "has play existed?" → yes, pushes `true` to `node-3`, suspends
3. `node-3` wakes (~1ms), evaluates `And(true, ...)` → pushes to `node-2`, suspends
4. ... cascade continues to `node-1` (TlDurationWhere) → pushes delta to `aggregator-cdn-a`
5. Meanwhile, `node-8` wakes (~1ms), records latest state as `"buffer"`, pushes to `node-7`, suspends
6. `node-7` evaluates `EqualTo("buffer", "buffer")` → `true`, pushes to `node-2`
7. All agents suspend. Total wall time: ~5–10ms. Total agents in memory during this: ~5

All of John's agents remain completely suspended during this — zero cost.

### Step 5: Querying results

**Per-session query** — "What is Afsal's CIRR duration right now?"

```shell
golem agent invoke \
  'timeline-processor("afsal-mando-1-node-1")' \
  'timeline:core/timeline-processor.{get-derived-result}' \
  '100'
```

This is a local point lookup on `node-1`'s precomputed state — no RPC cascade.

**Cross-session query** — "What is the average CIRR across all Akamai sessions?"

```shell
golem agent invoke \
  'aggregator("aggregator-cdn-a")' \
  'timeline:core/aggregator.{get-aggregation-result}'
```

## TimeLine DSL Operations

| Operation | Type | Description |
|-----------|------|-------------|
| `TlLatestEventToState(col)` | Leaf | Track latest event value for a column as state |
| `TlHasExisted(predicate)` | Leaf | Has the predicate ever been true? (cumulative OR) |
| `TlHasExistedWithin(predicate, duration)` | Leaf | Has the predicate been true within a time window? |
| `EqualTo(timeline, value)` | Derived | Is the timeline state equal to a constant? |
| `GreaterThan(timeline, value)` | Derived | Is the timeline state greater than a constant? |
| `GreaterThanOrEqual(timeline, value)` | Derived | Is the timeline state ≥ a constant? |
| `LessThan(timeline, value)` | Derived | Is the timeline state less than a constant? |
| `LessThanOrEqual(timeline, value)` | Derived | Is the timeline state ≤ a constant? |
| `And(left, right)` | Derived | Boolean AND of two timelines |
| `Or(left, right)` | Derived | Boolean OR of two timelines |
| `Not(timeline)` | Derived | Negate a boolean timeline |
| `TlDurationWhere(timeline)` | Derived | Cumulative duration where timeline is true. Uses a `DurationState` — `Climbing { base, since }` while true, `Flat { value }` while false — so queries at time *t* return `base + (t − since)` without storing per-tick data. |
| `TlDurationInCurState(timeline)` | Derived | Duration in the current state. Resets to `Climbing { base: 0, since: t }` on every state change; queries return elapsed time since the last transition. |

## Cross-Session Aggregation

The **Aggregator** agent enables metrics across multiple independent sessions (e.g., "average CIRR
across all sessions on CDN X"). Each session's root `TimelineProcessor` is wired to a shared
`Aggregator` agent during initialization.

**How it works:**

- When the root node's value changes (e.g., CIRR duration goes from 2 → 5), it computes the
  **delta** (5 − 2 = 3) and calls `on_delta(3.0)` on the aggregator.
- The aggregator maintains only running accumulators (`sum`, `count`), so adding more sessions
  costs O(1) memory per session — no per-session history is stored in the aggregator.
- `register_session` is called once per session during initialization to increment the count.
- Query `get_aggregation_result` at any time to get `{ count, sum, avg, min, max }`.

| Aggregation | Description |
|-------------|-------------|
| `Count` | Number of registered sessions |
| `Sum` | Running sum of all deltas |
| `Avg` | `sum / count` |
| `Min` | Minimum value seen (not yet tracked) |
| `Max` | Maximum value seen (not yet tracked) |

## System Design: Capacity Planning at Scale

This section walks through what a production deployment looks like at streaming-platform scale
(e.g., Disney+), how many agents exist, how many are actually in memory at any instant, and what
the Kubernetes deployment looks like.

### Agents per session

Consider the full CIRR expression from the examples above:

```
TlDurationWhere(
  And(
    And(
      TlHasExisted(play),
      Not(TlHasExistedWithin(seek, 5))
    ),
    EqualTo(TlLatestEventToState("playerStateChange"), "buffer")
  )
)
```

The `TimelineDriver` walks this tree and spawns one agent per node:

```
                    TlDurationWhere          ← TimelineProcessor
                         │
                        And                  ← TimelineProcessor
                       /   \
                     And    EqualTo          ← TimelineProcessor × 2
                    /   \       \
          TlHasExisted  Not    TlLatestEvent ← EventProcessor, TimelineProcessor, EventProcessor
                         |
              TlHasExistedWithin             ← EventProcessor
```

| Agent type | Count per session |
|---|---|
| EventProcessor (leaf) | 3 |
| TimelineProcessor (derived) | 5 |
| **Total per session** | **8** |

Plus 1 shared `Aggregator` per CDN (not per session).

### Disney+ scale estimate

| Parameter | Estimate |
|---|---|
| Total subscribers | ~150 M |
| Peak concurrent streams | ~10 M |
| Agents per session | 8 |
| **Total agents at peak** | **~80 M** |
| CDNs (aggregator agents) | ~10–50 |

80 million agents sounds enormous — but the critical insight is that **Golem suspends idle agents
to durable storage**. An agent that isn't actively processing is not in memory. It's persisted and
can be resumed on demand.

### How many agents are *actually in memory* at any instant?

When Afsal presses play, an event hits his session's `EventProcessor` leaf. The push cascade
wakes agents one at a time up the tree:

```
t=0ms   EventProcessor (leaf)        ← wakes, processes event, pushes to parent, suspends
t=1ms   TimelineProcessor (And)      ← wakes, recomputes, pushes to parent, suspends
t=2ms   TimelineProcessor (And)      ← wakes, recomputes, pushes to parent, suspends
t=3ms   TimelineProcessor (Duration) ← wakes, recomputes, pushes delta to aggregator, suspends
t=4ms   Aggregator                   ← wakes, adds delta to sum, suspends
```

Each agent is active for **< 1ms** per event. At any instant, only the agents *currently
processing a push notification* are in memory. The rest — including all agents for sessions
where no events are arriving — are suspended.

**Rough estimate of in-memory agents:**

| Parameter | Value |
|---|---|
| Events per session per minute | ~2–5 (state changes are sparse) |
| Processing time per agent per event | < 1 ms |
| Agents woken per event (cascade depth) | ~5 (for CIRR) |
| Active time per event | ~5 ms total across the chain |
| Peak events/sec across 10M sessions | ~300K–800K events/sec |
| **In-memory agents at any instant** | **~1,500–4,000** |

That is: out of 80M total agents, only a few thousand are in memory at any moment.
The rest cost nothing beyond durable storage.

### Kubernetes deployment (Golem Cloud)

Golem Cloud runs on Kubernetes. The key resources to size are:

#### Golem worker executor pods

These are the pods that execute agent (worker) code. Each pod hosts many agents concurrently.

| Resource | Estimate | Notes |
|---|---|---|
| Memory per active agent | ~1–5 MB | `StateDynamicsTimeLine` + `DurationState` + WASM runtime overhead |
| In-memory agents per pod | ~500–2,000 | Depends on pod memory limit |
| Pod memory | 4–8 GB | Standard for worker executor pods |
| **Pods needed (steady state)** | **3–8** | For ~4,000 concurrently active agents |
| Pods needed (burst/headroom) | 10–20 | For event spikes (e.g., popular show premiere) |

#### Durable storage

All 80M suspended agents live in Golem's durable persistence layer (e.g., Redis, blob store, or
Golem's built-in storage). Each suspended agent is a serialized snapshot:

| Resource | Estimate |
|---|---|
| Serialized size per agent | ~0.5–2 KB (state timeline + config) |
| **Total storage at peak** | **~40–160 GB** |

This is modest — a single cloud storage volume handles it comfortably.

#### Network (agent-to-agent RPC)

The push cascade means each event triggers a chain of ~5 agent-to-agent calls. At 500K events/sec:

| Parameter | Value |
|---|---|
| Internal RPC calls/sec | ~2.5M |
| Payload per call | ~100–200 bytes (time + EventValue) |
| **Bandwidth** | ~250–500 MB/s internal |

This is within the capacity of a standard Kubernetes cluster's pod-to-pod network, but worth
monitoring. Golem's worker executor pods colocate agents, so many of these calls are in-process
and never hit the network.

### Scaling scenario walkthrough

**8 PM Friday — a popular new show drops on Disney+:**

1. 2M users start streaming within 5 minutes → 2M new sessions → 16M new agents created.
2. The `TimelineDriver` for each session spawns 8 agents and wires them. This is a burst of
   creation, but each driver runs once and suspends. Golem can spread creation across executor pods.
3. Events start flowing: ~10M events/min across 2M sessions. The push cascade processes each
   event in ~5ms end-to-end. Only ~5,000 agents are in memory at any instant.
4. Each session's root pushes deltas to its CDN's `Aggregator`. With 10 CDNs, each aggregator
   handles ~1M sessions but only processes one `on_delta` call at a time — it's a simple
   `sum += delta`, so it never becomes a bottleneck.
5. An hour later, 1M users stop watching. Their agents remain suspended in storage but cost
   zero memory and zero compute. They can be resumed if needed for historical queries.

### Potential hurdles

| Challenge | Mitigation |
|---|---|
| **Agent creation burst** (millions of agents at once) | Golem lazy-creates agents on first invocation. The `TimelineDriver` itself can be parallelized across sessions. Rate-limit session initialization if needed. |
| **Storage growth** (80M serialized agents) | Serialized state is small (~1 KB). Implement TTL-based cleanup for completed sessions. Golem's persistence layer supports compaction. |
| **Hot aggregator** (one aggregator per CDN receiving millions of deltas) | `on_delta` is O(1) — a single addition. If a single CDN has 5M sessions and each emits ~2 events/min, that's ~170K deltas/sec to one aggregator. May need sharding (e.g., `aggregator-cdn-x-shard-0`) for extreme cases. |
| **Cold-start latency** (resuming a suspended agent) | Golem's resume time is typically <10ms. For latency-sensitive paths, keep agents warm with periodic heartbeats. |
| **Event ordering across leaves** | The push-based model processes events per-leaf independently. Two leaves in the same session may receive events at different wall-clock times. The `And`/`Or` nodes use `time + 1` lookups to see the latest state, which handles this correctly for monotonically increasing timestamps. |

## Future Design: Compute Reuse Across Workflows

### The opportunity

Timeline expressions naturally share sub-expressions. Consider a streaming platform that
runs two workflows for the same session:

```
Workflow A (CIRR):
  TlDurationWhere(
    And(
      EqualTo(TlLatestEventToState("playerStateChange"), "buffer"),   ← subtree X
      TlHasExisted(playerStateChange == "play")                       ← subtree Y
    )
  )

Workflow B (Time-to-First-Play):
  TlHasExisted(playerStateChange == "play")                           ← same as Y
```

Subtree `Y` is identical in both workflows. Ideally, a single `EventProcessor` agent
computes `Y` once, and both Workflow A's `And` node and Workflow B's root consume its
output. This is **compute reuse** — the agent DAG shares common sub-expressions instead
of duplicating them.

```
Without reuse (current):              With reuse (future):

  Workflow A        Workflow B           Workflow A        Workflow B
  ┌───────┐         ┌───────┐           ┌───────┐
  │ DurWh │         │       │           │ DurWh │
  └───┬───┘         │       │           └───┬───┘
  ┌───┴───┐         │       │           ┌───┴───┐
  │  And  │         │       │           │  And  │
  └─┬───┬─┘         │       │           └─┬───┬─┘
    │   │           │       │             │   │
  ┌─┴─┐ ┌┴──┐    ┌──┴──┐              ┌─┴─┐ │
  │ X │ │ Y │    │ Y'  │  ← duplicate  │ X │ │
  └───┘ └───┘    └─────┘              └───┘ │
                                             │    ┌───────┐
    2 leaves compute Y                       └────┤   Y   ├──── Workflow B root
    same events sent twice                        └───────┘
                                              1 leaf, shared
```

At Disney+ scale with 10M sessions and 3 workflows sharing 2 common sub-expressions,
this saves ~20M redundant agents and eliminates duplicate event processing.

### Why it doesn't work today

Three structural limitations prevent compute reuse in the current implementation:

**1. `ParentRef` is singular**

Each agent has at most one parent:

```rust
pub struct ParentRef {
    pub worker_name: String,
    pub child_index: u32,
}
```

If `Y` pushes to Workflow A's `And` node, it cannot also push to Workflow B's root.
Sharing requires fan-out to multiple parents.

**2. Agent naming is positional, not content-addressed**

The `TimelineDriver` names agents by traversal order:

```rust
let worker_name = format!("{}-node-{}", self.name, counter);
```

Two workflows traversing the same sub-expression `Y` produce different names
(`sess-1-node-4` vs `sess-1-node-7`). There is no way to detect that they
compute the same thing.

**3. The driver always re-initializes**

Calling `initialize_leaf` on an existing agent overwrites its operation and parent ref.
There is no "add another parent" operation.

### What the fix looks like

```
                 Current                              Future
          ┌──────────────────┐                ┌──────────────────┐
          │   ParentRef      │                │   Vec<ParentRef> │
          │   (single)       │                │   (fan-out)      │
          └──────────────────┘                └──────────────────┘

          ┌──────────────────┐                ┌──────────────────┐
          │  "{sid}-node-{N}"│                │  content-addressed│
          │  (positional)    │                │  naming (hash of │
          └──────────────────┘                │  operation + cols)│
                                              └──────────────────┘

          ┌──────────────────┐                ┌──────────────────┐
          │  initialize_leaf │                │  add_parent_ref  │
          │  (overwrite)     │                │  (append)        │
          └──────────────────┘                └──────────────────┘
```

**Step 1: Content-addressed agent names**

Name agents by what they compute, not where they appear in the tree:

```rust
// Instead of: "{sid}-node-4"
// Use:        "{sid}-leaf-{hash(TlHasExisted, playerStateChange, play)}"
fn agent_name(session_id: &str, op: &TimeLineOp) -> String {
    let hash = hash_operation(op);  // deterministic hash of the subtree
    format!("{}-{}", session_id, hash)
}
```

Two workflows requesting `TlHasExisted(playerStateChange == "play")` for the same
session produce the same agent name → same agent.

**Step 2: Fan-out parent refs**

```rust
// Before:
pub parent: Option<ParentRef>,

// After:
pub parents: Vec<ParentRef>,
```

`notify_parent` becomes `notify_parents` — iterates over all registered parents:

```rust
async fn notify_parents(parents: &[ParentRef], time: u64, value: EventValue) {
    for parent in parents {
        let mut client = TimelineProcessorClient::get(parent.worker_name.clone());
        client.on_child_state_changed(parent.child_index, time, value.clone()).await;
    }
}
```

**Step 3: Idempotent initialization with parent accumulation**

The driver checks if an agent already exists before creating it:

```rust
if agent_exists(&worker_name) {
    // Agent already computing this — just add our parent ref
    add_parent_ref(&worker_name, new_parent_ref).await;
} else {
    // First time — create and initialize
    initialize_leaf(&worker_name, operation).await;
    set_parent(&worker_name, new_parent_ref).await;
}
```

### Trade-offs of compute reuse

| Benefit | Cost |
|---|---|
| Fewer agents (saves memory + storage) | Agent naming becomes content-dependent — harder to debug |
| Less duplicate event processing | Fan-out notifications add latency (serial parent pushes) |
| Fewer events routed by feeder | Cleanup requires reference counting (can't delete a shared agent until all workflows are done) |
| Natural DAG structure | The tree becomes a DAG — cycle detection may be needed for safety |

### Impact at scale

| Metric | Without reuse | With reuse |
|---|---|---|
| Agents per session (3 workflows, 2 shared subtrees) | 24 | 16 |
| Total agents at 10M sessions | 240M | 160M |
| Events processed per input event | 3× (duplicated leaves) | 1× |
| Aggregator delta calls | Same | Same |
