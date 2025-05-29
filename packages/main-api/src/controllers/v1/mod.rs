pub mod assembly_members;
pub mod bills;
// pub mod patrons;
// pub mod topics;
mod bots;
mod election_pledges;
mod feeds;
mod news;
mod presidential_candidates;
mod quizzes;
mod followers;
pub mod subscriptions;
pub mod supports;
mod teams;
pub mod users;

use bdk::prelude::*;

use dto::*;

pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
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
            "/followers",
            followers::FollowerController::new(pool.clone()).route()?,
        )
        .nest(
            "/subscriptions",
            subscriptions::SubscriptionController::new(pool.clone()).route(),
        ))
}
