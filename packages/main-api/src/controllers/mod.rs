mod hooks;
pub mod m3;
pub mod v3;
pub mod web;
// pub mod well_known {
//     pub mod get_did_document;
// }

use std::sync::Arc;

use bdk::prelude::*;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::{
    AppState, controllers,
    utils::{aws::*, sqs_client::SqsClient, telegram::ArcTelegramBot},
};

pub async fn route(bot: Option<ArcTelegramBot>) -> Result<by_axum::axum::Router, crate::Error> {
    Ok(by_axum::axum::Router::new()
        .with_state(AppState::default())
        // .merge(well_known_router)
        .nest("/v3", v3::route(bot)?)
        .nest("/m3", m3::route()?)
        .nest("/hooks", hooks::route()?)
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
