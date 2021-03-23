mod http;
mod tcp;
use std::collections::HashMap;

pub use http::create_http_server;
pub use tcp::create_tcp_server;

use crate::types::Stores;

pub enum SinkType {
    Tcp,
    Http,
}

impl From<&str> for SinkType {
    fn from(s: &str) -> Self {
        match s {
            "tcp" => SinkType::Tcp,
            "http" => SinkType::Http,
            _ => SinkType::Tcp,
        }
    }
}

pub async fn serve(
    sink_type: SinkType,
    map: HashMap<String, String>,
    stores: Stores,
) -> anyhow::Result<()> {
    match sink_type {
        SinkType::Http => create_http_server(map, stores).await,
        SinkType::Tcp => create_tcp_server(map, stores).await,
    }
}
