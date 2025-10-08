use std::env;

use crate::{
    AppState, config,
    controllers::{self, web},
    route::{RouteDeps, route},
    utils::{
        aws::{
            BedrockClient, DynamoClient, RekognitionClient, S3Client, SesClient, TextractClient,
            get_aws_config,
        },
        dynamo_session_store::DynamoSessionStore,
        mcp_middleware::mcp_middleware,
        sqs_client,
        telegram::TelegramBot,
    },
};

use bdk::prelude::{by_axum::axum::Router, *};
use by_axum::axum::middleware;
use by_types::DatabaseConfig;
use dto::{
    by_axum::auth::{authorization_middleware, set_auth_token_key},
    sqlx::PgPool,
    *,
};
use sqlx::postgres::PgPoolOptions;
use tower_sessions::{
    SessionManagerLayer,
    cookie::time::{Duration, OffsetDateTime},
};

pub async fn db_init(url: &'static str, max_conn: u32) -> Result<PgPool> {
    let url = if let Ok(host) = env::var("PGHOST") {
        let url = if let Some(at_pos) = url.rfind('@') {
            let (before_at, after_at) = url.split_at(at_pos + 1);
            if let Some(slash_pos) = after_at.find('/') {
                let (_, after_slash) = after_at.split_at(slash_pos);
                format!("{}{}{}", before_at, host, after_slash)
            } else {
                url.to_string()
            }
        } else {
            url.to_string()
        };
        url
    } else {
        url.to_string()
    };

    tracing::debug!("Connecting to database at {}", url);

    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&url)
        .await?;

    Ok(pool)
}

pub async fn api_main() -> Result<Router> {
    let app = by_axum::new();
    let conf = config::get();
    by_axum::auth::set_auth_config(conf.auth.clone());

    let auth_token_key = format!("{}_auth_token", conf.env);
    let auth_token_key = Box::leak(Box::new(auth_token_key));
    set_auth_token_key(auth_token_key);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        let pool = db_init(url, pool_size).await?;
        tracing::info!(
            "Connected to Postgres at {}",
            pool.connect_options().get_host()
        );
        pool
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let is_local = conf.env == "local" || conf.env == "test";
    let aws_sdk_config = get_aws_config();
    let dynamo_client = DynamoClient::new(Some(aws_sdk_config.clone()));
    let ses_client = SesClient::new(aws_sdk_config, is_local);

    let sqs_client = sqs_client::SqsClient::new().await;
    let bedrock_client = BedrockClient::new();
    let rek_client = RekognitionClient::new();
    let textract_client = TextractClient::new();
    let private_s3_client = S3Client::new(conf.private_bucket_name);
    let metadata_s3_client = S3Client::new(conf.bucket.name);

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
    let mcp_router = by_axum::axum::Router::new()
        .nest_service("/mcp", controllers::mcp::route(pool.clone()).await?)
        .layer(middleware::from_fn(mcp_middleware));
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

    let app_state = AppState {
        dynamo: dynamo_client.clone(),
        pool: pool.clone(),
        ses: ses_client.clone(),
    };
    let web = web::route(app_state)?;

    let api_router = route(RouteDeps {
        pool,
        sqs_client,
        bedrock_client,
        rek_client,
        textract_client,
        metadata_s3_client,
        private_s3_client,
        bot,
        dynamo_client,
        ses_client,
    })
    .await?;

    let app = app
        .merge(mcp_router)
        .merge(web)
        .merge(api_router)
        .layer(middleware::from_fn(authorization_middleware))
        .layer(session_layer);

    Ok(app)
}
