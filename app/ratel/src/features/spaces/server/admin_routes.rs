use super::proxy_registry::ProxyRegistry;
use crate::common::axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    AxumRouter, Extension, Json,
    native_routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub base_name: String,
    pub endpoint: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ListResponse {
    pub endpoints: std::collections::HashMap<String, String>,
}

async fn register_endpoint(
    Extension(registry): Extension<ProxyRegistry>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    match registry.register(&req.base_name, &req.endpoint).await {
        Ok(()) => (
            StatusCode::OK,
            Json(RegisterResponse {
                message: format!("Registered '{}' -> '{}'", req.base_name, req.endpoint),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(RegisterResponse { message: e }),
        )
            .into_response(),
    }
}

async fn list_endpoints(Extension(registry): Extension<ProxyRegistry>) -> impl IntoResponse {
    let endpoints = registry.list().await;
    Json(ListResponse { endpoints })
}

async fn unregister_endpoint(
    Extension(registry): Extension<ProxyRegistry>,
    Path(base_name): Path<String>,
) -> impl IntoResponse {
    if registry.unregister(&base_name).await {
        (
            StatusCode::OK,
            Json(RegisterResponse {
                message: format!("Unregistered '{}'", base_name),
            }),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(RegisterResponse {
                message: format!("'{}' not found", base_name),
            }),
        )
            .into_response()
    }
}

pub fn admin_router(registry: ProxyRegistry) -> AxumRouter {
    AxumRouter::new()
        .route("/admin/proxy/endpoints", post(register_endpoint))
        .route("/admin/proxy/endpoints", get(list_endpoints))
        .route(
            "/admin/proxy/endpoints/{base_name}",
            delete(unregister_endpoint),
        )
        .layer(Extension(registry))
}
