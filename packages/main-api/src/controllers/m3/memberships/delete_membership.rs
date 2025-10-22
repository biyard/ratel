use crate::features::membership::dto::*;
use crate::{
    AppState, Error2, aide::OperationIo, controllers::v3::verify_service_admin,
    features::membership::*, models::user::User, types::*,
};
use aide::NoApi;
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct DeleteMembershipResponse {
    pub success: bool,
    pub message: String,
}

/// Delete membership (ServiceAdmin only)
pub async fn delete_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(MembershipPathParam { membership_id }): Path<MembershipPathParam>,
) -> Result<Json<DeleteMembershipResponse>, Error2> {
    let cli = &dynamo.client;

    // Delete membership
    let pk = Partition::Membership(membership_id);
    let sk = Some(EntityType::Membership);

    Membership::delete(cli, pk, sk).await?;

    Ok(Json(DeleteMembershipResponse {
        success: true,
        message: "Membership deleted successfully".to_string(),
    }))
}
