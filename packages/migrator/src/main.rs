#![allow(warnings)]
mod migrate_poll_spaces;
mod migrate_posts;
mod migrate_teams;
mod migrate_users;

use aws_sdk_dynamodb::{
    Client, Config,
    config::{Credentials, Region},
};
use dto::sqlx::postgres::PgPoolOptions;
use main_api::*;

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:postgres@localhost:5432/ratel")
        .await
        .expect("Failed to create Postgres connection pool");

    let conf = Config::builder()
        .credentials_provider(
            Credentials::builder()
                .access_key_id(env!("AWS_ACCESS_KEY_ID"))
                .secret_access_key(env!("AWS_SECRET_ACCESS_KEY"))
                .provider_name("ratel")
                .build(),
        )
        .region(Region::new("us-east-1"))
        .behavior_version_latest()
        .endpoint_url("http://localhost:4566")
        .build();

    let cli = Client::from_conf(conf);

    migrate_users::migrate_users(&pool, &cli).await;
    migrate_teams::migrate_teams(&pool, &cli).await;
    migrate_posts::migrate_posts(&pool, &cli).await;
    migrate_poll_spaces::migrate_poll_spaces(&pool, &cli).await;

    // Migrate Survey for poll spaces

    // Migrate survey responses for poll spaces

    // TODO: Migrate SpaceType::Commitee with badges
    // let commitee_spaces: Vec<dto::Space> = dto::Space::query_builder(0)
    //     .space_type_equals(dto::SpaceType::Commitee)
    //     .query()
    //     .map(Into::into)
    //     .fetch_all(&pool)
    //     .await
    //     .expect("Failed to fetch commitee spaces from Postgres");
    // tracing::info!(
    //     "Total commitee spaces to migrate: {}",
    //     commitee_spaces.len()
    // );

    // let badges = commitee_spaces[0].badges.clone();
    // tracing::info!("Badges in first commitee space: {:?}", badges.len());

    // let user_badges: Vec<dto::UserBadge> = dto::UserBadge::query_builder()
    //     .query()
    //     .map(Into::into)
    //     .fetch_all(&pool)
    //     .await
    //     .expect("Failed to fetch user badges for commitee space from Postgres");

    // tracing::info!(
    //     "Total user badges to migrate for commitee space: {}",
    //     user_badges.len()
    // );
}
