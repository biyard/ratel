mod config;
mod controllers;

use crate::controllers::telegram::*;
use bdk::prelude::{
    by_axum::{auth::authorization_middleware, axum::Router, axum::middleware},
    *,
};
use dto::{sqlx::PgPool, *};
use teloxide::{Bot, dispatching::UpdateFilterExt, dptree, prelude::Dispatcher, types::Update};
use tokio::net::TcpListener;

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

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");

    migrate!(
        pool,
        EventLog,
        Space,
        SpaceComment,
        SpaceUser,
        SpaceMember,
        SpaceContract,
        SpaceHolder,
        TelegramChannel
    );

    tracing::info!("Migration done");
    Ok(())
}

async fn api_main(pool: &PgPool) -> Result<Router> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);

    if conf.migrate {
        migration(pool).await?;
    }

    let app = app
        .nest("/m1", controllers::m1::route(pool.clone()).await?)
        .layer(middleware::from_fn(authorization_middleware));

    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = if let by_types::DatabaseConfig::Postgres { url, pool_size } = config::get().database
    {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let bot = Bot::new(config::get().telegram_token);
    set_command(bot.clone()).await;

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_my_chat_member().endpoint(member_update_handler));

    let mut binding = Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool.clone()])
        .enable_ctrlc_handler()
        .build();

    let teloxide_dispatcher = binding.dispatch();

    let app = api_main(&pool).await?;

    let port = option_env!("PORT").unwrap_or("4000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    let axum_server = by_axum::serve(listener, app);

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

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use bdk::prelude::{by_types::DatabaseConfig, sqlx::postgres::PgPoolOptions};
    use std::time::SystemTime;

    use super::*;
    use rest_api::ApiService;

    pub struct TestContext {
        pub pool: sqlx::Pool<sqlx::Postgres>,
        pub app: Box<dyn ApiService>,
        pub now: i64,
        pub endpoint: String,
    }

    pub async fn setup() -> Result<TestContext> {
        let conf = config::get();
        let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
            PgPoolOptions::new()
                .max_connections(pool_size)
                .connect(url)
                .await
                .expect("Failed to connect to Postgres")
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

        let app = api_main(&pool).await?;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        tracing::debug!("config: {:?}", conf);

        let _ = sqlx::query(
            r#"
        CREATE OR REPLACE FUNCTION set_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at := EXTRACT(EPOCH FROM now()); -- seconds
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
        "#,
        )
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
        CREATE OR REPLACE FUNCTION set_created_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.created_at := EXTRACT(EPOCH FROM now()); -- seconds
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
        "#,
        )
        .execute(&pool)
        .await;

        let _ = migration(&pool).await;

        let app = by_axum::into_api_adapter(app);
        let app = Box::new(app);
        rest_api::set_api_service(app.clone());
        rest_api::add_authorization(&format!(
            "x-server-key {}",
            option_env!("SERVER_KEY").unwrap()
        ));

        Ok(TestContext {
            pool,
            app,
            now: now as i64,
            endpoint: format!("http://localhost:3000"),
        })
    }
}
