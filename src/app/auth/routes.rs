use axum::{
    Router,
    routing::{get, post, put},
};

use crate::app::{auth::handler, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handler::register))
        .route("/login", post(handler::login))
        .route("/me", get(handler::me))
        .route("/password", put(handler::change_password))
        .route("/profile", put(handler::update_profile))
}
