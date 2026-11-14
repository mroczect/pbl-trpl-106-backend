use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use db::prelude::Connection;
use serde_json::json;

use crate::app::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/live", get(live))
        .route("/ready", get(ready))
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn live() -> &'static str {
    "ok"
}

async fn ready(State(state): State<AppState>) -> StatusCode {
    match state.db.ping().await {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            tracing::warn!(err = %e, "readiness check failed");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
