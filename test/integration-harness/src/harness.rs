// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
//! Test harness orchestration.
//!
//! Manages the lifecycle of infrastructure (Golem, Kafka) and provides
//! helpers for test setup/teardown.
//!
//! The Golem server is started once and shared across all tests in a run.
//! The Makefile is responsible for stopping it after tests complete.

use std::net::TcpStream;
use std::process::Command;
use std::time::Duration;

fn workspace_root() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("Cannot determine workspace root")
}

const GOLEM_PORT: u16 = 9881;

fn golem_is_running() -> bool {
    TcpStream::connect_timeout(
        &format!("127.0.0.1:{GOLEM_PORT}").parse().unwrap(),
        Duration::from_secs(2),
    )
    .is_ok()
}

/// Verify the Golem server is running.
///
/// The Makefile (`cargo make smoke`, etc.) starts and stops the server.
/// This function just confirms it's reachable before tests proceed.
pub fn ensure_golem_running() -> Result<(), Box<dyn std::error::Error>> {
    if golem_is_running() {
        return Ok(());
    }
    Err("Golem server not running. Use `cargo make smoke` (or `cargo make integration`) which starts/stops the server automatically.".into())
}

/// Deploy the timeline component to Golem.
///
/// Runs `golem deploy --yes` from the workspace root.
pub fn deploy_timeline_component() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("golem")
        .args(["deploy", "--yes"])
        .current_dir(workspace_root())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "golem deploy failed.\nstdout: {stdout}\nstderr: {stderr}"
        )
        .into());
    }

    Ok(())
}

/// Ensure Kafka is running via Docker Compose.
///
/// Checks if Kafka is reachable at localhost:9092. If not, starts it via Docker Compose.
pub fn ensure_kafka_running() -> Result<(), Box<dyn std::error::Error>> {
    if TcpStream::connect_timeout(
        &"127.0.0.1:9092".parse().unwrap(),
        Duration::from_secs(2),
    )
    .is_ok()
    {
        return Ok(());
    }

    let status = Command::new("docker")
        .args([
            "compose",
            "-f",
            "test/integration-harness/docker/compose.yml",
            "up",
            "-d",
        ])
        .current_dir(workspace_root())
        .status()?;

    if !status.success() {
        return Err("Failed to start Kafka via docker compose".into());
    }

    // Wait for Kafka to be ready
    for _ in 0..30 {
        std::thread::sleep(Duration::from_secs(1));
        if TcpStream::connect_timeout(
            &"127.0.0.1:9092".parse().unwrap(),
            Duration::from_secs(2),
        )
        .is_ok()
        {
            return Ok(());
        }
    }

    Err("Timeout waiting for Kafka at localhost:9092".into())
}