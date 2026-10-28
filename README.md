# Zed RAG & Guardrail Service

A high-performance, ultra-low-memory cloud microservice written in Rust for the **Zed Harness Agent**.

## Features

- **Team Guardrail Registry**: Store and serve workspace-level rules (e.g. blocking direct edits to `common-grpc` and requiring proto releases).
- **Onboard Ingestion**: Ingest and index canonical codebase patterns (controller conventions, data models, error handling) to minimize LLM hallucination.
- **Hybrid RAG Retrieval**: Fast vector and lexical search to supply relevant context to the Zed AI Harness.
- **Ultra-low Memory**: Built with Rust, Axum, and FastEmbed/SQLite (<100MB RAM footprint).
- **Containerized**: Independent Docker packaging and GitHub Actions CI/CD to GHCR.

## Documentation

- [PRD.md](PRD.md): Product Requirements Document
- [SPEC.md](SPEC.md): API & Data Model Specification
- [Plan.md](Plan.md): Implementation Roadmap

## Quickstart

### Running with Docker

```bash
docker compose up -d
```

### Running Locally

```bash
cargo run --release
```

The service will start listening on `http://localhost:8080`.

## API Endpoints

- `GET /health` - Health check
- `GET /api/v1/workspaces/:id/guardrails` - Fetch workspace guardrail rules
- `PUT /api/v1/workspaces/:id/guardrails` - Update workspace guardrail rules
- `POST /api/v1/workspaces/:id/onboard` - Ingest codebase patterns & generate embeddings
- `POST /api/v1/workspaces/:id/query` - Hybrid search for codebase reference patterns
