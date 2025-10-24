use std::env;

use bdk::prelude::*;
use main_api::{
    Error,
    api_main::api_main,
    config,
    utils::{
        aws::{DynamoClient, get_aws_config},
        telegram::TelegramBot,
    },
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conf = config::get();
    let aws_sdk_config = get_aws_config();
    let dynamo_client = DynamoClient::new(Some(aws_sdk_config.clone()));

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

    let app = api_main(bot.clone()).await?;

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    let axum_handler = by_axum::serve(listener, app);

    if let Some(ref bot) = bot {
        tokio::select! {
            result = bot.dispatcher(&dynamo_client.client) => {
                tracing::info!("Teloxide dispatcher finished: {:?}", result);
            }
            result = axum_handler => {
                if let Err(e) = result {
                    tracing::error!("Axum server has failed: {}", e);
                } else {
                    tracing::info!("Axum server finished successfully");
                }
            }
        }
    } else {
        axum_handler.await.unwrap();
    }

    Ok(())
}
