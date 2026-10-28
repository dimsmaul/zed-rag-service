# Implementation Plan: Zed Cloud RAG & Guardrail Service

**Repository:** `dimsmaul/zed-rag-service`  
**Version:** 1.0.0  
**References:** [`PRD.md`](file:///Users/dimasmaulana/Documents/Research/zed-rag-service/PRD.md), [`SPEC.md`](file:///Users/dimasmaulana/Documents/Research/zed-rag-service/SPEC.md)

---

## Phases & Milestones

### Phase 1: Project Scaffolding & Core Server
- [x] Create GitHub repository (`dimsmaul/zed-rag-service`).
- [ ] Initialize Rust Cargo project with Axum, Tokio, Serde, Tracing.
- [ ] Implement Health (`/health`) and basic routing setup.
- [ ] Set up Dockerfile and `docker-compose.yml`.

### Phase 2: Guardrail Registry Subsystem
- [ ] Implement `GuardrailConfig` data types with Serde validation.
- [ ] SQLite storage layer for persisting workspace policies.
- [ ] Endpoints:
  - `GET /api/v1/workspaces/{workspace_id}/guardrails`
  - `PUT /api/v1/workspaces/{workspace_id}/guardrails`
- [ ] Unit tests for policy serialization and retrieval.

### Phase 3: FastEmbed & Vector Index Subsystem
- [ ] Integrate `fastembed` for in-process dense vector generation.
- [ ] Implement hybrid index (In-memory Cosine similarity + keyword matching).
- [ ] Persistence of vector embeddings in SQLite.
- [ ] Unit tests for cosine distance calculation.

### Phase 4: Onboard & Query Ingestion Pipelines
- [ ] Endpoint `POST /api/v1/workspaces/{workspace_id}/onboard`:
  - Receives documents, chunks text, generates embeddings, persists to DB.
- [ ] Endpoint `POST /api/v1/workspaces/{workspace_id}/query`:
  - Embeds incoming query, performs nearest-neighbor search, formats top-k results.
- [ ] Integration tests covering end-to-end Onboard -> Query cycle.

### Phase 5: Production Containerization & GitHub Actions CI/CD
- [ ] Multi-stage `Dockerfile` optimizing binary size (strip symbols, Alpine/Debian-slim base).
- [ ] `.github/workflows/docker-publish.yml`:
  - Automatic build and push to GHCR on Git tag push (`v*.*.*`).
