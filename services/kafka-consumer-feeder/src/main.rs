// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
//! Standalone Kafka consumer/feeder binary.
//!
//! Consumes events from Kafka and feeds them to Golem agents.
//! Can be used independently for cloud deployments.
//!
//! # Environment variables
//! - `KAFKA_BROKER`: Kafka broker address (default: "localhost:9092")
//! - `KAFKA_TOPIC`: Kafka topic name (default: "timeline-events")
//! - `KAFKA_GROUP`: Consumer group ID (default: "timeline-feeder")
//! - `GOLEM_BASE_URL`: Golem API base URL (default: "http://localhost:9005")
//! - `COMPONENT_ID`: Golem component ID (UUID)
//! - `LEAF_WORKERS`: Comma-separated list of leaf worker names

use kafka_consumer_feeder::{FeederConfig, GolemFeeder};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let kafka_broker =
        env::var("KAFKA_BROKER").unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka_topic =
        env::var("KAFKA_TOPIC").unwrap_or_else(|_| "timeline-events".to_string());
    let consumer_group =
        env::var("KAFKA_GROUP").unwrap_or_else(|_| "timeline-feeder".to_string());
    let golem_base_url =
        env::var("GOLEM_BASE_URL").unwrap_or_else(|_| "http://localhost:9005".to_string());
    let component_id = env::var("COMPONENT_ID").expect("COMPONENT_ID must be set");
    let leaf_agents = env::var("LEAF_WORKERS").expect("LEAF_WORKERS must be set");

    let leaf_agent_names: Vec<String> = leaf_agents
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    log::info!(
        "Starting Kafka consumer/feeder: broker={}, topic={}, group={}, agents={:?}",
        kafka_broker, kafka_topic, consumer_group, leaf_agent_names,
    );

    let config = FeederConfig {
        kafka_broker,
        kafka_topic,
        consumer_group,
        golem_base_url,
        component_id,
        leaf_agent_names,
    };

    let start = std::time::Instant::now();

    let feeder = GolemFeeder::new(config)
        .await
        .expect("Failed to create GolemFeeder");

    let count = feeder
        .run_until_idle()
        .await
        .expect("Failed to run feeder");

    let elapsed = start.elapsed();
    println!("Fed {} events to Golem agents in {:.2?}", count, elapsed);
}