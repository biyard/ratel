mod config;

use aws_config::Region;
use aws_credential_types::Credentials;
use bdk::prelude::*;

use main_api::utils::telegram::TelegramBot;

// mod controllers;
// use bdk::prelude::sqlx::PgPool;
use by_axum::axum::Router;
// use tokio::net::TcpListener;

async fn api_main() -> main_api::Result<Router> {
    // let conf = config::get();

    // let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
    //     let pool = db_init(url, pool_size).await;
    //     tracing::info!(
    //         "Connected to Postgres at {}",
    //         pool.connect_options().get_host()
    //     );
    //     pool
    // } else {
    //     panic!("Database is not initialized. Call init() first.");
    // };

    // let aws_sdk_config = get_aws_config();
    // let dynamo_client = DynamoClient::new(Some(aws_sdk_config.clone()));

    let app = by_axum::new();
    Ok(app)
}

pub fn get_dynamo_client() -> aws_sdk_dynamodb::Client {
    let conf = config::get();

    match conf.dynamodb {
        by_types::DatabaseConfig::DynamoDb {
            ref aws, endpoint, ..
        } => {
            let mut builder = aws_sdk_dynamodb::Config::builder()
                .credentials_provider(
                    Credentials::builder()
                        .access_key_id(aws.access_key_id)
                        .secret_access_key(aws.secret_access_key)
                        .provider_name("ratel")
                        .build(),
                )
                .region(Region::new(aws.region))
                .behavior_version_latest();

            if let Some(endpoint) = endpoint {
                builder = builder.endpoint_url(endpoint.to_string());
            }
            let conf = builder.build();
            let client = aws_sdk_dynamodb::Client::from_conf(conf);
            client
        }
        _ => {
            tracing::error!("DynamoDB config not found.");
            panic!(
                "DynamoDB config not found. In Local env, you must set DynamoDB config with Endpoint"
            )
        }
    }
}

#[tokio::main]
async fn main() -> main_api::Result<()> {
    let _ = api_main().await?;
    let conf = config::get();

    let client = get_dynamo_client();

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

    // let port = env::var("PORT").unwrap_or("3000".to_string());
    // let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
    //     .await
    //     .unwrap();
    // tracing::info!("listening on {}", listener.local_addr().unwrap());
    // let axum_handler = by_axum::serve(listener, app);
    if let Some(ref bot) = bot {
        tokio::select! {
            result = bot.dispatcher(&client) => {
                tracing::debug!("Teloxide dispatcher finished: {:?}", result);
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Received Ctrl+C, shutting down gracefully...");
            }
            // result = axum_handler => {
            //     if let Err(e) = result {
            //         tracing::error!("Axum server has failed: {}", e);
            //     } else {
            //         tracing::info!("Axum server finished successfully");
            //     }
            // }
        }
    } else {
        // axum_handler.await.unwrap();
    }

    Ok(())
}
