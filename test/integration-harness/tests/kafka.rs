//! Kafka pipeline test: event-generator → kafka-producer → Kafka → consume → Golem → assert.
//!
//! Verifies that events survive the full Kafka round-trip and produce correct
//! timeline results when fed to Golem.
//!
//! Run: `cargo make kafka-test`

#[cfg(test)]
mod tests {
    use event_generator::EventValue;
    use integration_harness::golem_client::GolemClient;
    use integration_harness::harness;
    use rdkafka::consumer::{Consumer, StreamConsumer};
    use rdkafka::ClientConfig;
    use rdkafka::Message;

    const KAFKA_BROKER: &str = "localhost:9092";
    const KAFKA_TOPIC: &str = "timeline-kafka-test";

    /// Convert an event-generator Event (JSON from Kafka) back to a WAVE string
    /// suitable for `golem agent invoke`.
    fn event_json_to_wave(json: &serde_json::Value) -> Option<String> {
        let time = json.get("time")?.as_u64()?;
        let event_arr = json.get("event")?.as_array()?;

        let pairs: Vec<String> = event_arr
            .iter()
            .filter_map(|pair| {
                let arr = pair.as_array()?;
                let key = arr.get(0)?.as_str()?;
                let val = arr.get(1)?;

                // Determine which EventValue variant from the JSON
                let wave_val = if let Some(s) = val.get("string-value").and_then(|v| v.as_str()) {
                    format!("string-value(\"{}\")", s)
                } else if let Some(i) = val.get("int-value").and_then(|v| v.as_i64()) {
                    format!("int-value({})", i)
                } else if let Some(f) = val.get("float-value").and_then(|v| v.as_f64()) {
                    format!("float-value({})", f)
                } else if let Some(b) = val.get("bool-value").and_then(|v| v.as_bool()) {
                    format!("bool-value({})", b)
                } else {
                    return None;
                };

                Some(format!("(\"{}\", {})", key, wave_val))
            })
            .collect();

        Some(format!("{{time: {}, event: [{}]}}", time, pairs.join(", ")))
    }

    /// Full Kafka pipeline test:
    ///   1. Generate CIRR events with event-generator
    ///   2. Publish to Kafka with kafka-producer
    ///   3. Consume from Kafka with rdkafka
    ///   4. Feed each event to Golem via GolemClient (golem CLI)
    ///   5. Query the timeline leaf and assert the final state
    ///
    /// Timeline: latest_event_to_state("playerStateChange")
    ///   - Single leaf node (node-1)
    ///   - After all events, the leaf should reflect the last playerStateChange value
    #[test]
    #[ignore]
    fn test_kafka_event_pipeline() {
        harness::ensure_golem_running().expect("Failed to start Golem server");
        harness::deploy_timeline_component().expect("Failed to deploy");
        harness::ensure_kafka_running().expect("Failed to start Kafka");

        let client = GolemClient::new();
        let session_id = "kafka-pipe-1";

        // Initialize a simple timeline
        client
            .initialize_timeline(
                session_id,
                "{nodes: [tl-latest-event-to-state(\"playerStateChange\")]}",
                "none",
            )
            .expect("Failed to init timeline");

        let leaf = format!("{}-node-1", session_id);

        // Generate a small set of events (10 events for speed)
        let events = event_generator::generate_cirr_session_events_n(session_id, "akamai", 10);

        // Find the last playerStateChange event for later assertion
        let last_state_change = events
            .iter()
            .rev()
            .find_map(|e| {
                e.event.iter().find_map(|(k, v)| {
                    if k == "playerStateChange" {
                        if let EventValue::StringValue(s) = v {
                            Some((e.time, s.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .expect("Should have at least one playerStateChange event");

        println!(
            "Last playerStateChange: '{}' at time {}",
            last_state_change.1, last_state_change.0
        );

        // Publish events to Kafka
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let producer =
                kafka_producer::TimelineKafkaProducer::new(KAFKA_BROKER, KAFKA_TOPIC)
                    .await
                    .expect("Failed to create Kafka producer");
            producer
                .publish_session_events(session_id, &events)
                .await
                .expect("Failed to publish events");
        });
        println!("Published {} events to Kafka topic '{}'", events.len(), KAFKA_TOPIC);

        // Consume from Kafka and feed to Golem via CLI
        let consumed = rt.block_on(async {
            let consumer: StreamConsumer = ClientConfig::new()
                .set("bootstrap.servers", KAFKA_BROKER)
                .set("group.id", "kafka-test-consumer")
                .set("auto.offset.reset", "earliest")
                .set("enable.auto.commit", "true")
                .create()
                .expect("Failed to create Kafka consumer");

            consumer
                .subscribe(&[KAFKA_TOPIC])
                .expect("Failed to subscribe");

            let mut count = 0u64;
            loop {
                let msg = tokio::time::timeout(
                    std::time::Duration::from_secs(10),
                    consumer.recv(),
                )
                .await;

                let msg = match msg {
                    Ok(Ok(m)) => m,
                    Ok(Err(e)) => {
                        eprintln!("Kafka consumer error: {}", e);
                        continue;
                    }
                    Err(_) => break, // timeout = no more messages
                };

                let payload = match msg.payload_view::<str>() {
                    Some(Ok(s)) => s.to_string(),
                    _ => continue,
                };

                let json: serde_json::Value = match serde_json::from_str(&payload) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to parse event JSON: {}", e);
                        continue;
                    }
                };

                // Only feed events that contain playerStateChange
                // (skip cdnChange, userAction, etc. since our timeline only tracks playerStateChange)
                let has_player_state = json
                    .get("event")
                    .and_then(|e| e.as_array())
                    .map(|arr| arr.iter().any(|pair| {
                        pair.as_array()
                            .and_then(|a| a.get(0))
                            .and_then(|k| k.as_str())
                            .map(|k| k == "playerStateChange")
                            .unwrap_or(false)
                    }))
                    .unwrap_or(false);

                if !has_player_state {
                    count += 1;
                    continue;
                }

                let wave = match event_json_to_wave(&json) {
                    Some(w) => w,
                    None => {
                        eprintln!("Failed to convert event to WAVE: {}", payload);
                        continue;
                    }
                };

                client
                    .add_event(&leaf, &wave)
                    .expect("Failed to add event via GolemClient");
                count += 1;
            }
            count
        });

        println!("Consumed and fed {} events from Kafka", consumed);
        assert!(consumed > 0, "Should have consumed at least one event");

        // Query the leaf at a time after the last event
        let query_time = last_state_change.0 + 1;
        let result = client
            .get_leaf_result(&leaf, query_time)
            .expect("Failed to get leaf result");
        let s = serde_json::to_string(&result).unwrap();
        println!("Leaf result at t={}: {}", query_time, s);

        assert!(
            s.contains(&last_state_change.1),
            "Expected '{}' in result, got: {}",
            last_state_change.1,
            s
        );

        println!("✓ Kafka pipeline test passed!");
    }
}
