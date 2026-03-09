// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
mod feeder;
mod registry;
mod wave;

use axum::extract::{Path, State};
use axum::response::Html;
use axum::routing::{get, post};
use axum::Json;
use std::process::Command;
use tower_http::cors::CorsLayer;

use feeder::FeederManager;
use registry::{MetricRegistry, RegisterMetricRequest, RegisteredMetric};

#[derive(Clone)]
struct AppState {
    registry: MetricRegistry,
    feeder: FeederManager,
}

fn workspace_root() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("Cannot determine workspace root")
}

fn invoke_agent(agent_id: &str, function: &str, args: &[&str]) -> Json<serde_json::Value> {
    let mut cmd = Command::new("golem");
    cmd.args(["agent", "invoke", "--format", "json", agent_id, function]);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.current_dir(workspace_root());

    match cmd.output() {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            match serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                Ok(json) => Json(json),
                Err(_) => Json(serde_json::json!({"error": "failed to parse response"})),
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            Json(serde_json::json!({"error": stderr.to_string()}))
        }
        Err(e) => Json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn index() -> Html<&'static str> {
    Html(include_str!("dashboard.html"))
}

async fn aggregation(Path(agent): Path<String>) -> Json<serde_json::Value> {
    let agent_id = format!("aggregator(\"{}\")", agent);
    invoke_agent(&agent_id, "get-aggregation-result", &[])
}

async fn leaf_result(Path((agent, t1)): Path<(String, u64)>) -> Json<serde_json::Value> {
    let agent_id = format!("event-processor(\"{}\")", agent);
    let t1_str = t1.to_string();
    invoke_agent(&agent_id, "get-leaf-result", &[&t1_str])
}

async fn derived_result(Path((agent, t1)): Path<(String, u64)>) -> Json<serde_json::Value> {
    let agent_id = format!("timeline-processor(\"{}\")", agent);
    let t1_str = t1.to_string();
    invoke_agent(&agent_id, "get-derived-result", &[&t1_str])
}

async fn register_metric(
    State(state): State<AppState>,
    Json(req): Json<RegisterMetricRequest>,
) -> Json<serde_json::Value> {
    match state.registry.register(&req) {
        Ok((metric, graph_wave, agg_wave)) => {
            state.feeder.start_feeder(&metric, graph_wave, agg_wave);
            match serde_json::to_value(&metric) {
                Ok(v) => Json(v),
                Err(e) => Json(serde_json::json!({"error": e.to_string()})),
            }
        }
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

async fn list_metrics(State(state): State<AppState>) -> Json<Vec<RegisteredMetric>> {
    match state.registry.list() {
        Ok(metrics) => Json(metrics),
        Err(_) => Json(Vec::new()),
    }
}

#[tokio::main]
async fn main() {
    let kafka_broker =
        std::env::var("KAFKA_BROKER").unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka_topic =
        std::env::var("KAFKA_TOPIC").unwrap_or_else(|_| "timeline-events".to_string());

    let state = AppState {
        registry: MetricRegistry::new(),
        feeder: FeederManager::new(kafka_broker, kafka_topic),
    };

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/api/aggregation/{agent}", get(aggregation))
        .route("/api/leaf/{agent}/{t1}", get(leaf_result))
        .route("/api/derived/{agent}/{t1}", get(derived_result))
        .route("/api/metrics", post(register_metric))
        .route("/api/metrics", get(list_metrics))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Dashboard running at http://localhost:3000");
    axum::serve(listener, app).await.expect("Server error");
}
