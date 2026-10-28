# Product Requirements Document (PRD): Zed Cloud RAG & Guardrail Service

**Repository:** `dimsmaul/zed-rag-service`  
**Version:** 1.0.0-draft  
**Language:** Rust (Axum, Tokio, FastEmbed / SQLite)  
**Deployment:** Containerized (Docker, Docker Compose, GHCR)

---

## 1. Executive Summary

`zed-rag-service` is a high-performance, ultra-low-memory backend microservice written in Rust. It serves as the intelligent cloud backbone for the **Zed Harness Agent**, providing:

1. **Centralized Team Guardrail Registry**: Defines what AI agents can and cannot modify across repositories (e.g. blocking edits to `common-grpc` and enforcing proto release workflows).
2. **Codebase Onboard Ingestion**: Ingests canonical patterns extracted during the developer's `onboard` command (controller conventions, model schemas, error handling) to minimize LLM hallucination.
3. **Hybrid Code Retrieval (RAG)**: Combines dense vector similarity with lexical (BM25) matching for fast, accurate context retrieval with minimal memory footprint (<80MB baseline).
4. **Independent Release Lifecycle**: Decoupled from the Zed editor desktop client, enabling continuous delivery and multi-arch Docker image releases.

---

## 2. System Architecture

```
┌────────────────────────────────────────────────────────┐
│                   ZED HARNESS AGENT                    │
│            (Desktop Client - macOS / Linux)            │
└───────────────────────────┬────────────────────────────┘
                            │ HTTPS / REST (or gRPC)
                            ▼
┌────────────────────────────────────────────────────────┐
│                   ZED-RAG-SERVICE                      │
│                                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │         Axum HTTP Server (Port 8080)             │  │
│  │   Auth Middleware, Tracing & Prometheus Metrics   │  │
│  └───────────┬──────────────────────────┬───────────┘  │
│              │                          │              │
│  ┌───────────▼──────────┐    ┌──────────▼───────────┐  │
│  │   Guardrail Engine   │    │  RAG Search & Index  │  │
│  │   (Policy Registry)  │    │  (Hybrid BM25+Dense) │  │
│  └───────────┬──────────┘    └──────────┬───────────┘  │
│              │                          │              │
│  ┌───────────▼──────────────────────────▼───────────┐  │
│  │       In-Memory & SQLite Persistence             │  │
│  │    (Zero-overhead FastEmbed / Vector Store)      │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

---

## 3. Core Functional Requirements

### 3.1. Team Guardrail Registry
* **Workspace Isolation**: Guardrails are scoped by `workspace_id` (e.g. `github.com/myorg/backend`).
* **Rule Definitions**:
  - `deny_paths`: Glob patterns forbidden from direct writes (e.g. `common-grpc/**`, `proto/gen/**`).
  - `path_rules`: Actionable explanation messages injected back to LLMs upon rule violation.
  - `protected_branches`: Git branch protection checks.
  - `allowed_commands`: Whitelist of shell tasks.
* **APIs**:
  - `GET /api/v1/workspaces/{id}/guardrails`: Fetched by Zed client on workspace open.
  - `PUT /api/v1/workspaces/{id}/guardrails`: Managed by team leads / CI.

### 3.2. Codebase Onboard Ingestion
* **Endpoint**: `POST /api/v1/workspaces/{id}/onboard`
* **Payload**: Structured documentation synthesized by the client agent:
  - Document Title (e.g. *"Controller Development Guide & Samples"*)
  - Category (`architecture`, `controller`, `models`, `testing`, `grpc`)
  - Content & Code snippets
* **Processing**:
  - Text chunking & normalization.
  - FastEmbed generates dense embeddings (in-process ONNX without Python runtime).
  - Stored with metadata and full-text index.

### 3.3. RAG Query & Search
* **Endpoint**: `POST /api/v1/workspaces/{id}/query`
* **Payload**: `query` string (e.g. *"how to write a user controller with validation"*) + optional `category` filter + `top_k`.
* **Search Strategy**:
  - Hybrid Score = $\alpha \cdot \text{VectorCosineSim} + (1 - \alpha) \cdot \text{BM25Score}$.
  - Returns top matching reference chunks with relevance scores.

### 3.4. Health & Observability
* `GET /health`: Liveness and readiness probe for Docker / Kubernetes.
* `GET /metrics`: Standard Prometheus metrics (request latency, memory usage, index count).

---

## 4. Non-Functional Requirements

* **Low Memory Footprint**: Max RSS < 150MB under active load (written purely in Rust).
* **Latency**: Query p95 < 25ms.
* **Deployment**: Standalone Docker image (< 100MB compressed), no external heavy dependencies required by default.
