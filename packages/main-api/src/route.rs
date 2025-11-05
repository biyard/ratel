use std::sync::Arc;

use bdk::prelude::*;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::{
    AppState, controllers,
    utils::{aws::*, sqs_client::SqsClient, telegram::ArcTelegramBot},
};

pub async fn route(bot: Option<ArcTelegramBot>) -> Result<by_axum::axum::Router, crate::Error> {
    // Create a separate router for non-documented routes (like .well-known)
    let well_known_router = by_axum::axum::AxumRouter::new().route(
        "/.well-known/did.json",
        by_axum::axum::native_routing::get(
            controllers::well_known::get_did_document::get_did_document_handler,
        ),
    );

    Ok(by_axum::axum::Router::new()
        .with_state(AppState::default())
        .merge(well_known_router)
        .nest("/v3", controllers::v3::route(bot)?)
        .nest("/m3", controllers::m3::route()?)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::span!(
                        Level::INFO,
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        headers = ?request.headers(),
                        version = ?request.version()
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        if !response.status().is_success() {
                            tracing::error!(
                                status = %response.status(),
                                latency = ?latency,
                                "error response generated"
                            );
                            return;
                        }

                        tracing::info!(
                            status = %response.status(),
                            latency = ?latency,
                            "response generated"
                        )
                    },
                ),
        ))
}
