use axum::{Router, routing::{get, post}};

use crate::app::{auth::handler, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handler::register))
        .route("/login", post(handler::login))
        .route("/me", get(handler::me))   
}
