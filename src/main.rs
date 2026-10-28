mod db;
mod guardrails;
mod models;
mod rag;

use anyhow::Result;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use db::Database;
use serde_json::json;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "service": "zed-rag-service",
            "version": env!("CARGO_PKG_VERSION")
        })),
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zed_rag_service=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "data/rag.db".to_string());
    tracing::info!("Initializing SQLite database at: {}", db_path);
    let db = Database::new(&db_path)?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route(
            "/api/v1/workspaces/:workspace_id/guardrails",
            get(guardrails::get_guardrails).put(guardrails::update_guardrails),
        )
        .route(
            "/api/v1/workspaces/:workspace_id/onboard",
            post(rag::onboard_handler),
        )
        .route(
            "/api/v1/workspaces/:workspace_id/query",
            post(rag::query_handler),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Zed RAG & Guardrail Service listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c signal");
    tracing::info!("Shutting down zed-rag-service gracefully...");
}
