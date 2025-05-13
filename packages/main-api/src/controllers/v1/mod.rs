pub mod assembly_members;
pub mod bills;
// pub mod patrons;
// pub mod topics;
mod election_pledges;
mod feeds;
mod presidential_candidates;
mod promotions;
pub mod subscriptions;
pub mod supports;
pub mod users;

use bdk::prelude::*;

use dto::*;

pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
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
            "/promotions",
            promotions::PromotionController::new(pool.clone()).route()?,
        ))
}
