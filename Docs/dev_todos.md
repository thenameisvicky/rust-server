# Stardust AI use cases

- Big or small SaaS who needs to provide context to their Agents from documents.
- Internal coding agent needs context from the code base (Link github and document prepared automatically and injected into quadrant something like this).
And more i need to research and add it here close deals with them.

# Todos stardustAI

- Intelligence layer for AI systems — retrieve the right context, reduce hallucination.

## March 14 2026

[X] Refactor current architecture to scalable one.
[X] Introduce static client,Test full flow.
[X] Use the AppState across the modules.
[X] Implement response streaming to the client raised the request by tagging each client with unique id.

## March 15 2026

[X] Implement tokens streaming to client.
[X] Configure ollama to run in parallel to support parallel prompts (3).

## March 18 2026

[X] Containerize and publish to docker.
[X] Create landing page for product.
[X] Test full flow pull form docker and test.
[X] Setup github action to deploy in github pages and publish to docker.

## April 11 2026

[] Implement chunking (fixed + overlap).
[] Setup embedding pipeline.
[] Integerate vector DB (Quadrant).
[] Metadata strategy
[] Build document preparation guide according to the chunking model we use cause everything depends on chunking.

## April 12 2026

[] Implement dense retrieval.
[] Add BM25 retrieval.
[] Merge dense + sparse retrieval.
[] Add basic reranker.

## April 18 2026

[] Context builder (top-k, token limit).
[] Anti-hallucination system prompt.
[] Guardinals.
[] APIs (Ingestion & Reterival).
[] Source attribution (return doc references).

## Upcoming todos

[] Hard latency target (<1.5s).
[] Max cost per query tracking.
[] Bring in billing system integerating chargebee or stripe.
[] Add more info in landing page.
[] Decide what logs to store and what not from docker container/prometheus.
[] Run this engine image in personal laptop.
[] Calculate resource usage (RAM, ROM, CPU), Calculate infra cost for this MVP, Plan money saving plan to build home server.
[] Setup telegram notificaiton for weekly report on resource usage.
[] Provider strategy (OpenAI, etc.)
[] Replace RabbitMQ, add scheduler
[] Add evaluation dashboard
[] Graph RAG
[] Agentic RAG
[] vLLM
