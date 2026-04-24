# Stardust RAG

Self-hosted, streaming RAG engine for AI-powered SaaS.

## Architecture

```mermaid
sequenceDiagram
    participant Client
    participant REST_API as REST API (Axum)
    participant WS as WebSocket /ws
    participant Chat as Chat retrieval Module
    participant AppState
    participant LLM as LLM Module (Ollama API)

    Client->>REST_API: POST /chat with prompt
    REST_API->>Chat: Forward prompt
    Chat->>AppState: Publish Job to Queue
    AppState->>LLM: Send prompt for inference
    LLM-->>AppState: Return response
    AppState-->>WS: Broadcast response
    WS-->>Client: Receive processed response
```

## Deployment

### GitHub Pages (Landing Page)
The landing page located in `src/views` is automatically deployed to GitHub Pages via the `Production-Client` workflow.

### Docker (Backend Service)
The backend service is built and pushed to Docker Hub via the `Production-Docker` workflow.

### Repository Secrets
To enable automated deployments, set the following secrets in your GitHub repository:
- `DOCKER_USERNAME`: Your Docker Hub username.
- `DOCKER_PASSWORD`: Your Docker Hub personal access token.
- `DEPLOYMENT_EMAIL`: Authorized email for triggering production workflows.

## Development

```bash
# Run locally
cargo run
```
