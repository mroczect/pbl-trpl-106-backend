use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use pbl_trpl_106_backend::app::{config::Config, router, state::AppState};
use std::net::SocketAddr;
use tower::ServiceExt;

async fn setup() -> axum::Router {
    drop(dotenvy::dotenv());
    let cfg = Config::from_env().expect("Config failed to load");
    let db = db::Database::connect(&cfg.database_url)
        .await
        .expect("DB connection failed. Pastikan Podman/MySQL menyala!");
    let state = AppState::new(cfg, db);
    router::build(state)
}

fn inject_ip(mut req: Request<Body>) -> Request<Body> {
    let _ = req
        .extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 12345))));
    req
}

fn get_request(uri: &str) -> Request<Body> {
    let req = Request::builder()
        .uri(uri)
        .method("GET")
        .body(Body::empty())
        .unwrap();
    inject_ip(req)
}

fn post_request(uri: &str, body: &str) -> Request<Body> {
    let req = Request::builder()
        .uri(uri)
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    inject_ip(req)
}

#[tokio::test]
async fn test_basic_routing() {
    let app = setup().await;

    let res = app.clone().oneshot(get_request("/health")).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = app.clone().oneshot(get_request("/live")).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let cfg = Config::from_env().unwrap();
    let info_path = format!("/api/{}/hello", cfg.api_prefix);
    let res = app.clone().oneshot(get_request(&info_path)).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = app.oneshot(get_request("/route-tidak-ada")).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_security_headers() {
    let app = setup().await;

    let res = app.oneshot(get_request("/health")).await.unwrap();

    assert_eq!(
        res.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(),
        "nosniff"
    );
    assert_eq!(res.headers().get(header::X_FRAME_OPTIONS).unwrap(), "DENY");
    assert_eq!(
        res.headers().get(header::CONTENT_SECURITY_POLICY).unwrap(),
        "default-src 'none'; frame-ancestors 'none'"
    );
}

#[tokio::test]
async fn test_echo_validation() {
    let app = setup().await;
    let cfg = Config::from_env().unwrap();
    let path = format!("/api/{}/echo", cfg.api_prefix);

    let res = app
        .clone()
        .oneshot(post_request(&path, r#"{"message":"  "}"#))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let res = app
        .clone()
        .oneshot(post_request(&path, r#"{"message":"hello"}"#))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["message"], "hello");
}

#[tokio::test]
async fn test_error_demo() {
    let app = setup().await;
    let cfg = Config::from_env().unwrap();
    let base = format!("/api/{}/error", cfg.api_prefix);

    let res = app
        .clone()
        .oneshot(get_request(&format!("{base}/validation")))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let res = app
        .clone()
        .oneshot(get_request(&format!("{base}/unauthorized")))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let res = app
        .clone()
        .oneshot(get_request(&format!("{base}/conflict")))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT);

    let res = app
        .clone()
        .oneshot(get_request(&format!("{base}/not-found")))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = app
        .oneshot(get_request(&format!("{base}/internal")))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_body_limit() {
    let app = setup().await;
    let cfg = Config::from_env().unwrap();
    let path = format!("/api/{}/echo", cfg.api_prefix);

    let huge_payload = format!(r#"{{"message":"{}"}}"#, "A".repeat(3 * 1024 * 1024));

    let res = app
        .oneshot(post_request(&path, &huge_payload))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_cors() {
    let app = setup().await;
    let cfg = Config::from_env().unwrap();
    let path = format!("/api/{}/hello", cfg.api_prefix);

    let req = Request::builder()
        .uri(&path)
        .method("OPTIONS")
        .header(header::ORIGIN, "http://localhost:3000")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(inject_ip(req)).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .unwrap(),
        "*"
    );
}

#[tokio::test]
async fn test_rate_limiting() {
    let app = setup().await;

    for _ in 0..5 {
        let res = app.clone().oneshot(get_request("/health")).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    let res = app.oneshot(get_request("/health")).await.unwrap();
    assert_eq!(res.status(), StatusCode::TOO_MANY_REQUESTS);
}
