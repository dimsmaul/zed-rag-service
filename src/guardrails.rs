use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::{db::Database, models::GuardrailConfig};

pub async fn get_guardrails(
    Path(workspace_id): Path<String>,
    State(db): State<Database>,
) -> impl IntoResponse {
    match db.get_guardrails(&workspace_id) {
        Ok(Some(config)) => (StatusCode::OK, Json(config)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Guardrails not found for this workspace",
                "workspace_id": workspace_id
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn update_guardrails(
    Path(workspace_id): Path<String>,
    State(db): State<Database>,
    Json(mut config): Json<GuardrailConfig>,
) -> impl IntoResponse {
    config.workspace_id = workspace_id;
    if config.updated_at.is_empty() {
        config.updated_at = chrono::Utc::now().to_rfc3339();
    }

    match db.save_guardrails(&config) {
        Ok(_) => (StatusCode::OK, Json(config)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
