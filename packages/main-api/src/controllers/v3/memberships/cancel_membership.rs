use crate::features::membership::dto::*;

use crate::{AppState, Error2, features::membership::*, models::user::User, types::*};
use aide::NoApi;
use axum::{Json, extract::State};
use bdk::prelude::*;

/// Cancel current user's membership
pub async fn cancel_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Json(req): Json<CancelMembershipRequest>,
) -> Result<Json<UserMembershipResponse>, Error2> {
    let cli = &dynamo.client;

    let user = user.ok_or(Error2::NoUserFound)?;

    // Get user's membership
    let pk = user.pk.clone();
    let sk = Some(EntityType::UserMembership);

    let mut user_membership = UserMembership::get(cli, pk, sk)
        .await?
        .ok_or(Error2::NotFound("No active membership found".to_string()))?;

    // Check if membership is already cancelled
    if user_membership.get_status() == MembershipStatus::Cancelled {
        return Err(Error2::BadRequest(
            "Membership is already cancelled".to_string(),
        ));
    }

    // Cancel membership
    user_membership.cancel(req.reason);

    user_membership.create(cli).await?;

    Ok(Json(user_membership.into()))
}
