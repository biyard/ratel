use crate::features::membership::dto::*;
use crate::{AppState, Error, features::membership::*, types::*};
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;

/// Update membership (ServiceAdmin only)
pub async fn update_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(MembershipPathParam { membership_id }): Path<MembershipPathParam>,
    Json(req): Json<UpdateMembershipRequest>,
) -> Result<Json<MembershipResponse>, Error> {
    let cli = &dynamo.client;

    let pk = Partition::Membership(membership_id);

    let membership = Membership::updater(&pk, EntityType::Membership)
        .with_tier(req.tier)
        .with_price_dollars(req.price_dollars)
        .with_credits(req.credits)
        .with_duration_days(req.duration_days)
        .with_display_order(req.display_order)
        .with_is_active(req.is_active)
        .with_max_credits_per_space(req.max_credits_per_space)
        .execute(cli)
        .await?;

    Ok(Json(membership.into()))
}
