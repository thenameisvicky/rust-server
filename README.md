# Architecture

flowchart TD
    REST["REST API - Axum Router"]
    WS["WebSocket Endpoint /ws"]
    AppState["AppState: RabbitMQ, WS Broadcast, HTTP Client, Config, Prometheus"]
    LLM["LLM Module - Ollama API"]
    Chat["Chat Agent Module - routes/logic"]
    Analytics["Analytics Module"]
    Campaigns["Campaigns Module"]
    Feed["Feed Module"]

    REST -->|Routes| Chat
    REST -->|Routes| Analytics
    REST -->|Routes| Campaigns
    REST -->|Routes| Feed

    Chat --> AppState
    Analytics --> AppState
    Campaigns --> AppState
    Feed --> AppState

    WS --> AppState
    AppState --> LLM
    LLM --> AppState
    WS -->|Broadcast| AppState
