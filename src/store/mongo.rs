use super::Store;

use async_trait::async_trait;
use mongodb::{bson, bson::doc, Client};
use serde_json::Value;
pub struct Mongo {
    // Client to write data to
    client: Client,

    // String slice or string what to use and when to use
    database: String,

    // String slice or string
    collection: String,
}

impl Mongo {}
#[async_trait]
impl Store for Mongo {
    async fn store(&self, val: Value) -> anyhow::Result<()> {
        self.client
            .database(&self.database)
            .collection(&self.collection)
            .insert_one(doc! { "data":  bson::to_bson(&val).unwrap() }, None)
            .await?;
        Ok(())
    }
}
