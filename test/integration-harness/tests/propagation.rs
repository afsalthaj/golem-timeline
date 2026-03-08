//! Propagation tests: verify the push-based cascade works correctly.
//!
//! Run: `cargo make propagation`

#[cfg(test)]
mod tests {
    use integration_harness::golem_client::GolemClient;
    use integration_harness::harness;

    /// Test boolean logic (And, Not) propagation.
    ///
    /// Timeline: has_existed(status == "active") && !has_existed(status == "error")
    ///
    /// Graph (flat, nodes[0]=root):
    ///   0: And(1, 2)
    ///   1: TlHasExisted(status == "active")
    ///   2: Negation(3)
    ///   3: TlHasExisted(status == "error")
    ///
    /// Node numbering from setup_node (depth-first):
    ///   node-1: And (TimelineProcessor)
    ///   node-2: TlHasExisted(status=="active") (EventProcessor LEAF)
    ///   node-3: Not (TimelineProcessor)
    ///   node-4: TlHasExisted(status=="error") (EventProcessor LEAF)
    #[test]
    #[ignore]
    fn test_boolean_logic_propagation() {
        harness::ensure_golem_running().expect("Failed to start Golem server");
        harness::deploy_timeline_component().expect("Failed to deploy");

        let client = GolemClient::new();
        let session_id = "prop-bool-1";

        let timeline_wave = concat!(
            "{nodes: [",
            "and((1, 2)), ",
            "tl-has-existed({col-name: \"status\", value: string-value(\"active\"), op: equal}), ",
            "negation(3), ",
            "tl-has-existed({col-name: \"status\", value: string-value(\"error\"), op: equal})",
            "]}"
        );

        client
            .initialize_timeline(session_id, timeline_wave, "none")
            .expect("Failed to init boolean timeline");

        let leaf_active = format!("{}-node-2", session_id);
        let leaf_error = format!("{}-node-4", session_id);
        let root = format!("{}-node-1", session_id);

        // Step 1: Send {status: "active"} at time 10
        let event1 = r#"{time: 10, event: [("status", string-value("active"))]}"#;
        client.add_event(&leaf_active, event1).unwrap();
        client.add_event(&leaf_error, event1).unwrap();

        // After: leaf_active=true (predicate matches), leaf_error=false ("active"!="error")
        // Not(false)=true, And(true, true)=true
        let result = client.get_derived_result(&root, 11).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        println!("Root at t=11 after 'active': {}", s);
        assert!(s.contains("true"), "Expected true after 'active', got: {}", s);

        // Step 2: Send {status: "error"} at time 20
        let event2 = r#"{time: 20, event: [("status", string-value("error"))]}"#;
        client.add_event(&leaf_active, event2).unwrap();
        client.add_event(&leaf_error, event2).unwrap();

        // After: leaf_error=true, Not(true)=false, And(true, false)=false
        let result = client.get_derived_result(&root, 21).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        println!("Root at t=21 after 'error': {}", s);
        assert!(s.contains("false"), "Expected false after 'error', got: {}", s);

        println!("✓ Boolean logic propagation test passed!");
    }

    /// Test the CIRR expression cascade.
    ///
    /// Timeline:
    ///   duration_where(
    ///     has_existed(playerStateChange == "play")
    ///     && !has_existed_within(playerStateChange == "seek", 5)
    ///     && latest_event_to_state(playerStateChange) == "buffer"
    ///   )
    ///
    /// Graph (flat, nodes[0]=root):
    ///   0: TlDurationWhere(1)
    ///   1: And(2, 6)
    ///   2: And(3, 4)
    ///   3: TlHasExisted(playerStateChange == "play")
    ///   4: Negation(5)
    ///   5: TlHasExistedWithin(playerStateChange == "seek", 5)
    ///   6: Comparison(EqualTo, 7, "buffer")
    ///   7: TlLatestEventToState("playerStateChange")
    ///
    /// Leaves: node-4, node-6, node-8. Root: node-1.
    #[test]
    #[ignore]
    fn test_cirr_propagation() {
        harness::ensure_golem_running().expect("Failed to start Golem server");
        harness::deploy_timeline_component().expect("Failed to deploy");

        let client = GolemClient::new();
        let session_id = "prop-cirr-1";

        let timeline_wave = concat!(
            "{nodes: [",
            "tl-duration-where(1), ",
            "and((2, 6)), ",
            "and((3, 4)), ",
            "tl-has-existed({col-name: \"playerStateChange\", value: string-value(\"play\"), op: equal}), ",
            "negation(5), ",
            "tl-has-existed-within(({col-name: \"playerStateChange\", value: string-value(\"seek\"), op: equal}, 5)), ",
            "comparison((equal-to, 7, string-value(\"buffer\"))), ",
            "tl-latest-event-to-state(\"playerStateChange\")",
            "]}"
        );

        let init = client
            .initialize_timeline(session_id, timeline_wave, "none")
            .expect("Failed to init CIRR timeline");
        println!("CIRR init: {:?}", GolemClient::extract_result(&init));

        // Leaves: node-4, node-6, node-8
        let leaves: Vec<String> = [4, 6, 8]
            .iter()
            .map(|n| format!("{}-node-{}", session_id, n))
            .collect();

        // Event 1: playerStateChange = "play" at time 100
        let event1 = r#"{time: 100, event: [("playerStateChange", string-value("play"))]}"#;
        for leaf in &leaves {
            client.add_event(leaf, event1).unwrap();
        }

        // Event 2: playerStateChange = "buffer" at time 200
        let event2 = r#"{time: 200, event: [("playerStateChange", string-value("buffer"))]}"#;
        for leaf in &leaves {
            client.add_event(leaf, event2).unwrap();
        }

        // Root (node-1) should now have DurationWhere counting from time 200
        let root = format!("{}-node-1", session_id);
        let result = client.get_derived_result(&root, 250).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        println!("Root at t=250: {}", s);

        // Should have non-empty results with a positive duration
        assert!(
            !s.contains("\"results\":[]"),
            "Expected non-empty results, got: {}",
            s
        );

        println!("✓ CIRR propagation test passed!");
    }
}
