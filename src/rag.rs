use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;
use crate::{
    db::Database,
    models::{OnboardRequest, OnboardResponse, QueryRequest, QueryResponse, QueryResult},
};

pub async fn onboard_handler(
    Path(workspace_id): Path<String>,
    State(db): State<Database>,
    Json(req): Json<OnboardRequest>,
) -> impl IntoResponse {
    let mut indexed_count = 0;

    for doc in req.documents {
        let doc_id = Uuid::new_v4().to_string();
        if let Err(e) = db.insert_document(
            &doc_id,
            &workspace_id,
            &doc.title,
            &doc.category,
            &doc.content,
            &doc.tags,
        ) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to index document: {}", e) })),
            )
                .into_response();
        }
        indexed_count += 1;
    }

    let response = OnboardResponse {
        status: "success".to_string(),
        indexed_chunks: indexed_count,
        workspace_id,
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

pub async fn query_handler(
    Path(workspace_id): Path<String>,
    State(db): State<Database>,
    Json(req): Json<QueryRequest>,
) -> impl IntoResponse {
    match db.search_documents(
        &workspace_id,
        &req.query,
        req.category.as_deref(),
        req.top_k,
    ) {
        Ok(matches) => {
            let results: Vec<QueryResult> = matches
                .into_iter()
                .map(|(id, title, category, content, tags, score)| QueryResult {
                    id,
                    title,
                    category,
                    content,
                    score,
                    tags,
                })
                .collect();

            let response = QueryResponse {
                workspace_id,
                query: req.query,
                results,
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Query failed: {}", e) })),
        )
            .into_response(),
    }
}
