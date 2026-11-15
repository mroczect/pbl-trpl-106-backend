use axum::{
    body::to_bytes,
    http::{StatusCode, header},
    response::IntoResponse,
};
use pbl_trpl_106_backend::app::error::AppError;
use serde_json::Value;

async fn assert_error(
    err: AppError,
    expected_status: StatusCode,
    expected_code: &str,
    expected_msg: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = err.into_response();

    assert_eq!(response.status(), expected_status);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert_eq!(content_type, "application/json");

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let obj = json.as_object().expect("Body must be a JSON Object");
    assert_eq!(obj.len(), 1, "Response must only have an 'error' field");

    let err_obj = json["error"]
        .as_object()
        .expect("Field 'error' must be an object");
    assert_eq!(err_obj.len(), 2, "Only 'code' and 'message' are allowed");

    assert_eq!(json["error"]["code"], expected_code);
    assert_eq!(json["error"]["message"], expected_msg);

    Ok(())
}

#[tokio::test]
async fn test_validation_error() -> Result<(), Box<dyn std::error::Error>> {
    assert_error(
        AppError::Validation("invalid email".to_string()),
        StatusCode::BAD_REQUEST,
        "VALIDATION",
        "invalid email",
    )
    .await
}

#[tokio::test]
async fn test_unauthorized_error() -> Result<(), Box<dyn std::error::Error>> {
    assert_error(
        AppError::Unauthorized,
        StatusCode::UNAUTHORIZED,
        "UNAUTHORIZED",
        "unauthorized",
    )
    .await
}

#[tokio::test]
async fn test_forbidden_error() -> Result<(), Box<dyn std::error::Error>> {
    assert_error(
        AppError::Forbidden,
        StatusCode::FORBIDDEN,
        "FORBIDDEN",
        "forbidden",
    )
    .await
}

#[tokio::test]
async fn test_conflict_error() -> Result<(), Box<dyn std::error::Error>> {
    assert_error(
        AppError::Conflict("email is already registered".to_string()),
        StatusCode::CONFLICT,
        "CONFLICT",
        "email is already registered",
    )
    .await
}

#[tokio::test]
async fn test_not_found_error() -> Result<(), Box<dyn std::error::Error>> {
    assert_error(
        AppError::NotFound,
        StatusCode::NOT_FOUND,
        "NOT_FOUND",
        "not found",
    )
    .await
}

#[tokio::test]
async fn test_internal_error_masked() -> Result<(), Box<dyn std::error::Error>> {
    let err = AppError::Internal(anyhow::anyhow!(
        "Password DB: root, Query failed: syntax error"
    ));
    assert_error(
        err,
        StatusCode::INTERNAL_SERVER_ERROR,
        "INTERNAL",
        "internal error",
    )
    .await
}

#[test]
fn test_error_display_trait() {
    assert_eq!(AppError::Validation("test".into()).to_string(), "test");
    assert_eq!(AppError::Unauthorized.to_string(), "unauthorized");
    assert_eq!(AppError::Forbidden.to_string(), "forbidden");
    assert_eq!(AppError::Conflict("test".into()).to_string(), "test");
    assert_eq!(AppError::NotFound.to_string(), "not found");

    let internal = AppError::Internal(anyhow::anyhow!("db error"));
    assert_eq!(internal.to_string(), "internal error");
}

#[test]
fn test_from_anyhow() {
    let anyhow_err = anyhow::anyhow!("some random error");
    let app_err: AppError = anyhow_err.into();
    assert!(matches!(app_err, AppError::Internal(_)));
}

#[test]
fn test_debug_trait() {
    let err = AppError::Forbidden;
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("Forbidden"));
}
