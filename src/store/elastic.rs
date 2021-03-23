use super::Store;
use async_trait::async_trait;
use elasticsearch::{Elasticsearch, IndexParts};
use serde_json::Value;

pub struct Elastic {
    client: Elasticsearch,
    host: String,
    port: u16,
}
impl Elastic {}
#[async_trait]
impl Store for Elastic {
    async fn store(&self, val: Value) -> anyhow::Result<()> {
        self.client
            .index(IndexParts::IndexId("tweets", "1"))
            .body(val)
            .send()
            .await?;
        Ok(())
    }
}
