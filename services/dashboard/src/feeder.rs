// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
//! Feeder manager that spawns per-metric Kafka consumer tasks to feed events
//! to Golem agents via `golem agent invoke`.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;

use crate::registry::RegisteredMetric;
use crate::wave;

/// Tracks which metrics have active feeder tasks.
#[derive(Clone)]
pub struct FeederManager {
    kafka_broker: String,
    kafka_topic: String,
    active_metrics: Arc<Mutex<HashSet<String>>>,
}

impl FeederManager {
    pub fn new(kafka_broker: String, kafka_topic: String) -> Self {
        Self {
            kafka_broker,
            kafka_topic,
            active_metrics: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Start a background feeder task for the given metric.
    ///
    /// If a feeder is already running for this metric name, returns immediately.
    /// Otherwise, spawns a tokio task that consumes from Kafka and routes events
    /// to the appropriate Golem agents.
    pub fn start_feeder(&self, metric: &RegisteredMetric, graph_wave: String, agg_wave: String) {
        let metric_name = metric.name.0.clone();

        {
            let mut active = match self.active_metrics.lock() {
                Ok(guard) => guard,
                Err(_) => return,
            };
            if active.contains(&metric_name) {
                return;
            }
            active.insert(metric_name.clone());
        }

        // Build routing table: column name → list of leaf suffixes
        let mut routing: HashMap<String, Vec<String>> = HashMap::new();
        for leaf in &metric.leaves {
            for col in &leaf.event_columns {
                routing
                    .entry(col.0.clone())
                    .or_default()
                    .push(leaf.agent_suffix.0.clone());
            }
        }

        let kafka_broker = self.kafka_broker.clone();
        let kafka_topic = self.kafka_topic.clone();

        tokio::spawn(async move {
            if let Err(e) =
                run_feeder_loop(&kafka_broker, &kafka_topic, &metric_name, routing, &graph_wave, &agg_wave).await
            {
                eprintln!("[feeder:{}] error: {}", metric_name, e);
            }
        });
    }
}

/// Run the Kafka consumer loop for a single metric.
async fn run_feeder_loop(
    kafka_broker: &str,
    kafka_topic: &str,
    metric_name: &str,
    routing: HashMap<String, Vec<String>>,
    graph_wave: &str,
    agg_wave: &str,
) -> Result<(), String> {
    let group_id = format!(
        "dashboard-feeder-{}-{}",
        metric_name,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    );

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_broker)
        .set("group.id", &group_id)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "true")
        .create()
        .map_err(|e| e.to_string())?;

    consumer
        .subscribe(&[kafka_topic])
        .map_err(|e| e.to_string())?;

    println!("[feeder:{}] started, group={}", metric_name, group_id);

    let mut initialized_sessions: HashSet<String> = HashSet::new();
    let mut event_count: u64 = 0;

    loop {
        let msg = tokio::time::timeout(std::time::Duration::from_secs(10), consumer.recv()).await;

        let msg = match msg {
            Ok(Ok(m)) => m,
            Ok(Err(e)) => {
                eprintln!("[feeder:{}] kafka error: {}", metric_name, e);
                continue;
            }
            Err(_) => {
                println!("[feeder:{}] idle (no messages for 10s)", metric_name);
                continue;
            }
        };

        // Extract session_id from key
        let session_id = match msg.key_view::<str>() {
            Some(Ok(k)) => k.to_string(),
            _ => continue,
        };

        // Parse payload as JSON
        let payload = match msg.payload_view::<str>() {
            Some(Ok(s)) => s,
            _ => continue,
        };

        let json: serde_json::Value = match serde_json::from_str(payload) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Initialize timeline if this is a new session
        if !initialized_sessions.contains(&session_id) {
            let agent_id = format!("timeline-driver(\"{}\")", session_id);
            match invoke_agent_raw(&agent_id, "initialize-timeline", &[graph_wave, agg_wave]).await
            {
                Ok(()) => {
                    initialized_sessions.insert(session_id.clone());
                    println!(
                        "[feeder:{}] initialized session {}",
                        metric_name, session_id
                    );
                }
                Err(e) => {
                    eprintln!(
                        "[feeder:{}] failed to initialize session {}: {}",
                        metric_name, session_id, e
                    );
                    continue;
                }
            }
        }

        // Determine which event columns are present
        let event_columns: Vec<String> = extract_event_columns(&json);

        // Collect matching leaf suffixes (deduplicated)
        let mut target_suffixes: HashSet<String> = HashSet::new();
        for col in &event_columns {
            if let Some(suffixes) = routing.get(col) {
                for s in suffixes {
                    target_suffixes.insert(s.clone());
                }
            }
        }

        // Convert event to WAVE
        let wave_event = match wave::event_json_to_wave(&json) {
            Some(w) => w,
            None => continue,
        };

        // Send to each matching leaf agent
        for suffix in &target_suffixes {
            let agent_name = format!("{}-{}", session_id, suffix);
            let agent_id = format!("event-processor(\"{}\")", agent_name);
            if let Err(e) = invoke_agent_raw(&agent_id, "add-event", &[&wave_event]).await {
                eprintln!(
                    "[feeder:{}] failed to send to {}: {}",
                    metric_name, agent_name, e
                );
            }
        }

        event_count += 1;
        if event_count % 10 == 0 {
            println!("[feeder:{}] processed {} events", metric_name, event_count);
        }
    }
}

/// Extract event column names from a JSON event payload.
///
/// Expects `{"time": N, "event": [["col", {...}], ...]}`.
/// Returns the column names from the event array.
fn extract_event_columns(json: &serde_json::Value) -> Vec<String> {
    let mut columns = Vec::new();
    if let Some(event_arr) = json.get("event").and_then(|e| e.as_array()) {
        for entry in event_arr {
            if let Some(pair) = entry.as_array() {
                if let Some(col) = pair.first().and_then(|c| c.as_str()) {
                    columns.push(col.to_string());
                }
            }
        }
    }
    columns
}

/// Invoke a Golem agent via CLI. Uses async `tokio::process::Command` to avoid
/// blocking the tokio runtime.
async fn invoke_agent_raw(agent_id: &str, function: &str, args: &[&str]) -> Result<(), String> {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("Cannot determine workspace root");

    let mut cmd = tokio::process::Command::new("golem");
    cmd.args(["agent", "invoke", "--format", "json", agent_id, function]);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.current_dir(workspace_root);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }
    Ok(())
}
