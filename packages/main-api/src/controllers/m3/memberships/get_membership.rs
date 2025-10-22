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

/// Get membership by ID (ServiceAdmin only)
pub async fn get_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(MembershipPathParam { membership_id }): Path<MembershipPathParam>,
) -> Result<Json<MembershipResponse>, Error2> {
    let cli = &dynamo.client;

    // Verify user is a ServiceAdmin
    let _admin = verify_service_admin(user, cli).await?;

    let pk = Partition::Membership(membership_id);
    let sk = Some(EntityType::Membership);

    let membership = Membership::get(cli, pk, sk)
        .await?
        .ok_or(Error2::NotFound("Membership not found".to_string()))?;

    Ok(Json(membership.into()))
}
