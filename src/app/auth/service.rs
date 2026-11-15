use db::prelude::{Executor, Query, Row};

use crate::app::{
    auth::{
        dto::{
            AuthResponse, ChangePasswordRequest, LoginRequest, RegisterRequest,
            UpdateProfileRequest, UserResponse,
        },
        jwt, password,
    },
    error::AppError,
    state::AppState,
};

const DUMMY_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$w2mZ1bI4y19zV52wV4h1O3j78uXjP9a6oM3b5oP7w9U";

pub async fn register(state: &AppState, req: RegisterRequest) -> Result<AuthResponse, AppError> {
    let email = req.email.trim().to_lowercase();
    let name = req.name.trim().to_string();

    let existing = state
        .db
        .fetch_optional(Query::new("SELECT id FROM users WHERE email = ?").bind(email.as_str()))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db check email failed: {e}")))?;

    if existing.is_some() {
        return Err(AppError::Conflict("email is already registered".into()));
    }

    let password_hash = password::hash_password(&req.password)?;

    let result = match state
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
    {
        Ok(res) => res,
        Err(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("Duplicate entry") || err_msg.contains("1062") {
                return Err(AppError::Conflict("email is already registered".into()));
            }
            return Err(AppError::Internal(anyhow::anyhow!(
                "db insert user failed: {e}"
            )));
        }
    };

    let user_id_u64 = result
        .last_insert_id
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("insert did not return an id")))?;
    let user_id = i64::try_from(user_id_u64)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("user id overflowed i64")))?;

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

    let row_opt = state
        .db
        .fetch_optional(
            Query::new(
                "SELECT id, email, password_hash, name, role, status \
                 FROM users WHERE email = ?",
            )
            .bind(email.as_str()),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db login query failed: {e}")))?;

    let (hash_to_verify, user_data) = match &row_opt {
        Some(r) => (col_string(r.as_ref(), "password_hash")?, Some(r)),
        None => (DUMMY_HASH.to_string(), None),
    };

    let is_valid = password::verify_password(&req.password, &hash_to_verify)?;

    let row = match user_data {
        Some(r) if is_valid => r,
        _ => return Err(AppError::Unauthorized),
    };

    let id = col_i64(row.as_ref(), "id")?;
    let email = col_string(row.as_ref(), "email")?;
    let name = col_string(row.as_ref(), "name")?;
    let role = col_string(row.as_ref(), "role")?;
    let status = col_string(row.as_ref(), "status")?;

    if status != "active" {
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

pub async fn change_password(
    state: &AppState,
    user_id: i64,
    req: ChangePasswordRequest,
) -> Result<(), AppError> {
    let row = state
        .db
        .fetch_optional(Query::new("SELECT password_hash FROM users WHERE id = ?").bind(user_id))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db get hash failed: {e}")))?
        .ok_or(AppError::Unauthorized)?;

    let current_hash = col_string(row.as_ref(), "password_hash")?;

    if !password::verify_password(&req.old_password, &current_hash)? {
        return Err(AppError::Validation("incorrect old password".into()));
    }

    let new_hash = password::hash_password(&req.new_password)?;
    let _ = state
        .db
        .execute(
            Query::new("UPDATE users SET password_hash = ?, updated_at = NOW() WHERE id = ?")
                .bind(new_hash.as_str())
                .bind(user_id),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db update password failed: {e}")))?;

    Ok(())
}

pub async fn update_profile(
    state: &AppState,
    user_id: i64,
    req: UpdateProfileRequest,
) -> Result<UserResponse, AppError> {
    let name = req.name.trim();

    let _ = state
        .db
        .execute(
            Query::new("UPDATE users SET name = ?, updated_at = NOW() WHERE id = ?")
                .bind(name)
                .bind(user_id),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db update profile failed: {e}")))?;

    let row = state
        .db
        .fetch_optional(
            Query::new("SELECT id, email, name, role FROM users WHERE id = ?").bind(user_id),
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("db get profile failed: {e}")))?
        .ok_or(AppError::Unauthorized)?;

    Ok(UserResponse {
        id: col_i64(row.as_ref(), "id")?,
        email: col_string(row.as_ref(), "email")?,
        name: col_string(row.as_ref(), "name")?,
        role: col_string(row.as_ref(), "role")?,
    })
}

fn col_i64(row: &dyn Row, col: &str) -> Result<i64, AppError> {
    let v = row
        .get(col)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("failed to read column {col}: {e}")))?;
    if let Some(i) = v.as_i64() {
        return Ok(i);
    }
    if let Some(u) = v.as_u64() {
        return i64::try_from(u)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("column {col} overflowed i64")));
    }
    Err(AppError::Internal(anyhow::anyhow!(
        "column {col} is not an integer"
    )))
}

fn col_string(row: &dyn Row, col: &str) -> Result<String, AppError> {
    row.get(col)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("failed to read column {col}: {e}")))?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("column {col} is not a string")))
}
