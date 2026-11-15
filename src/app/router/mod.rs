pub mod admin;
pub mod echo;
pub mod error_demo;
pub mod health;
pub mod hello;
pub mod info;

use crate::app::AppState;
use crate::app::auth::routes as auth_routes;
use crate::app::config::Mode;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, header};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

pub fn build(state: AppState) -> Router {
    let prefix = state.cfg.api_url_prefix();
    let is_prod = state.cfg.mode == Mode::Prod;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .merge(hello::routes())
        .merge(echo::routes())
        .merge(info::routes())
        .merge(error_demo::routes())
        .nest("/auth", auth_routes::routes())
        .nest("/admin", admin::routes());

    let mut app = Router::new()
        .nest(&prefix, api)
        .merge(health::routes())
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'"),
        ))
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    if is_prod {
        let governor_conf = GovernorConfigBuilder::default()
            .per_second(5)
            .burst_size(5)
            .finish()
            .expect("failed to create rate limiter config");

        app = app.layer(GovernorLayer::new(governor_conf));
    }

    app
}
