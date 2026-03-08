//! Smoke test: verifies the basic end-to-end flow works.
//!
//! Run: `cargo make smoke`

#[cfg(test)]
mod tests {
    use integration_harness::golem_client::GolemClient;
    use integration_harness::harness;

    /// Smoke test: initialize a trivial timeline, feed one event, check result.
    ///
    /// Timeline: latest_event_to_state(playerStateChange)
    ///   - Single leaf node (node-1).
    ///   - Feed one event: {time: 100, playerStateChange: "play"}
    ///   - Assert: get_leaf_result(101) returns "play"
    #[test]
    #[ignore]
    fn test_smoke_single_leaf() {
        harness::ensure_golem_running().expect("Failed to start Golem server");
        harness::deploy_timeline_component().expect("Failed to deploy");

        let client = GolemClient::new();

        // Initialize a simple timeline: latest_event_to_state(playerStateChange)
        let session_id = "smoke-leaf-1";
        let timeline_wave = "{nodes: [tl-latest-event-to-state(\"playerStateChange\")]}";

        let init = client
            .initialize_timeline(session_id, timeline_wave, "none")
            .expect("Failed to initialize timeline");

        let result_value = GolemClient::extract_result(&init);
        println!("Init result: {:?}", result_value);

        // The driver creates one leaf: {session_id}-node-1
        let leaf = format!("{}-node-1", session_id);

        // Feed one event
        let event_wave = r#"{time: 100, event: [("playerStateChange", string-value("play"))]}"#;
        let add_result = client.add_event(&leaf, event_wave).expect("Failed to add event");
        println!("Add event result: {:?}", GolemClient::extract_result(&add_result));

        // Query the leaf result at time 101
        let result = client.get_leaf_result(&leaf, 101).expect("Failed to get leaf result");
        let result_str = serde_json::to_string(&result).unwrap();
        println!("Leaf result at t=101: {}", result_str);

        assert!(
            result_str.contains("play"),
            "Expected 'play' in result, got: {}",
            result_str
        );

        println!("✓ Smoke test passed!");
    }

    /// Smoke test with multiple events: verify state transitions.
    #[test]
    #[ignore]
    fn test_smoke_state_transitions() {
        harness::ensure_golem_running().expect("Failed to start Golem server");
        harness::deploy_timeline_component().expect("Failed to deploy");

        let client = GolemClient::new();
        let session_id = "smoke-trans-1";

        client
            .initialize_timeline(
                session_id,
                "{nodes: [tl-latest-event-to-state(\"playerStateChange\")]}",
                "none",
            )
            .expect("Failed to init");

        let leaf = format!("{}-node-1", session_id);

        // Feed sequence: init → buffer → play
        let events = [
            r#"{time: 10, event: [("playerStateChange", string-value("init"))]}"#,
            r#"{time: 20, event: [("playerStateChange", string-value("buffer"))]}"#,
            r#"{time: 30, event: [("playerStateChange", string-value("play"))]}"#,
        ];

        for event in &events {
            client.add_event(&leaf, event).expect("Failed to add event");
        }

        // Query at t=25 should give "buffer"
        let result = client.get_leaf_result(&leaf, 25).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        println!("Result at t=25: {}", s);
        assert!(s.contains("buffer"), "Expected 'buffer' at t=25, got: {}", s);

        // Query at t=31 should give "play"
        let result = client.get_leaf_result(&leaf, 31).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        println!("Result at t=31: {}", s);
        assert!(s.contains("play"), "Expected 'play' at t=31, got: {}", s);

        println!("✓ State transitions test passed!");
    }
}
