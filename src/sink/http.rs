use serde_json::Value;
use std::{collections::HashMap, convert::Infallible, net::Ipv4Addr};
use warp::http::StatusCode;
use warp::Filter;

use crate::types::Stores;

pub async fn create_http_server(
    map: HashMap<String, String>,
    stores: Stores,
) -> anyhow::Result<()> {
    let endpoint = map
        .get("endpoint")
        .ok_or(anyhow::Error::msg(
            "No Key `endpoint` provided for HTTP Sink Type in Config File.",
        ))?
        .to_string();

    // Get Server Ip
    let host = map
        .get("host")
        .ok_or(anyhow::Error::msg("Host not Present."))?
        .parse::<Ipv4Addr>()?;

    // Get Server Port
    let port = map
        .get("port")
        .ok_or(anyhow::Error::msg("Port not Present."))?
        .parse::<u16>()?;

    let sink = warp::post()
        .and(warp::path(endpoint))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(warp::any().map(move || stores.clone()))
        .and_then(write_to_store);

    warp::serve(sink).run((host, port)).await;
    Ok(())
}

async fn write_to_store(val: Value, stores: Stores) -> Result<impl warp::Reply, Infallible> {
    for store in stores.write().await.iter_mut() {
        store.store(val.clone()).await.unwrap();
    }
    Ok(StatusCode::ACCEPTED)
}
