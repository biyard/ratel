use crate::features::membership::dto::*;

use crate::{AppState, Error2, features::membership::*};
use axum::{Json, extract::State};
use bdk::prelude::*;

/// Create a new membership (ServiceAdmin only)
pub async fn create_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<CreateMembershipRequest>,
) -> Result<Json<MembershipResponse>, Error2> {
    let cli = &dynamo.client;

    // Create membership
    let membership = Membership::new(
        req.tier,
        req.price_dollars,
        req.credits,
        req.duration_days,
        req.display_order,
    );

    membership.create(cli).await?;

    Ok(Json(membership.into()))
}
