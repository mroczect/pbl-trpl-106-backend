use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::app::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/hello", get(hello))
}

async fn hello() -> Json<serde_json::Value> {
    Json(json!({
        "message": "hello from rust backend",
        "service": "pbl-trpl-106-backend",
        "version": "0.1.0"
    }))
}
