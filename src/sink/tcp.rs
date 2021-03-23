use serde_json::Value;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::types::Stores;

pub async fn create_tcp_server(
    // Config for The Tcp Listener
    map: HashMap<String, String>,
    // Vec of Store i.e sinks.
    stores: Stores,
) -> anyhow::Result<()> {
    // Get Server Ip
    let host = map
        .get("host")
        .ok_or(anyhow::Error::msg("Host not Present."))?
        .as_str();

    // Get Server Port
    let port = map
        .get("port")
        .ok_or(anyhow::Error::msg("Port not Present."))?
        .parse::<u16>()?;

    let listener = TcpListener::bind((host, port)).await?;

    loop {
        let (socket, _addr) = listener.accept().await?;

        let mut stream = BufReader::new(socket);

        let mut buf = String::new();

        while let Ok(_) = stream.read_line(&mut buf).await {
            let val = serde_json::from_str(&buf).unwrap();
            write_to_store(val, stores.clone()).await.unwrap();
        }
    }
}
async fn write_to_store(val: Value, stores: Stores) -> anyhow::Result<()> {
    for store in stores.write().await.iter_mut() {
        store.store(val.clone()).await.unwrap();
    }
    Ok(())
}
