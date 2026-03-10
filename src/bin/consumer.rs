use futures_util::StreamExt;
use lapin::{
    options::*,
    types::FieldTable,
    Connection, ConnectionProperties
};
use serde::Deserialize;

#[derive(Deserialize)]
struct JobPayload {
    prompt: String
}

#[tokio::main]
async fn main() {

    let conn = Connection::connect(
        "amqp://127.0.0.1:5672/%2f",
        ConnectionProperties::default()
    ).await.unwrap();

    let channel = conn.create_channel().await.unwrap();

    channel.queue_declare(
        "LLM_INFERENCE",
        QueueDeclareOptions::default(),
        FieldTable::default()
    ).await.unwrap();

    let mut consumer = channel.basic_consume(
        "LLM_INFERENCE",
        "rust_worker",
        BasicConsumeOptions::default(),
        FieldTable::default()
    ).await.unwrap();

    println!("Consumer ready!");

    while let Some(delivery) = consumer.next().await {

        let delivery = delivery.unwrap();

        let payload: JobPayload =
            serde_json::from_slice(&delivery.data).unwrap();

        println!("Received prompt: {}", payload.prompt);

        // Call Ollama
        // let response = reqwest::Client::new()
        //     .post("http://localhost:11434/api/generate")
        //     .json(&serde_json::json!({
        //         "model": "qwen2.5:3b",
        //         "prompt": payload.prompt,
        //         "stream": false
        //     }))
        //     .send()
        //     .await
        //     .unwrap();

        let text = "Response!";

        println!("LLM response: {}", text);

        delivery.ack(BasicAckOptions::default()).await.unwrap();
    }
}