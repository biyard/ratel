mod config;

mod axum_handler;
use axum_handler::notify_handler;

mod telegram_handler;
use std::sync::Arc;
use telegram_handler::telegram_handler;

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

    let port = option_env!("PORT").unwrap_or("4000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    let axum_server = by_axum::serve(listener, app);

    let handler = Update::filter_message().endpoint(telegram_handler);

    let mut binding = Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool])
        .enable_ctrlc_handler()
        .build();
    let teloxide_dispatcher = binding.dispatch();

    let (_, axum_result) = tokio::join!(teloxide_dispatcher, axum_server);

    if let Err(e) = axum_result {
        tracing::error!("Axum server has failed: {}", e);
    }
}
