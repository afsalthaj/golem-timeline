// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
//! Aggregation tests: verify cross-session aggregation works.
//!
//! Run: `cargo make aggregation`

#[cfg(test)]
mod tests {
    use integration_harness::golem_client::GolemClient;
    use integration_harness::harness;

    /// Test aggregation across multiple sessions.
    ///
    /// Timeline per session:
    ///   latest_event_to_state(playerStateChange) == "buffer"
    ///   | aggregate(group_by(cdn), count, sum, avg)
    ///
    /// Graph:
    ///   0: Comparison(EqualTo, 1, "buffer")
    ///   1: TlLatestEventToState("playerStateChange")
    ///
    /// Nodes from setup_node:
    ///   equal-to-1: EqualTo (TimelineProcessor, ROOT)
    ///   latest-event-to-state-2: TlLatestEventToState (EventProcessor, LEAF)
    #[test]
    #[ignore]
    fn test_multi_session_aggregation() {
        harness::ensure_golem_running().expect("Failed to start Golem server");
        harness::deploy_timeline_component().expect("Failed to deploy");

        let client = GolemClient::new();

        let timeline_wave = concat!(
            "{nodes: [",
            "comparison((equal-to, 1, string-value(\"buffer\"))), ",
            "tl-latest-event-to-state(\"playerStateChange\")",
            "]}"
        );

        let agg_wave = "some({group-by-column: \"cdn\", aggregations: [count, sum, avg]})";

        // Initialize 2 sessions, both with cdn="akamai"
        let sessions = ["agg-sess-1", "agg-sess-2"];
        for session_id in &sessions {
            client
                .initialize_timeline(session_id, timeline_wave, agg_wave)
                .expect(&format!("Failed to init {}", session_id));
        }

        // Feed events to each session's leaf (latest-event-to-state-2)
        for session_id in &sessions {
            let leaf = format!("{}-latest-event-to-state-2", session_id);

            // Event with cdn + playerStateChange = "init"
            let event1 = r#"{time: 100, event: [("playerStateChange", string-value("init")), ("cdn", string-value("akamai"))]}"#;
            client.add_event(&leaf, event1).unwrap();

            // Event: playerStateChange = "buffer" → makes EqualTo("buffer") = true
            let event2 = r#"{time: 200, event: [("playerStateChange", string-value("buffer")), ("cdn", string-value("akamai"))]}"#;
            client.add_event(&leaf, event2).unwrap();
        }

        // Query the aggregator: aggregator-cdn-akamai
        let result = client
            .get_aggregation_result("aggregator-cdn-akamai")
            .expect("Failed to get aggregation result");

        let s = serde_json::to_string(&result).unwrap();
        println!("Aggregator result: {}", s);

        // Both sessions should have registered
        assert!(
            s.contains("\"count\":2") || s.contains("\"count\": 2"),
            "Expected count=2, got: {}",
            s
        );

        println!("✓ Multi-session aggregation test passed!");
    }
}