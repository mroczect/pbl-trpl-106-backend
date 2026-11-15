use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

impl RegisterRequest {
    pub fn validate(&self) -> Result<(), String> {
        let email = self.email.trim();
        if email.is_empty() || email.len() > 254 || !email.contains('@') || !email.contains('.') {
            return Err("email tidak valid".into());
        }
        if self.password.len() < 8 {
            return Err("password minimal 8 karakter".into());
        }
        if self.password.len() > 72 {
            return Err("password maksimal 72 karakter".into());
        }
        if self.name.trim().is_empty() {
            return Err("nama wajib diisi".into());
        }
        if self.name.len() > 255 {
            return Err("nama maksimal 255 karakter".into());
        }
        Ok(())
    }
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.trim().is_empty() {
            return Err("email wajib diisi".into());
        }
        if self.password.is_empty() {
            return Err("password wajib diisi".into());
        }
        Ok(())
    }
}
