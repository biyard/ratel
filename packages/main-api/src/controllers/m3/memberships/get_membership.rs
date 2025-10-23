use crate::features::membership::dto::*;
use crate::{AppState, Error, features::membership::*, types::*};
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;

/// Get membership by ID (ServiceAdmin only)
pub async fn get_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(MembershipPathParam { membership_id }): Path<MembershipPathParam>,
) -> Result<Json<MembershipResponse>, Error> {
    let cli = &dynamo.client;

    let pk = Partition::Membership(membership_id);
    let sk = Some(EntityType::Membership);

    let membership = Membership::get(cli, pk, sk)
        .await?
        .ok_or(Error::NotFound("Membership not found".to_string()))?;

    Ok(Json(membership.into()))
}
