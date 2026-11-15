use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::app::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn generate_jwt(user_id: i64, role: &str, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let expires_at = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        role: role.to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
    };

    let encoding_key = EncodingKey::from_secret(secret.as_bytes())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid JWT secret: {e}")))?;

    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to generate JWT: {e}")))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let validation = Validation::new(Algorithm::HS256);

    let decoding_key =
        DecodingKey::from_secret(secret.as_bytes()).map_err(|_| AppError::Unauthorized)?;

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
}
