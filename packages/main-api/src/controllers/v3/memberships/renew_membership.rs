use crate::features::membership::dto::*;

use crate::{AppState, Error, features::membership::*, models::user::User, types::*};
use aide::NoApi;
use axum::{Json, extract::State};
use bdk::prelude::*;

/// Renew current user's membership
pub async fn renew_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
) -> Result<Json<UserMembershipResponse>, Error> {
    let cli = &dynamo.client;

    let user = user.ok_or(Error::NoUserFound)?;

    // Get user's membership
    let pk = user.pk.clone();
    let sk = Some(EntityType::UserMembership);

    let mut user_membership = UserMembership::get(cli, pk, sk)
        .await?
        .ok_or(Error::NotFound("No membership found".to_string()))?;

    // Check if membership is cancelled
    if user_membership.get_status() == MembershipStatus::Cancelled {
        return Err(Error::BadRequest(
            "Cannot renew a cancelled membership. Please purchase a new one.".to_string(),
        ));
    }

    // Get membership details to get duration
    let membership_pk = user_membership.membership_pk.clone();
    let membership_sk = Some(EntityType::Membership);

    let membership = Membership::get(cli, membership_pk, membership_sk)
        .await?
        .ok_or(Error::NotFound(
            "Associated membership not found".to_string(),
        ))?;

    // Renew membership
    user_membership.renew(membership.duration_days)?;

    user_membership.create(cli).await?;

    Ok(Json(user_membership.into()))
}
