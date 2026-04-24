use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use lapin::{Connection, ConnectionProperties};
use prometheus::{Counter, Registry};
use qdrant_client::Qdrant;
use reqwest::Client;

mod api;
mod core;
mod modules;
mod state;

use crate::modules::ingestion::store::create_collection;
use state::{AppState, Config};

#[tokio::main]
async fn main() {
    let amqp_host = std::env::var("AMQP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let qdrant_host = std::env::var("QDRANT_HOST").unwrap_or_else(|_| "http://localhost:6334".to_string());
    let ollama_host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());

    let amqp_addr = format!("amqp://{}:5672/%2f", amqp_host);
    let conn = Connection::connect(&amqp_addr, ConnectionProperties::default())
        .await
        .unwrap();

    let registry = Registry::new();

    let api_requests = Counter::new("api_requests_total", "Total API").unwrap();

    let qdrant_client = Qdrant::from_url(&qdrant_host)
        .timeout(Duration::from_secs(120))
        .connect_timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    registry.register(Box::new(api_requests.clone())).unwrap();

    let state = Arc::new(AppState {
        amqp: Arc::new(conn),
        http_client: Client::new(),
        config: Config {
            ollama_url: ollama_host,
        },
        api_requests,
        prom_registry: registry,
        clients: DashMap::new(),
        qdrant_client,
    });

    create_collection(&state.qdrant_client).await;

    for _ in 0..4 {
        tokio::spawn(core::queue::consumer::run(state.clone()));
    }

    api::router::run(state).await;
}
