use axum::{Json, extract::State};

use crate::app::{
    auth::{
        dto::{
            AuthResponse, ChangePasswordRequest, LoginRequest, RegisterRequest,
            UpdateProfileRequest, UserResponse,
        },
        extractor::AuthUser,
        service,
    },
    error::AppError,
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate().map_err(AppError::Validation)?;
    let res = service::register(&state, payload).await?;
    Ok(Json(res))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate().map_err(AppError::Validation)?;
    let res = service::login(&state, payload).await?;
    Ok(Json(res))
}

pub async fn me(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    use db::prelude::{Executor, Query};

    let row = state
        .db
        .fetch_optional(
            Query::new("SELECT id, email, name, role FROM users WHERE id = ?").bind(claims.sub),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db me query: {e}")))?
        .ok_or(AppError::Unauthorized)?;

    let id = row
        .as_ref()
        .get("id")
        .map_err(|e| AppError::Internal(anyhow::anyhow!("baca id: {e}")))?
        .as_i64()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("id bukan i64")))?;

    let email = row
        .as_ref()
        .get("email")
        .map_err(|e| AppError::Internal(anyhow::anyhow!("baca email: {e}")))?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("email bukan string")))?;

    let name = row
        .as_ref()
        .get("name")
        .map_err(|e| AppError::Internal(anyhow::anyhow!("baca name: {e}")))?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("name bukan string")))?;

    let role = row
        .as_ref()
        .get("role")
        .map_err(|e| AppError::Internal(anyhow::anyhow!("baca role: {e}")))?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("role bukan string")))?;

    Ok(Json(UserResponse {
        id,
        email,
        name,
        role,
    }))
}

pub async fn change_password(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    payload.validate().map_err(AppError::Validation)?;
    service::change_password(&state, claims.sub, payload).await?;
    Ok(Json(
        serde_json::json!({ "message": "password changed successfully" }),
    ))
}

pub async fn update_profile(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate().map_err(AppError::Validation)?;
    let res = service::update_profile(&state, claims.sub, payload).await?;
    Ok(Json(res))
}
