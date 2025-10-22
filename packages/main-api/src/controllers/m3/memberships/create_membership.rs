use crate::features::membership::dto::*;

use crate::{
    AppState, Error2, controllers::v3::verify_service_admin, features::membership::*,
    models::user::User,
};
use aide::NoApi;
use axum::{Json, extract::State};
use bdk::prelude::*;

/// Create a new membership (ServiceAdmin only)
pub async fn create_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Json(req): Json<CreateMembershipRequest>,
) -> Result<Json<MembershipResponse>, Error2> {
    let cli = &dynamo.client;

    // Verify user is a ServiceAdmin
    let _admin = verify_service_admin(user, cli).await?;

    // Create membership
    let membership = Membership::new(
        req.tier,
        req.price_dollers,
        req.credits,
        req.duration_days,
        req.display_order,
    );

    membership.create(cli).await?;

    Ok(Json(membership.into()))
}
