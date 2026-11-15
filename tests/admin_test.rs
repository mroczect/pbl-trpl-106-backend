use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode, header},
};
use pbl_trpl_106_backend::app::{auth::jwt, config::Config, router, state::AppState};
use std::net::SocketAddr;
use tower::ServiceExt;

async fn setup() -> axum::Router {
    drop(dotenvy::dotenv());
    let cfg = Config::from_env().expect("Config failed to load");
    let db = db::Database::connect(&cfg.database_url)
        .await
        .expect("DB connection failed");
    let state = AppState::new(cfg, db);
    router::build(state)
}

fn inject_ip(mut req: Request<Body>) -> Request<Body> {
    let _ = req
        .extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 12345))));
    req
}

fn get_request_with_token(uri: &str, token: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder().uri(uri).method("GET");

    if let Some(t) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {t}"));
    }

    let req = builder.body(Body::empty()).unwrap();
    inject_ip(req)
}

#[tokio::test]
async fn test_admin_rbac() {
    let app = setup().await;
    let cfg = Config::from_env().unwrap();
    let path = format!("/api/{}/admin/dashboard", cfg.api_prefix);

    let res = app
        .clone()
        .oneshot(get_request_with_token(&path, None))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let customer_token = jwt::generate_jwt(1, "customer", &cfg.jwt_secret).unwrap();
    let res = app
        .clone()
        .oneshot(get_request_with_token(&path, Some(&customer_token)))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    let agent_token = jwt::generate_jwt(2, "agent", &cfg.jwt_secret).unwrap();
    let res = app
        .clone()
        .oneshot(get_request_with_token(&path, Some(&agent_token)))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    let admin_token = jwt::generate_jwt(3, "admin", &cfg.jwt_secret).unwrap();
    let res = app
        .clone()
        .oneshot(get_request_with_token(&path, Some(&admin_token)))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["message"], "Welcome to the Admin Dashboard");
    assert_eq!(json["admin_id"], 3);
}
