use async_trait::async_trait;

mod elastic;
mod kafka;
mod mongo;
mod postgres;
mod redis;
mod sqlite;

use serde_json::Value;

pub enum StoreType {
    Elastic,
    Mongo,
    Redis,
    Kafka,
    Postgres,
    Sqlite,
}

#[async_trait]
pub trait Store: Sync + Send {
    async fn store(&self, val: Value) -> anyhow::Result<()>;
}

// pub fn create_store<S>(store_type: StoreType) -> Box<dyn Store> {

// }
