use lapin::Connection;
use prometheus::{Counter, Registry};
use reqwest::Client;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{ mpsc, Mutex};

pub struct Config {
    pub ollama_url: String,
}

pub struct AppState {
    pub amqp: Arc<Connection>,
    pub http_client: Client,
    pub config: Config,
    pub prom_registry: Registry,
    pub api_requests: Counter,
    pub clients: Arc<Mutex<HashMap<String, mpsc::Sender<String>>>>,
}
