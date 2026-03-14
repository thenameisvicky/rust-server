use crate::state::AppState;
use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct JobPayload {
    client_id: String,
    prompt: String,
}

#[derive(Deserialize, Debug)]
struct LLMResponse {
    response: String,
}

pub async fn run(state: Arc<AppState>) {
    let channel = state.amqp.create_channel().await.unwrap();

    channel
        .queue_declare(
            "LLM_INFERENCE",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let mut consumer = channel
        .basic_consume(
            "LLM_INFERENCE",
            "rust_worker",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    println!("Consumer ready!");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.unwrap();

        let payload: JobPayload = serde_json::from_slice(&delivery.data).unwrap();

        println!("Received prompt: {}", payload.prompt);

        let response = match state
            .http_client
            .post(format!("{}/api/generate", state.config.ollama_url))
            .json(&serde_json::json!({
                "model": "llama3.2",
                "prompt": payload.prompt,
                "stream": false
            }))
            .send()
            .await
        {
            Ok(r) => r,
            Err(err) => {
                let tx = {
                    let clients = state.clients.lock().await;
                    clients.get(&payload.client_id).cloned()
                };

                if let Some(tx) = tx {
                    let _ = tx.send(format!("LLM error: {}", err)).await;
                }

                continue;
            }
        };

        let body = response.text().await.unwrap();

        let llm_resp: LLMResponse = serde_json::from_str(&body).unwrap();

        println!("LLM response {:?}", llm_resp);

        let tx = {
            let clients = state.clients.lock().await;
            clients.get(&payload.client_id).cloned()
        };

        if let Some(tx) = tx {
            let _ = tx.send(llm_resp.response.clone()).await;
        }

        delivery.ack(BasicAckOptions::default()).await.unwrap();
    }
}
