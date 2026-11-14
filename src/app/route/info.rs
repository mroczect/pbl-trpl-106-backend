use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::app::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/info", get(info))
}

async fn info(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "service": "pbl-trpl-106-backend",
        "mode": state.cfg.mode.as_str(),
        "version": state.cfg.version,
        "api_prefix": state.cfg.api_url_prefix(),
        "listen": state.cfg.listen.to_string(),
    }))
}
