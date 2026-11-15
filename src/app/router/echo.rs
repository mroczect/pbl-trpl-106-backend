use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::app::error::AppError;

pub fn routes() -> Router<AppState> {
    Router::new().route("/echo", post(echo))
}

#[derive(Deserialize)]
struct EchoRequest {
    message: String,
}

#[derive(Serialize)]
struct EchoResponse {
    message: String,
    length: usize,
}

async fn echo(Json(req): Json<EchoRequest>) -> Result<Json<EchoResponse>, AppError> {
    if req.message.trim().is_empty() {
        return Err(AppError::Validation("message must not be empty".into()));
    }

    Ok(Json(EchoResponse {
        length: req.message.len(),
        message: req.message,
    }))
}
