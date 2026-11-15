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

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: String,
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

fn is_valid_email(email: &str) -> bool {
    let email = email.trim();
    if email.is_empty() || email.len() > 254 {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let (local, domain) = (parts[0], parts[1]);

    if local.is_empty() || local.len() > 64 || domain.is_empty() || domain.len() > 255 {
        return false;
    }

    if !domain.contains('.')
        || domain.starts_with('.')
        || domain.ends_with('.')
        || domain.contains("..")
    {
        return false;
    }

    let valid_local = local
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "!#$%&'*+-/=?^_`{|}~.".contains(c));
    let valid_domain = domain
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-');

    valid_local && valid_domain
}

impl RegisterRequest {
    pub fn validate(&self) -> Result<(), String> {
        if !is_valid_email(&self.email) {
            return Err("invalid email format".into());
        }

        let pass_len = self.password.chars().count();
        if pass_len < 8 {
            return Err("password must be at least 8 characters".into());
        }
        if pass_len > 72 {
            return Err("password must be at most 72 characters".into());
        }

        let name_len = self.name.trim().chars().count();
        if name_len == 0 {
            return Err("name is required".into());
        }
        if name_len > 255 {
            return Err("name must be at most 255 characters".into());
        }
        Ok(())
    }
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.trim().is_empty() {
            return Err("email is required".into());
        }
        if self.password.is_empty() {
            return Err("password is required".into());
        }
        Ok(())
    }
}

impl ChangePasswordRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.old_password.is_empty() {
            return Err("old password is required".into());
        }

        let pass_len = self.new_password.chars().count();
        if pass_len < 8 {
            return Err("new password must be at least 8 characters".into());
        }
        if pass_len > 72 {
            return Err("new password must be at most 72 characters".into());
        }
        if self.old_password == self.new_password {
            return Err("new password must be different from old password".into());
        }
        Ok(())
    }
}

impl UpdateProfileRequest {
    pub fn validate(&self) -> Result<(), String> {
        let name_len = self.name.trim().chars().count();
        if name_len == 0 {
            return Err("name is required".into());
        }
        if name_len > 255 {
            return Err("name must be at most 255 characters".into());
        }
        Ok(())
    }
}
