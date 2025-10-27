use crate::features::membership::dto::*;

use crate::features::binances::{PurchaseMembershipResponse, create_binance_subscription};
use crate::{AppState, Error, features::membership::*, models::user::User, types::*};
use aide::NoApi;
use axum::{Json, extract::State};
use bdk::prelude::*;

/// Purchase a membership
pub async fn purchase_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Json(req): Json<PurchaseMembershipRequest>,
) -> Result<Json<PurchaseMembershipResponse>, Error> {
    let cli = &dynamo.client;

    let user = user.ok_or(Error::NoUserFound)?;

    // Get membership details
    let membership_pk = Partition::Membership(req.membership_id.clone());
    let membership_sk = Some(EntityType::Membership);

    let membership = Membership::get(cli, membership_pk.clone(), membership_sk)
        .await?
        .ok_or(Error::NotFound("Membership not found".to_string()))?;

    // Check if membership is active
    if !membership.is_active {
        return Err(Error::BadRequest(
            "This membership is not available for purchase".to_string(),
        ));
    }

    // Check if user already has an active membership
    let existing_membership =
        UserMembership::get(cli, user.pk.clone(), Some(EntityType::UserMembership)).await?;

    if let Some(existing) = existing_membership {
        if existing.is_active() {
            return Err(Error::AlreadyExists(
                "You already have an active membership. Please cancel it first or wait for it to expire.".to_string(),
            ));
        }
    }

    let membership = create_binance_subscription(user.pk, membership).await?;

    // Create user membership
    // let mut user_membership = UserMembership::new(
    //     user.pk.clone(),
    //     membership_pk,
    //     membership.duration_days,
    //     membership.credits,
    //     membership.price_dollars,
    // )?;

    // // Set transaction details if provided
    // if let Some(transaction_id) = req.transaction_id {
    //     user_membership.transaction_id = Some(transaction_id);
    // }

    // user_membership.create(cli).await?;

    Ok(Json(membership))
}
