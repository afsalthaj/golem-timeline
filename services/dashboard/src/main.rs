use axum::extract::Path;
use axum::response::Html;
use axum::routing::get;
use axum::Json;
use std::process::Command;
use tower_http::cors::CorsLayer;

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

async fn aggregation(Path(worker): Path<String>) -> Json<serde_json::Value> {
    let agent_id = format!("aggregator(\"{}\")", worker);
    invoke_agent(&agent_id, "get-aggregation-result", &[])
}

async fn leaf_result(Path((worker, t1)): Path<(String, u64)>) -> Json<serde_json::Value> {
    let agent_id = format!("event-processor(\"{}\")", worker);
    let t1_str = t1.to_string();
    invoke_agent(&agent_id, "get-leaf-result", &[&t1_str])
}

async fn derived_result(Path((worker, t1)): Path<(String, u64)>) -> Json<serde_json::Value> {
    let agent_id = format!("timeline-processor(\"{}\")", worker);
    let t1_str = t1.to_string();
    invoke_agent(&agent_id, "get-derived-result", &[&t1_str])
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(index))
        .route("/api/aggregation/{worker}", get(aggregation))
        .route("/api/leaf/{worker}/{t1}", get(leaf_result))
        .route("/api/derived/{worker}/{t1}", get(derived_result))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Dashboard running at http://localhost:3000");
    axum::serve(listener, app).await.expect("Server error");
}
