use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathRule {
    pub pattern: String,
    pub action: String, // "block" | "warn"
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailConfig {
    pub workspace_id: String,
    pub version: String,
    #[serde(default)]
    pub deny_paths: Vec<String>,
    #[serde(default)]
    pub path_rules: Vec<PathRule>,
    #[serde(default)]
    pub protected_branches: Vec<String>,
    #[serde(default)]
    pub allowed_commands: Vec<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardDocument {
    pub title: String,
    pub category: String, // "architecture" | "controller" | "models" | "testing" | "grpc"
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardRequest {
    pub workspace_id: String,
    pub documents: Vec<OnboardDocument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardResponse {
    pub status: String,
    pub indexed_chunks: usize,
    pub workspace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
    pub category: Option<String>,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
}

fn default_top_k() -> usize {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub id: String,
    pub title: String,
    pub category: String,
    pub content: String,
    pub score: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    pub workspace_id: String,
    pub query: String,
    pub results: Vec<QueryResult>,
}
