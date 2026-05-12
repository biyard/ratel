use crate::common::axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use crate::features::membership::controllers::{handle_portone_webhook, PortoneRequest};
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
        Ok(()) => (StatusCode::OK, Json(WebhookResponse { status: "ok" })).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(WebhookErrorResponse {
                error: err.to_string(),
            }),
        )
            .into_response(),
    }
}

pub fn router() -> Router {
    Router::new().route("/hooks/portone", post(portone_webhook))
}
