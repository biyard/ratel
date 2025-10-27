use crate::features::membership::dto::*;
use crate::{AppState, Error, features::membership::*, models::user::User, types::*};
use aide::NoApi;
use axum::{Json, extract::State};
use bdk::prelude::*;

/// Get current user's active membership
pub async fn get_my_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
) -> Result<Json<Option<UserMembershipResponse>>, Error> {
    let cli = &dynamo.client;

    let user = user.ok_or(Error::NoUserFound)?;

    // Get user's membership
    let pk = user.pk.clone();
    let sk = Some(EntityType::UserMembership);

    let user_membership = UserMembership::get(cli, pk, sk).await?;

    // Check if membership exists and is active
    let response = user_membership.map(|mut um| {
        // Auto-update expiration if needed
        um.check_and_update_expiration();
        um.into()
    });

    Ok(Json(response))
}
