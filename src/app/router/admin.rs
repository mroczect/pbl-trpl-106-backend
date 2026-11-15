use axum::Json;
use axum::Router;
use axum::routing::get;
use serde_json::json;

use crate::app::AppState;
use crate::app::auth::extractor::AdminUser;

pub fn routes() -> Router<AppState> {
    Router::new().route("/dashboard", get(admin_dashboard))
}

async fn admin_dashboard(AdminUser(claims): AdminUser) -> Json<serde_json::Value> {
    Json(json!({
        "message": "Welcome to the Admin Dashboard",
        "admin_id": claims.sub
    }))
}
