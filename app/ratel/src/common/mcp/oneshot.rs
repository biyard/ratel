use std::sync::OnceLock;

use dioxus::server::axum::{self, Router};

static APP_ROUTER: OnceLock<Router> = OnceLock::new();

pub fn set_app_router(router: Router) {
    APP_ROUTER.set(router).ok();
}

pub fn get_app_router() -> Router {
    APP_ROUTER
        .get()
        .expect("MCP app router not initialized. Call set_app_router() in run.rs")
        .clone()
}

/// Send an HTTP request through the Axum router via `tower::ServiceExt::oneshot`.
/// Used by `#[mcp_tool]`-generated `_mcp_impl` functions.
pub async fn mcp_oneshot<T: serde::de::DeserializeOwned>(
    method: &str,
    path: &str,
    mcp_secret: &str,
    body: Option<Vec<u8>>,
) -> crate::common::Result<T> {
    use tower::ServiceExt;

    let router = get_app_router();
    let encoded_path = path.replace("#", "%23");

    let mut builder = axum::http::Request::builder()
        .uri(format!("http://localhost{}", encoded_path))
        .method(method)
        .header("authorization", format!("McpSecret {}", mcp_secret));

    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }

    let req = builder
        .body(axum::body::Body::from(body.unwrap_or_default()))
        .map_err(|e| {
            crate::common::Error::InternalServerError(format!(
                "Failed to build MCP oneshot request: {e}"
            ))
        })?;

    let res = router.oneshot(req).await.map_err(|e| {
        crate::common::Error::InternalServerError(format!("MCP oneshot failed: {e}"))
    })?;

    let (parts, body) = res.into_parts();
    let bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
        .await
        .map_err(|e| {
            crate::common::Error::InternalServerError(format!(
                "Failed to read MCP oneshot response: {e}"
            ))
        })?;

    if !parts.status.is_success() {
        let msg = String::from_utf8_lossy(&bytes).to_string();
        return Err(crate::common::Error::InternalServerError(msg));
    }

    serde_json::from_slice(&bytes).map_err(|e| {
        crate::common::Error::InternalServerError(format!(
            "Failed to parse MCP oneshot response: {e}"
        ))
    })
}
