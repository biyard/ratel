mod config;

mod notify;

use notify::notify_handler;

mod telegram_handler;
use std::sync::Arc;
use telegram_handler::{set_command, telegram_handler};

use base64::{Engine, engine::general_purpose};
use serde::Serialize;

use dto::{
    Result, TelegramSubscribe,
    by_axum::{self, axum::routing::post},
    by_types::DatabaseConfig,
    sqlx::{PgPool, migrate, postgres::PgPoolOptions},
};
use teloxide::{Bot, dispatching::UpdateFilterExt, dptree, prelude::Dispatcher, types::Update};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

macro_rules! migrate {
    ($pool:ident, $($table:ident),* $(,)?) => {
        {
            $(
                let t = $table::get_repository($pool.clone());
                t.create_this_table().await?;
            )*
            $(
                let t = $table::get_repository($pool.clone());
                t.create_related_tables().await?;
            )*
        }
    };
}

pub async fn migration(pool: &PgPool) -> Result<()> {
    tracing::info!("Running migration");

    migrate!(pool, TelegramSubscribe);
    Ok(())
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    bot: Bot,
}

#[tokio::main]
async fn main() {
    let conf = config::get();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from(conf.log_level))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    tracing::debug!("Configuration: {:?}", conf.env);
    tracing::debug!("Starting throw dice bot...");

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await
            .expect("Failed to connect to the database")
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    migration(&pool).await.expect("Failed to run migrations");

    let bot = Bot::new(conf.telegram_token);

    let state = Arc::new(AppState {
        pool: pool.clone(),
        bot: bot.clone(),
    });
    let app = by_axum::axum::Router::new()
        .route("/notify", post(notify_handler))
        .with_state(state);

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    let axum_server = by_axum::serve(listener, app);
    set_command(bot.clone()).await;

    let handler = Update::filter_message().endpoint(telegram_handler);

    let mut binding = Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool])
        .enable_ctrlc_handler()
        .build();
    let teloxide_dispatcher = binding.dispatch();

    tokio::select! {
        result = teloxide_dispatcher => {
            tracing::info!("Teloxide dispatcher finished: {:?}", result);
        }
        result = axum_server => {
            if let Err(e) = result {
                tracing::error!("Axum server has failed: {}", e);
            } else {
                tracing::info!("Axum server finished successfully");
            }
        }
    }

    tracing::info!("Application shutting down...");
}

#[derive(Serialize)]
pub struct TgWebParams {
    pub command: TgWebCommand,
}

#[derive(Serialize)]
pub enum TgWebCommand {
    Subscribe { chat_id: i64, lang: Option<String> },
    SprintLeague { space_id: i64 },
}

pub fn generate_link(web_command: TgWebCommand) -> String {
    let params = TgWebParams {
        command: web_command,
    };
    let json_string = serde_json::to_string(&params).unwrap();
    let b64_string = general_purpose::STANDARD.encode(json_string);
    format!(
        "{}/app?startapp={}",
        config::get().telegram_mini_app_uri,
        b64_string
    )
}
