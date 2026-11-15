pub mod echo;
pub mod error_demo;
pub mod health;
pub mod hello;
pub mod info;

use axum::Router;
use tower_http::trace::TraceLayer;
use crate::app::auth::routes as auth_routes;
use crate::app::AppState;

pub fn build(state: AppState) -> Router {
    let prefix = state.cfg.api_url_prefix();

    let api = Router::new()
        .merge(hello::routes())
        .merge(echo::routes())
        .merge(info::routes())
        .merge(error_demo::routes())
        .nest("/auth", auth_routes::routes());


    Router::new()
        .nest(&prefix, api)
        .merge(health::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
