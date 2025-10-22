use crate::features::membership::dto::*;
use crate::{
    AppState, Error2, controllers::v3::verify_service_admin, features::membership::*,
    models::user::User, types::*,
};
use aide::NoApi;
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;

/// Update membership (ServiceAdmin only)
pub async fn update_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(MembershipPathParam { membership_id }): Path<MembershipPathParam>,
    Json(req): Json<UpdateMembershipRequest>,
) -> Result<Json<MembershipResponse>, Error2> {
    let cli = &dynamo.client;

    // Get membership
    let pk = Partition::Membership(membership_id);
    let sk = Some(EntityType::Membership);

    let mut membership = Membership::get(cli, pk, sk)
        .await?
        .ok_or(Error2::NotFound("Membership not found".to_string()))?;

    // Update membership fields
    membership.update(
        req.tier,
        req.price_dollers,
        req.credits,
        req.duration_days,
        req.display_order,
        req.is_active,
    );

    // Save to database
    membership.create(cli).await?;

    Ok(Json(membership.into()))
}
