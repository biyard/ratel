pub mod assets;
mod auth;
mod bots;
pub mod feeds;
mod me;
mod my_networks;
mod network;
pub mod news;
mod notifications;
pub mod promotions;
mod spaces;
pub mod subscriptions;
mod teams;
pub mod users;

mod redeems;

use bdk::prelude::*;

use dto::*;

use crate::config;

pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    let conf = config::get();
    Ok(by_axum::axum::Router::new()
        .nest("/auth", auth::AuthController::new(pool.clone()).route()?)
        .nest("/me", me::MeController::new(pool.clone()).route()?)
        .nest(
            "/spaces",
            spaces::SpaceController::new(pool.clone()).route().await?,
        )
        .nest(
            "/network",
            network::NetworkController::new(pool.clone()).route()?,
        )
        .nest(
            "/assets",
            assets::AssetController::new(&conf.aws, &conf.bucket).route()?,
        )
        .nest(
            "/promotions",
            promotions::PromotionController::new(pool.clone()).route()?,
        )
        .nest("/news", news::NewsController::new(pool.clone()).route()?)
        .nest("/bots", bots::BotController::new(pool.clone()).route()?)
        .nest("/teams", teams::TeamController::new(pool.clone()).route()?)
        .nest("/feeds", feeds::FeedController::new(pool.clone()).route()?)
        .nest("/users", users::UserControllerV1::route(pool.clone())?)
        .nest(
            "/subscriptions",
            subscriptions::SubscriptionController::new(pool.clone()).route(),
        )
        .nest(
            "/redeems",
            redeems::RedeemCodeController::new(pool.clone()).route(),
        )
        .nest(
            "/my-networks",
            my_networks::MynetworkController::new(pool.clone()).route()?,
        )
        .nest(
            "/notifications",
            notifications::NotificationController::new(pool.clone()).route()?,
        ))
}
