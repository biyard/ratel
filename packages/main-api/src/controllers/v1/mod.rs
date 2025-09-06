pub mod assembly_members;
pub mod bills;
// pub mod patrons;
// pub mod topics;
mod advocacy_campaigns;
pub mod assets;
mod auth;
mod bots;
mod election_pledges;
pub mod feeds;
mod landing;
mod me;
mod my_networks;
mod network;
mod news;
mod notifications;
mod presidential_candidates;
mod promotions;
mod quizzes;
mod spaces;
pub mod subscriptions;
pub mod supports;
mod teams;
mod totals;
pub mod users;

mod redeems;

use bdk::prelude::*;

use dto::*;

use crate::config;

pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    let conf = config::get();
    Ok(by_axum::axum::Router::new()
        .nest("/auth", auth::AuthController::new(pool.clone()).route()?)
        .nest(
            "/advocacy-campaigns",
            advocacy_campaigns::AdvocacyCampaignController::new(pool.clone()).route()?,
        )
        .nest("/me", me::MeController::new(pool.clone()).route()?)
        .nest(
            "/spaces",
            spaces::SpaceController::new(pool.clone()).route().await?,
        )
        .nest(
            "/totals",
            totals::TotalController::new(pool.clone()).route()?,
        )
        .nest(
            "/landings",
            landing::LandingController::new(pool.clone()).route()?,
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
        .nest(
            "/quizzes",
            quizzes::QuizController::new(pool.clone()).route()?,
        )
        .nest("/news", news::NewsController::new(pool.clone()).route()?)
        .nest("/bots", bots::BotController::new(pool.clone()).route()?)
        .nest("/teams", teams::TeamController::new(pool.clone()).route()?)
        .nest("/feeds", feeds::FeedController::new(pool.clone()).route()?)
        .nest(
            "/election-pledges",
            election_pledges::ElectionPledgeController::new(pool.clone()).route()?,
        )
        .nest(
            "/presidential-candidates",
            presidential_candidates::PresidentialCandidateController::new(pool.clone()).route()?,
        )
        .nest("/users", users::UserControllerV1::route(pool.clone())?)
        .nest(
            "/assembly-members",
            assembly_members::AssemblyMemberControllerV1::route(pool.clone())?,
        )
        .nest("/bills", bills::BillController::new(pool.clone()).route())
        .nest(
            "/supports",
            supports::SupportController::route(pool.clone())?,
        )
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
