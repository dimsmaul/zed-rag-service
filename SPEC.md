# Technical Specification (SPEC): Zed Cloud RAG & Guardrail Service

**Repository:** `dimsmaul/zed-rag-service`  
**Protocol:** REST / JSON over HTTP (Port 8080)  
**Authentication:** Optional Bearer Token / API Key header (`Authorization: Bearer <token>`)

---

## 1. Data Models & JSON Schemas

### 1.1. Guardrail Models

#### `GuardrailConfig`
```json
{
  "workspace_id": "github.com/dimsmaul/backend-services",
  "version": "1.0",
  "deny_paths": [
    "common-grpc/**",
    "proto/generated/**",
    ".github/workflows/**"
  ],
  "path_rules": [
    {
      "pattern": "common-grpc/**",
      "action": "block",
      "reason": "Direct edits to common-grpc are strictly forbidden. You must edit service proto files in services/ and trigger the proto release script."
    }
  ],
  "protected_branches": ["main", "master"],
  "allowed_commands": ["cargo test", "cargo check", "go test ./..."],
  "updated_at": "2026-10-28T13:40:00Z"
}
```

### 1.2. Onboard Knowledge Ingestion Models

#### `OnboardRequest`
```json
{
  "workspace_id": "github.com/dimsmaul/backend-services",
  "documents": [
    {
      "title": "Controller Architecture & Validation Pattern",
      "category": "controller",
      "content": "All controllers must inherit from BaseController and validate payloads using Validator middleware...\n```go\nfunc CreateUser(c *gin.Context) {\n  var req CreateUserRequest\n  if err := c.ShouldBindJSON(&req); err != nil { ... }\n}\n```",
      "tags": ["controller", "validation", "gin"]
    }
  ]
}
```

#### `OnboardResponse`
```json
{
  "status": "success",
  "indexed_chunks": 4,
  "workspace_id": "github.com/dimsmaul/backend-services"
}
```

### 1.3. Query / Retrieval Models

#### `QueryRequest`
```json
{
  "query": "how to implement error response in controller",
  "category": "controller",
  "top_k": 3
}
```

#### `QueryResponse`
```json
{
  "results": [
    {
      "title": "Controller Architecture & Validation Pattern",
      "category": "controller",
      "content": "Standardized JSON error formatting should use api.ErrorResponse(code, message)...",
      "score": 0.892,
      "metadata": {
        "tags": ["controller", "errors"]
      }
    }
  ]
}
```

---

## 2. API Endpoints

| Method | Path | Description |
| :--- | :--- | :--- |
| `GET` | `/health` | Health check endpoint |
| `GET` | `/api/v1/workspaces/{workspace_id}/guardrails` | Fetch workspace guardrail policies |
| `PUT` | `/api/v1/workspaces/{workspace_id}/guardrails` | Create or update workspace guardrail policies |
| `POST` | `/api/v1/workspaces/{workspace_id}/onboard` | Ingest onboard documents & compute embeddings |
| `POST` | `/api/v1/workspaces/{workspace_id}/query` | Query RAG knowledge store for reference patterns |
| `DELETE` | `/api/v1/workspaces/{workspace_id}` | Purge all vectors and policies for a workspace |

---

## 3. Storage & Embedded Engine Design

1. **SQLite Database (`data/rag.db`)**:
   - `workspaces`: Table for registered workspaces.
   - `guardrails`: Table storing JSON-serialized `GuardrailConfig`.
   - `documents`: Table storing ingested document chunks and raw text.
   - `embeddings`: Table or in-memory vector index (Float32 vector arrays).
2. **FastEmbed Rust**:
   - Pre-trained model: `bge-small-en-v1.5` or `all-MiniLM-L6-v2` (dimension: 384).
   - CPU-friendly ONNX inference running in pure native code without Python.
