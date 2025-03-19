use std::env;

use log;
use pulsar::{
    authentication::oauth2::OAuth2Authentication, producer, Authentication, Error as PulsarError,
    Pulsar, SerializeMessage, TokioExecutor,
};
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub time: u64,
    pub event: Vec<(String, EventValue)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum EventValue {
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BoolValue(bool),
}

impl SerializeMessage for Event {
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        let payload = serde_json::to_vec(&input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message { payload, ..Default::default() })
    }
}

#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    env_logger::init();

    let addr =
        env::var("PULSAR_ADDRESS").ok().unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "non-persistent://public/default/test".to_string());

    let mut builder = Pulsar::builder(addr, TokioExecutor);

    if let Ok(token) = env::var("PULSAR_TOKEN") {
        let authentication = Authentication { name: "token".to_string(), data: token.into_bytes() };

        builder = builder.with_auth(authentication);
    } else if let Ok(oauth2_cfg) = env::var("PULSAR_OAUTH2") {
        builder = builder.with_auth_provider(OAuth2Authentication::client_credentials(
            serde_json::from_str(oauth2_cfg.as_str())
                .unwrap_or_else(|_| panic!("invalid oauth2 config [{}]", oauth2_cfg.as_str())),
        ));
    }

    let pulsar: Pulsar<_> = builder.build().await?;
    let mut producer =
        pulsar.producer().with_topic(topic).with_name("sample-playback-events").build().await?;

    // CIRR events
    let events = vec![
        Event {
            time: 1,
            event: vec![(
                "playerStateChange".to_string(),
                EventValue::StringValue("init".to_string()),
            )],
        },
        Event {
            time: 2,
            event: vec![(
                "cdnChange".to_string(),
                EventValue::StringValue("CDN1".to_string()),
            )],
        },
        Event {
            time: 4,
            event: vec![(
                "playerStateChange".to_string(),
                EventValue::StringValue("buffer".to_string()),
            )],
        },
        Event {
            time: 5,
            event: vec![(
                "playerStateChange".to_string(),
                EventValue::StringValue("play".to_string()),
            )],
        },
        Event {
            time: 6,
            event: vec![(
                "playerStateChange".to_string(),
                EventValue::StringValue("buffer".to_string()),
            )],
        },
        Event {
            time: 7,
            event: vec![(
                "userAction".to_string(),
                EventValue::StringValue("seek".to_string()),
            )],
        },
        Event {
            time: 9,
            event: vec![(
                "playerStateChange".to_string(),
                EventValue::StringValue("play".to_string()),
            )],
        },
        Event {
            time: 10,
            event: vec![(
                "playerStateChange".to_string(),
                EventValue::StringValue("pause".to_string()),
            )],
        },
        Event {
            time: 11,
            event: vec![(
                "cdnChange".to_string(),
                EventValue::StringValue("CDN2".to_string()),
            )],
        },
        Event {
            time: 13,
            event: vec![(
                "playerStateChange".to_string(),
                EventValue::StringValue("buffer".to_string()),
            )],
        },
        Event {
            time: 14,
            event: vec![(
                "playerStateChange".to_string(),
                EventValue::StringValue("play".to_string()),
            )],
        }
    ];

    for event in events {
        producer.send(event.clone()).await?.await.unwrap();

        log::info!("Sending event: {:?}", &event);
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

    Ok(())
}