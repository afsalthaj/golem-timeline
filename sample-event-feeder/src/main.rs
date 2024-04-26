use std::env;
use std::fmt::Error;
use serde::{Serialize, Deserialize};
use tokio;
use log;
use reqwest::Client;

use futures::TryStreamExt;
use pulsar::{
    authentication::oauth2::OAuth2Authentication, Authentication, Consumer, DeserializeMessage,
    Payload, Pulsar, SubType, TokioExecutor,
};
use pulsar::error::AuthenticationError::Custom;
use reqwest::header::{CONTENT_TYPE, HeaderName, HeaderValue};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub time: u64,
    pub event: Vec<(String, EventValue)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum EventValue{
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BoolValue(bool),
}

impl DeserializeMessage for Event {
    type Output = Result<Event, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomBody {
    params: Vec<Event>
}

#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "non-persistent://public/default/test".to_string());

    let mut builder = Pulsar::builder(addr, TokioExecutor);

    if let Ok(token) = env::var("PULSAR_TOKEN") {
        let authentication = Authentication {
            name: "token".to_string(),
            data: token.into_bytes(),
        };

        builder = builder.with_auth(authentication);
    } else if let Ok(oauth2_cfg) = env::var("PULSAR_OAUTH2") {
        builder = builder.with_auth_provider(OAuth2Authentication::client_credentials(
            serde_json::from_str(oauth2_cfg.as_str())
                .unwrap_or_else(|_| panic!("invalid oauth2 config [{}]", oauth2_cfg.as_str())),
        ));
    }

    let pulsar: Pulsar<_> = builder.build().await?;

    let mut consumer: Consumer<Event, _> = pulsar
        .consumer()
        .with_topic(topic)
        .with_consumer_name("sample-playback-consumer")
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("test_subscription")
        .build()
        .await?;

    let client = Client::new();


    let component_id = env::var("COMPONENT_ID").expect("Provide COMPONENT_ID. You can understand from the output logs of quick_test.sh which was used to register timeline with Golem");
    let worker_name = env::var("WORKER_NAME").expect("Provide WORKER_ID. This should correspond to the worker that directly process events. You can understand from the output logs of quick_test.sh which was used to register timeline with Golem");


    let url = format!("http://localhost:9005/v2/components/{}/workers/{}/key", component_id, worker_name);

    let mut counter = 0usize;
    while let Some(msg) = consumer.try_next().await? {
        consumer.ack(&msg).await?;
        log::info!("metadata: {:?}", msg.metadata());
        log::info!("id: {:?}", msg.message_id());
        let data = match msg.deserialize() {
            Ok(data) => {
                dbg!("got {} messages", data.clone());


                let response = client
                    .post(&url)
                    .send()
                    .await.map_err(|error| pulsar::Error::Custom(error.to_string()))?;


                let response_text = response.text().await.map_err(|error| pulsar::Error::Custom(error.to_string()))?;
                let value: Value = serde_json::from_str(&response_text).map_err(|error| pulsar::Error::Custom(error.to_string()))?;

                // Extract value from response
                let invocation_key =
                    value["value"].as_str().ok_or(pulsar::Error::Custom("Missing value field".to_string()))?;

                dbg!("The invocation_key key is: {}", invocation_key);

                let invoke_url =
                    format!("http://localhost:9005/v2/components/{}/workers/{}/invoke-and-await?invocation-key={}&function={}", component_id, worker_name, invocation_key, "timeline:event-processor/api/add-event");


                let params = json!({"params": [data]});

                dbg!(params.to_string());

                // Second POST request
                 let result = client
                     .post(&invoke_url)
                     .json(&params)
                     .send()
                     .await.map_err(|error| pulsar::Error::Custom(error.to_string()))?;

                dbg!(result);

            },
            Err(e) => {
                log::error!("could not deserialize message: {:?}", e);
                break;
            }
        };

        counter += 1;


        if counter > 10 {
            consumer.close().await.expect("Unable to close consumer");
            break;
        }
    }

    Ok(())
}