//! Event generator for timeline analytics testing.
//!
//! Generates realistic event streams (e.g., CIRR playback events) that can be
//! published to Kafka for both local integration tests and cloud deployments.
//!
//! # Usage
//! ```rust,ignore
//! let events = generate_cirr_session_events("sess-1", "akamai");
//! // Returns ~1000 events for a single playback session
//! ```

use serde::{Deserialize, Serialize};

/// An event value matching the Golem timeline EventValue type.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventValue {
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BoolValue(bool),
}

/// A single event to be ingested by the timeline system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub time: u64,
    pub event: Vec<(String, EventValue)>,
}

/// Configuration for event generation.
pub struct GeneratorConfig {
    /// Number of events to generate per session.
    pub events_per_session: usize,
    /// Number of sessions to generate.
    pub session_count: usize,
    /// CDN values to distribute sessions across (for aggregation testing).
    pub cdn_values: Vec<String>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            events_per_session: 100,
            session_count: 10,
            cdn_values: vec!["akamai".into(), "cloudfront".into(), "fastly".into()],
        }
    }
}

/// A generated session containing its ID, CDN, and event stream.
pub struct GeneratedSession {
    pub session_id: String,
    pub cdn: String,
    pub events: Vec<Event>,
}

/// Generate CIRR-style playback events for a single session.
///
/// Events include playerStateChange (init, buffer, play, pause, seek)
/// and cdnChange events, with realistic timing.
pub fn generate_cirr_session_events(session_id: &str, cdn: &str) -> Vec<Event> {
    generate_cirr_session_events_n(session_id, cdn, 100)
}

fn hash_session(session_id: &str) -> u64 {
    let mut h: u64 = 5381;
    for b in session_id.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    h
}

fn lcg_next(state: u64) -> u64 {
    state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)
}

pub fn generate_cirr_session_events_n(session_id: &str, cdn: &str, count: usize) -> Vec<Event> {
    let mut events = Vec::with_capacity(count);
    let mut rng = hash_session(session_id);
    let mut time: u64 = 1;

    // Event 0: init
    events.push(Event {
        time,
        event: vec![("playerStateChange".into(), EventValue::StringValue("init".into()))],
    });
    time += 1;

    // Event 1: cdn assignment
    events.push(Event {
        time,
        event: vec![("cdnChange".into(), EventValue::StringValue(cdn.into()))],
    });
    time += 1;

    // Remaining events: realistic state machine
    let mut current_state = "buffer"; // after init we buffer first
    let mut i = 2;
    while i < count {
        rng = lcg_next(rng);
        let time_step = (rng % 3) + 1; // 1-3
        time += time_step;

        let next_state = match current_state {
            "buffer" => "play",
            "play" => {
                rng = lcg_next(rng);
                let r = rng % 10;
                if r < 2 {
                    "buffer"
                } else if r < 4 {
                    "pause"
                } else if r == 4 {
                    // Insert a seek event before continuing to play
                    events.push(Event {
                        time,
                        event: vec![("userAction".into(), EventValue::StringValue("seek".into()))],
                    });
                    i += 1;
                    if i >= count {
                        break;
                    }
                    rng = lcg_next(rng);
                    time += (rng % 3) + 1;
                    "play"
                } else {
                    "play"
                }
            }
            "pause" => {
                rng = lcg_next(rng);
                if rng % 3 == 0 {
                    "buffer"
                } else {
                    "play"
                }
            }
            _ => "buffer",
        };

        current_state = next_state;
        events.push(Event {
            time,
            event: vec![(
                "playerStateChange".into(),
                EventValue::StringValue(current_state.into()),
            )],
        });
        i += 1;
    }

    events
}

/// Generate multiple sessions worth of events according to the config.
///
/// Sessions are distributed across CDN values round-robin.
/// Returns sessions that can be published to Kafka topics.
pub fn generate_sessions(config: &GeneratorConfig) -> Vec<GeneratedSession> {
    (0..config.session_count)
        .map(|i| {
            let session_id = format!("sess-{}", i);
            let cdn = &config.cdn_values[i % config.cdn_values.len()];
            let events =
                generate_cirr_session_events_n(&session_id, cdn, config.events_per_session);
            GeneratedSession {
                session_id,
                cdn: cdn.clone(),
                events,
            }
        })
        .collect()
}
