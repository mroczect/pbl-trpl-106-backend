use db::prelude::{Executor, Query, Row};

use crate::app::{
    auth::{
        dto::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
        jwt, password,
    },
    error::AppError,
    state::AppState,
};

pub async fn register(state: &AppState, req: RegisterRequest) -> Result<AuthResponse, AppError> {
    let email = req.email.trim().to_lowercase();
    let name = req.name.trim().to_string();

    let existing = state
        .db
        .fetch_optional(Query::new("SELECT id FROM users WHERE email = ?").bind(email.as_str()))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db check email: {e}")))?;

    if existing.is_some() {
        return Err(AppError::Conflict("email sudah terdaftar".into()));
    }

    let password_hash = password::hash_password(&req.password)?;

    let result = state
        .db
        .execute(
            Query::new(
                "INSERT INTO users \
                 (email, password_hash, name, created_at, updated_at) \
                 VALUES (?, ?, ?, NOW(), NOW())",
            )
            .bind(email.as_str())
            .bind(password_hash.as_str())
            .bind(name.as_str()),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db insert user: {e}")))?;

    let user_id_u64 = result
        .last_insert_id
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("insert tidak return id")))?;
    let user_id = i64::try_from(user_id_u64)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("user id overflow i64")))?;

    let token = jwt::generate_jwt(user_id, "customer", &state.cfg.jwt_secret)?;

    Ok(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            email,
            name,
            role: "customer".into(),
        },
    })
}

pub async fn login(state: &AppState, req: LoginRequest) -> Result<AuthResponse, AppError> {
    let email = req.email.trim().to_lowercase();

    let row = state
        .db
        .fetch_optional(
            Query::new(
                "SELECT id, email, password_hash, name, role, status \
                 FROM users WHERE email = ?",
            )
            .bind(email.as_str()),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db login query: {e}")))?
        .ok_or(AppError::Unauthorized)?;

    let id = col_i64(&row, "id")?;
    let email = col_string(&row, "email")?;
    let hash = col_string(&row, "password_hash")?;
    let name = col_string(&row, "name")?;
    let role = col_string(&row, "role")?;
    let status = col_string(&row, "status")?;

    if status != "active" {
        return Err(AppError::Unauthorized);
    }

    if !password::verify_password(&req.password, &hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = jwt::generate_jwt(id, &role, &state.cfg.jwt_secret)?;

    Ok(AuthResponse {
        token,
        user: UserResponse {
            id,
            email,
            name,
            role,
        },
    })
}

fn col_i64(row: &Box<dyn Row>, col: &str) -> Result<i64, AppError> {
    let v = row
        .get(col)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("baca kolom {col}: {e}")))?;
    if let Some(i) = v.as_i64() {
        return Ok(i);
    }
    if let Some(u) = v.as_u64() {
        return i64::try_from(u)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("kolom {col} overflow i64")));
    }
    Err(AppError::Internal(anyhow::anyhow!(
        "kolom {col} bukan integer"
    )))
}

fn col_string(row: &Box<dyn Row>, col: &str) -> Result<String, AppError> {
    row.get(col)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("baca kolom {col}: {e}")))?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("kolom {col} bukan string")))
}
