use by_axum::axum::middleware;
use by_types::DatabaseConfig;
use dto::*;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use utils::middlewares::authorization_middleware;

mod controllers {
    pub mod m1;
    pub mod v1;
}

pub mod config;
pub mod models;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let app = app
        // .nest(
        //     "/v1/patrons",
        //     controllers::patrons::v1::PatronControllerV1::route()?,
        // )
        .nest(
            "/v1/topics",
            controllers::v1::topics::TopicControllerV1::route(pool.clone())?,
        )
        .nest(
            "/v1/users",
            controllers::v1::users::UserControllerV1::route(pool.clone())?,
        )
        .nest(
            "/v1/assembly_members",
            controllers::v1::assembly_members::AssemblyMemberControllerV1::route(pool.clone())?,
        )
        .nest(
            "/m1/assembly_members",
            controllers::m1::assembly_members::AssemblyMemberControllerM1::route(pool)?,
        )
        .layer(middleware::from_fn(authorization_middleware));

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
