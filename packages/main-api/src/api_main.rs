use std::env;

use crate::{
    AppState, config,
    controllers::route,
    controllers::web,
    utils::{
        aws::{
            BedrockClient, DynamoClient, RekognitionClient, S3Client, SesClient, TextractClient,
            get_aws_config,
        },
        dynamo_session_store::DynamoSessionStore,
        sqs_client,
        telegram::TelegramBot,
    },
};

use bdk::prelude::sqlx::PgPool;
use bdk::prelude::{by_axum::axum::Router, *};
use by_types::DatabaseConfig;
use tower_sessions::{
    SessionManagerLayer,
    cookie::time::{Duration, OffsetDateTime},
};

pub async fn api_main() -> Result<Router, crate::Error> {
    let app = by_axum::new();
    let conf = config::get();

    let is_local = conf.env == "local" || conf.env == "test";
    let aws_sdk_config = get_aws_config();
    let dynamo_client = DynamoClient::new(Some(aws_sdk_config.clone()), true);
    let bot = if let Some(token) = conf.telegram_token {
        let res = TelegramBot::new(token).await;
        if let Err(err) = res {
            tracing::error!("Failed to initialize Telegram bot: {}", err);
            None
        } else {
            Some(res.unwrap())
        }
    } else {
        None
    };
    let session_store = DynamoSessionStore::new(dynamo_client.client.clone());

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(!is_local)
        .with_http_only(!is_local)
        .with_same_site(if is_local {
            tower_sessions::cookie::SameSite::Lax
        } else {
            tower_sessions::cookie::SameSite::None
        })
        .with_name(format!("{}_sid", conf.env))
        .with_path("/")
        .with_expiry(tower_sessions::Expiry::AtDateTime(
            OffsetDateTime::now_utc()
                .checked_add(Duration::days(30))
                .unwrap(),
        ));
    // let mcp_router = by_axum::axum::Router::new()
    //     .nest_service("/mcp", controllers::mcp::route(pool.clone()).await.expect("MCP router"))
    //     .layer(middleware::from_fn(mcp_middleware));

    let app_state = AppState::default();
    let web = web::route(app_state)?;

    let api_router = route(bot).await?;

    let app = app
        // .merge(mcp_router)
        .merge(api_router)
        .merge(web)
        // .layer(middleware::from_fn(authorization_middleware))
        .layer(session_layer);

    Ok(app)
}
