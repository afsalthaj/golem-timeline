//! Standalone Kafka producer binary.
//!
//! Generates events and publishes them to Kafka.
//! Can be used independently for cloud deployments.
//!
//! # Environment variables
//! - `KAFKA_BROKER`: Kafka broker address (default: "localhost:9092")
//! - `KAFKA_TOPIC`: Kafka topic name (default: "timeline-events")
//! - `SESSION_COUNT`: Number of sessions to generate (default: 10)
//! - `EVENTS_PER_SESSION`: Events per session (default: 100)

#[tokio::main]
async fn main() {
    env_logger::init();

    let broker = std::env::var("KAFKA_BROKER").unwrap_or_else(|_| "localhost:9092".into());
    let topic = std::env::var("KAFKA_TOPIC").unwrap_or_else(|_| "timeline-events".into());
    let session_count: usize = std::env::var("SESSION_COUNT")
        .unwrap_or_else(|_| "10".into())
        .parse()
        .expect("SESSION_COUNT must be a number");
    let events_per_session: usize = std::env::var("EVENTS_PER_SESSION")
        .unwrap_or_else(|_| "100".into())
        .parse()
        .expect("EVENTS_PER_SESSION must be a number");

    println!("Kafka producer config: broker={broker}, topic={topic}, sessions={session_count}, events_per_session={events_per_session}");

    let config = event_generator::GeneratorConfig {
        events_per_session,
        session_count,
        ..Default::default()
    };
    let sessions = event_generator::generate_sessions(&config);
    let total_events: usize = sessions.iter().map(|s| s.events.len()).sum();

    let start = std::time::Instant::now();
    let producer = kafka_producer::TimelineKafkaProducer::new(&broker, &topic)
        .await
        .expect("Failed to create Kafka producer");

    producer
        .publish_all_sessions(&sessions)
        .await
        .expect("Failed to publish sessions");

    let elapsed = start.elapsed();
    println!(
        "Published {total_events} events across {} sessions in {elapsed:.2?}",
        sessions.len()
    );
}
