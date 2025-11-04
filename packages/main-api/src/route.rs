use std::sync::Arc;

use bdk::prelude::*;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::{
    AppState, controllers,
    utils::{aws::*, sqs_client::SqsClient, telegram::ArcTelegramBot},
};

pub struct RouteDeps {
    pub sqs_client: Arc<SqsClient>,
    pub bedrock_client: BedrockClient,
    pub rek_client: RekognitionClient,
    pub textract_client: TextractClient,
    pub metadata_s3_client: S3Client,
    pub private_s3_client: S3Client,
    pub bot: Option<ArcTelegramBot>,
    pub dynamo_client: DynamoClient,
    pub ses_client: SesClient,
}

pub async fn route(deps: RouteDeps) -> Result<by_axum::axum::Router, crate::Error> {
    let RouteDeps {
        // sqs_client,
        // bedrock_client,
        // rek_client,
        // textract_client,
        // private_s3_client,
        bot,
        dynamo_client,
        ses_client,
        metadata_s3_client,
        ..
    } = deps;

    Ok(by_axum::axum::Router::new()
        .with_state(AppState::new(dynamo_client.clone(), ses_client.clone(), metadata_s3_client.clone()))
        .nest(
            "/v3",
            controllers::v3::route(controllers::v3::RouteDeps {
                dynamo_client: dynamo_client.clone(),
                ses_client: ses_client.clone(),
                bot: bot.clone(),
                s3: metadata_s3_client.clone(),
            })?,
        )
        .nest(
            "/m3",
            controllers::m3::route(AppState::new(
                dynamo_client.clone(),
                ses_client.clone(),
                metadata_s3_client.clone(),
            ))?,
        )
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
