//! Golem client using the `golem` CLI (v1.4.1) for agent invocation.
//!
//! Uses `golem agent invoke` which handles auth, routing, and
//! WAVE serialization automatically.
//!
//! # Agent invoke syntax
//! ```text
//! golem agent invoke --format json <AGENT_ID> <FUNCTION_NAME> [ARGUMENTS in WAVE]...
//! ```

use std::process::Command;

/// Client for interacting with Golem agents via the CLI.
pub struct GolemClient;

impl GolemClient {
    pub fn new() -> Self {
        Self
    }

    /// Invoke an agent function via `golem agent invoke --format json`.
    ///
    /// Returns the parsed JSON response.
    pub fn invoke(
        &self,
        agent_id: &str,
        function: &str,
        wave_args: &[&str],
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("golem");
        cmd.args(["agent", "invoke", "--format", "json"]);
        cmd.arg(agent_id);
        cmd.arg(function);
        for arg in wave_args {
            cmd.arg(arg);
        }

        // Run from workspace root so golem CLI finds golem.yaml and infers the component.
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("Cannot determine workspace root");
        cmd.current_dir(workspace_root);

        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(format!(
                "golem agent invoke failed.\nstdout: {stdout}\nstderr: {stderr}"
            )
            .into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(stdout.trim())?;
        Ok(json)
    }

    /// Initialize a timeline via the TimelineDriver agent.
    ///
    /// `session_id` becomes the driver's constructor arg.
    /// `timeline_wave` is the WAVE-encoded TimelineOpGraph.
    /// `aggregation_wave` is the WAVE-encoded Option<AggregationConfig> (e.g., "none").
    pub fn initialize_timeline(
        &self,
        session_id: &str,
        timeline_wave: &str,
        aggregation_wave: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent_id = format!("timeline-driver(\"{}\")", session_id);
        self.invoke(&agent_id, "initialize-timeline", &[timeline_wave, aggregation_wave])
    }

    /// Send an event to an EventProcessor leaf worker.
    ///
    /// `event_wave` is WAVE-encoded Event, e.g.:
    /// `{time: 100, event: [("playerStateChange", string-value("play"))]}`
    pub fn add_event(
        &self,
        worker_name: &str,
        event_wave: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent_id = format!("event-processor(\"{}\")", worker_name);
        self.invoke(&agent_id, "add-event", &[event_wave])
    }

    /// Query a leaf EventProcessor's result at a point in time.
    pub fn get_leaf_result(
        &self,
        worker_name: &str,
        t1: u64,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent_id = format!("event-processor(\"{}\")", worker_name);
        let t1_str = t1.to_string();
        self.invoke(&agent_id, "get-leaf-result", &[&t1_str])
    }

    /// Query a derived TimelineProcessor's result at a point in time.
    pub fn get_derived_result(
        &self,
        worker_name: &str,
        t1: u64,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent_id = format!("timeline-processor(\"{}\")", worker_name);
        let t1_str = t1.to_string();
        self.invoke(&agent_id, "get-derived-result", &[&t1_str])
    }

    /// Query an Aggregator's accumulated result.
    pub fn get_aggregation_result(
        &self,
        aggregator_worker_name: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent_id = format!("aggregator(\"{}\")", aggregator_worker_name);
        self.invoke(&agent_id, "get-aggregation-result", &[])
    }

    /// Extract the result value from a Golem invoke response JSON.
    ///
    /// Response format: `{"result_json": {"value": <result>}, ...}`
    pub fn extract_result(response: &serde_json::Value) -> Option<&serde_json::Value> {
        response
            .get("result_json")
            .and_then(|r| r.get("value"))
    }
}
