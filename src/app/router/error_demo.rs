use axum::Router;
use axum::extract::Path;
use axum::routing::get;

use crate::app::AppState;
use crate::app::error::AppError;

pub fn routes() -> Router<AppState> {
    Router::new().route("/error/{kind}", get(demo))
}

async fn demo(Path(kind): Path<String>) -> Result<&'static str, AppError> {
    match kind.as_str() {
        "validation" => Err(AppError::Validation("validation error".into())),
        "unauthorized" => Err(AppError::Unauthorized),
        "conflict" => Err(AppError::Conflict("resource conflict".into())),
        "not-found" => Err(AppError::NotFound),
        "internal" => Err(AppError::Internal(anyhow::anyhow!("unexpected error"))),
        other => Err(AppError::Validation(format!("unknown kind: {other}"))),
    }
}
