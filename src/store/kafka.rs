use super::Store;

use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::Value;
use std::{collections::HashMap, time::Duration};
pub struct Kafka {
    topic: String,
    client: FutureProducer,
}
impl Kafka {
    fn new(config: HashMap<String, String>) -> anyhow::Result<Self> {
        let topic = config
            .get("topic")
            .ok_or(anyhow::Error::msg(
                "No Key `topic` provide to Kafka Producer inn config file.",
            ))?
            .to_owned();

        Ok(Self {
            client: ClientConfig::new()
                .set("bootstrap.servers", "1")
                .set("message.timeout.ms", "5000")
                .create()
                .unwrap(),
            topic,
        })
    }
}
#[async_trait]
impl Store for Kafka {
    async fn store(&self, val: Value) -> anyhow::Result<()> {
        &self
            .client
            .send(
                FutureRecord::to(&self.topic)
                    .payload(&format!("{}", val))
                    .key(&format!("Key {}", "kl"))
                    .headers(OwnedHeaders::new().add("header_key", "header_value")),
                Duration::from_secs(0),
            )
            .await
            .unwrap();
        Ok(())
    }
}
