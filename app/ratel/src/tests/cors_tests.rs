use super::*;
use axum::http::{header, Method, StatusCode};

/// Build an Axum router that includes the CORS layer, mirroring what
/// `common::run` does in production.  `TestContext::setup()` intentionally
/// omits the CORS layer so other tests stay lightweight; we wire it here
/// explicitly to exercise the CORS middleware in isolation.
async fn build_cors_app() -> axum::Router {
    let config = crate::config::get();
    let cli = config.common.dynamodb();

    let session_layer = crate::common::middlewares::session_layer::get_session_layer(
        cli,
        config.common.env.to_string(),
    );

    let cors_layer = {
        use tower_http::cors::{AllowOrigin, CorsLayer};
        let allow = AllowOrigin::predicate(|origin, _req_parts| {
            let bytes = origin.as_bytes();
            bytes == b"http://tauri.localhost"
                || bytes == b"https://tauri.localhost"
                || bytes == b"https://ratel.foundation"
                || (bytes.starts_with(b"https://") && bytes.ends_with(b".ratel.foundation"))
        });
        CorsLayer::new()
            .allow_origin(allow)
            .allow_credentials(true)
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::PUT,
                axum::http::Method::PATCH,
                axum::http::Method::DELETE,
                axum::http::Method::OPTIONS,
            ])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
                axum::http::header::ACCEPT,
                axum::http::header::COOKIE,
            ])
    };

    let mcp_router = crate::common::mcp::mcp_router();
    let dioxus_router = dioxus::server::router(crate::App).merge(mcp_router);
    let app = dioxus_router
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(cors_layer)
        .layer(session_layer);
    crate::common::mcp::set_app_router(app.clone());
    app
}

#[tokio::test]
async fn cors_preflight_allows_tauri_localhost() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("error")
        .try_init();

    let app = build_cors_app().await;

    let req = axum::http::Request::builder()
        .method(Method::OPTIONS)
        .uri("/api/auth/me")
        .header(header::ORIGIN, "http://tauri.localhost")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
        .header(header::ACCESS_CONTROL_REQUEST_HEADERS, "content-type")
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = tower::ServiceExt::oneshot(app, req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "preflight should succeed");

    let headers = resp.headers();
    let allow_origin = headers.get(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    assert_eq!(
        allow_origin.and_then(|v| v.to_str().ok()),
        Some("http://tauri.localhost"),
        "allow-origin must echo the tauri origin"
    );
    let allow_credentials = headers.get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS);
    assert_eq!(
        allow_credentials.and_then(|v| v.to_str().ok()),
        Some("true"),
        "credentials must be allowed for cookie auth"
    );
}

#[tokio::test]
async fn cors_preflight_rejects_unknown_origin() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("error")
        .try_init();

    let app = build_cors_app().await;

    let req = axum::http::Request::builder()
        .method(Method::OPTIONS)
        .uri("/api/auth/me")
        .header(header::ORIGIN, "https://evil.example.com")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = tower::ServiceExt::oneshot(app, req).await.unwrap();
    let allow_origin = resp.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    assert!(
        allow_origin.is_none(),
        "unknown origin must not be echoed in allow-origin header"
    );
}
