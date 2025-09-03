mod config;
mod controllers;
// mod modules;
// mod utils;

use bdk::prelude::{
    by_axum::{auth::authorization_middleware, axum::Router, axum::middleware},
    *,
};
use dto::*;
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
        BillWriter,
        USBillWriter,
        HKBillWriter,
        CHBillWriter,
        EUBillWriter,
        EventLog,
        Space,
        SpaceComment,
        SpaceUser,
        SpaceMember,
        SpaceContract,
        SpaceHolder,
    );

    tracing::info!("Migration done");
    Ok(())
}

async fn api_main() -> Result<Router> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);

    let pool = if let by_types::DatabaseConfig::Postgres { url, pool_size } = conf.database {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    if conf.migrate {
        migration(&pool).await?;
    }

    let app = app
        .nest("/m1", controllers::m1::route(pool).await?)
        .layer(middleware::from_fn(authorization_middleware));

    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = api_main().await?;

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

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
        let app = api_main().await?;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let conf = config::get();
        tracing::debug!("config: {:?}", conf);

        let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
            PgPoolOptions::new()
                .max_connections(pool_size)
                .connect(url)
                .await
                .expect("Failed to connect to Postgres")
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

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
