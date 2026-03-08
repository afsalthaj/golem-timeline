//! Kafka consumer that feeds events to Golem workers via REST API.
//!
//! Consumes events from a Kafka topic and invokes Golem EventProcessor workers
//! using the Golem REST API. Reusable as a library (for test harness) or
//! as a standalone binary (for cloud deployments).
//!
//! # As a library
//! ```rust,ignore
//! let feeder = GolemFeeder::new(feeder_config).await?;
//! feeder.run_until_idle().await?;
//! ```
//!
//! # Golem REST API
//! Events are sent via:
//! ```text
//! POST /v1/components/{component_id}/workers/{worker_name}/invoke-and-await
//!      ?function=timeline:event-processor/api/add-event
//! Body: {"params": [<event>]}
//! ```

/// Configuration for the Golem feeder.
pub struct FeederConfig {
    /// Kafka broker address (e.g., "localhost:9092").
    pub kafka_broker: String,
    /// Kafka topic to consume from.
    pub kafka_topic: String,
    /// Kafka consumer group ID.
    pub consumer_group: String,
    /// Golem API base URL (e.g., "http://localhost:9005").
    pub golem_base_url: String,
    /// Golem component ID (UUID) for the timeline-core component.
    pub component_id: String,
    /// List of leaf worker names to feed events to.
    /// These are the EventProcessor workers created by the TimelineDriver.
    pub leaf_worker_names: Vec<String>,
}

use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;

/// Consumes from Kafka and feeds events to Golem workers.
pub struct GolemFeeder {
    config: FeederConfig,
    consumer: StreamConsumer,
    client: reqwest::Client,
}

impl GolemFeeder {
    /// Create a new feeder connected to Kafka and ready to invoke Golem workers.
    pub async fn new(config: FeederConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.kafka_broker)
            .set("group.id", &config.consumer_group)
            .set("auto.offset.reset", "earliest")
            .set("enable.auto.commit", "true")
            .create()?;

        consumer.subscribe(&[&config.kafka_topic])?;

        let client = reqwest::Client::new();

        Ok(Self {
            config,
            consumer,
            client,
        })
    }

    /// Consume events from Kafka and feed them to Golem workers.
    ///
    /// For each event consumed:
    /// 1. Deserialize the event from JSON
    /// 2. Send it to each leaf worker via the Golem REST API
    ///    POST /v1/components/{component_id}/workers/{worker}/invoke-and-await
    ///         ?function=timeline:event-processor/api/add-event
    ///    Body: {"params": [<event>]}
    /// 3. Acknowledge the Kafka message
    ///
    /// Runs until no more messages are available (for testing) or indefinitely (for cloud).
    pub async fn run_until_idle(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut count: u64 = 0;
        let function = "timeline:core/event-processor.{add-event}";

        loop {
            let msg = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                self.consumer.recv(),
            )
            .await;

            let msg = match msg {
                Ok(Ok(m)) => m,
                Ok(Err(e)) => {
                    log::error!("Kafka consumer error: {}", e);
                    continue;
                }
                Err(_) => {
                    log::info!("No messages for 5 seconds, considering idle");
                    break;
                }
            };

            let payload = match msg.payload_view::<str>() {
                Some(Ok(s)) => s,
                Some(Err(e)) => {
                    log::error!("Invalid UTF-8 payload: {}", e);
                    continue;
                }
                None => {
                    log::warn!("Empty payload, skipping");
                    continue;
                }
            };

            let event: serde_json::Value = match serde_json::from_str(payload) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Failed to parse event JSON: {}", e);
                    continue;
                }
            };

            for worker_name in &self.config.leaf_worker_names {
                let params = serde_json::json!({ "params": [event] });
                match self.invoke_worker(worker_name, function, params).await {
                    Ok(resp) => {
                        log::debug!("Worker {} response: {}", worker_name, resp);
                    }
                    Err(e) => {
                        log::error!("Failed to invoke worker {}: {}", worker_name, e);
                    }
                }
            }

            count += 1;
            log::info!("Processed event #{}", count);
        }

        Ok(count)
    }

    /// Invoke a single Golem worker function via REST API.
    ///
    /// The worker_name is the raw name (e.g., "sess-1-node-4").
    /// We construct the Golem agent_id as `event-processor("worker_name")`.
    async fn invoke_worker(
        &self,
        worker_name: &str,
        function: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let agent_id = format!("event-processor(\"{}\")", worker_name);
        let url = format!(
            "{}/v1/components/{}/workers/{}/invoke-and-await",
            self.config.golem_base_url, self.config.component_id, agent_id,
        );

        let resp = self
            .client
            .post(&url)
            .query(&[("function", function)])
            .json(&params)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(resp)
    }
}
