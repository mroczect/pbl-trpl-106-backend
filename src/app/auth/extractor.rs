use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};

use crate::app::{
    auth::jwt::{self, Claims},
    state::AppState,
};

#[derive(Debug)]
pub struct AuthUser(pub Claims);
#[derive(Debug)]
pub struct AdminUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let unauthorized = || {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": { "code": "UNAUTHORIZED", "message": "unauthorized" }
                })),
            )
        };

        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(unauthorized)?;

        let token = header.strip_prefix("Bearer ").ok_or_else(unauthorized)?;

        let claims = jwt::verify_jwt(token, &state.cfg.jwt_secret).map_err(|_| unauthorized())?;

        Ok(AuthUser(claims))
    }
}

impl FromRequestParts<AppState> for AdminUser {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;

        if auth_user.0.role != "admin" {
            return Err((
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "error": { "code": "FORBIDDEN", "message": "forbidden" }
                })),
            ));
        }

        Ok(AdminUser(auth_user.0))
    }
}
