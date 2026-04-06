use crate::common::axum::{
    AxumRouter, Json,
    http::StatusCode,
    native_routing::post,
    response::IntoResponse,
};
use crate::features::membership::controllers::{PortoneRequest, handle_portone_webhook};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct WebhookResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct WebhookErrorResponse {
    error: String,
}

async fn portone_webhook(Json(req): Json<PortoneRequest>) -> impl IntoResponse {
    match handle_portone_webhook(req).await {
        Ok(()) => (
            StatusCode::OK,
            Json(WebhookResponse { status: "ok" }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(WebhookErrorResponse {
                error: err.to_string(),
            }),
        )
            .into_response(),
    }
}

pub fn router() -> AxumRouter {
    AxumRouter::new().route("/hooks/portone", post(portone_webhook))
}
