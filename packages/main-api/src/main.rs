use by_axum::{auth::authorization_middleware, axum::middleware};
use by_types::DatabaseConfig;
use dto::*;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod controllers {
    pub mod m1;
    pub mod v1;
}

pub mod config;
pub mod models;
pub mod utils;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");
    let u = User::get_repository(pool.clone());
    let t = Topic::get_repository(pool.clone());
    let c = Comment::get_repository(pool.clone());
    let v = Vote::get_repository(pool.clone());
    let a = AssemblyMember::get_repository(pool.clone());
    let p = Patron::get_repository(pool.clone());
    let f = Feature::get_repository(pool.clone());

    u.create_this_table().await?;
    t.create_this_table().await?;
    c.create_this_table().await?;
    v.create_this_table().await?;
    a.create_this_table().await?;
    p.create_this_table().await?;
    f.create_this_table().await?;

    u.create_related_tables().await?;
    t.create_related_tables().await?;
    c.create_related_tables().await?;
    v.create_related_tables().await?;
    a.create_related_tables().await?;
    p.create_related_tables().await?;
    f.create_related_tables().await?;

    tracing::info!("Migration done");
    Ok(())
}

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
    rest_api::set_message(conf.signing_domain.to_string());

    migration(&pool).await?;

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
            "/v1/assembly-members",
            controllers::v1::assembly_members::AssemblyMemberControllerV1::route(pool.clone())?,
        )
        .nest(
            "/v1/patrons",
            controllers::v1::patrons::PatronControllerV1::route(pool.clone())?,
        )
        .nest(
            "/m1",
            controllers::m1::MenaceController::route(pool.clone())?,
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
