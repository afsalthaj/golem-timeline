# Issues

## Issue 1: Metric Registry — register, deduplicate, and configure feeder

**Problem**: There's no way to register a metric expression and have the system remember it.
Developers must manually construct WAVE strings, invoke the TimelineDriver by hand, and
manually configure the feeder with leaf worker names.

**What to build**:
- A `MetricRegistry` (in-memory for now) that stores registered metrics: name, DSL text,
  compiled `TimelineOpGraph`, `AggregationConfig`, and the **leaf routing table** (which
  leaf node indices exist and what event columns each leaf needs).
- Deduplication: if a developer registers a DSL expression identical to an existing metric,
  return the existing metric instead of creating a new one.
- On registration, derive the leaf routing table by walking the compiled graph. This tells
  the feeder which `event-processor` agents to send each event to per session.
- REST API in the dashboard:
  - `POST /api/metrics` — register a new metric (accepts DSL text + aggregation config),
    compiles it, derives the leaf routing table, and configures the feeder.
  - `GET /api/metrics` — list all registered metrics with their agent templates.
- The DSL parser (`timeline-dsl`) already exists — use it to compile DSL text to `TimelineOpGraph`.
- The `GET /api/metrics` endpoint must expose the leaf routing table per metric, so that
  feeders can poll it to discover new metrics and start routing events to the correct leaves.

**Scope**: Dashboard service (registry + API). Feeder changes are a separate concern —
the timeline project does not manage feeders. See "Feeder deployment — open design question"
in the README for the trade-offs between reusing existing feeders vs spinning up new ones.
No changes to timeline-core or Golem agents.

---

## Issue 2: Dashboard UI — DSL editor, deploy button, per-metric views

**Problem**: The current dashboard has hardcoded CIRR presets. Developers can't write
their own metrics or inspect them dynamically.

**What to build**:
- **Deploy Metric tab**: DSL text editor + optional aggregation config + "Deploy Metric" button.
  On deploy, calls `POST /api/metrics`, shows the agent template (full list of Golem agent IDs
  with `{session-id}` placeholder and business descriptions derived from the compiled graph).
- **Session lookup**: Developer enters a session ID they know (from their application logs or
  event stream) and a query time → Computation Progress queries all agents for that session.
  No enumeration of all sessions — that's impossible at scale.
- **Per-metric aggregation**: Aggregator worker names are derived from the metric's
  `group_by_column` — show the correct preset buttons per metric.

**Depends on**: Issue 1.


