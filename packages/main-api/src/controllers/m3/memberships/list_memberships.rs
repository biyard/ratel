use crate::features::membership::dto::*;
use crate::{
    AppState, Error2, controllers::v3::verify_service_admin, features::membership::*,
    models::user::User,
};
use aide::NoApi;
use axum::{Json, extract::State};
use bdk::prelude::*;

/// List all available memberships (ServiceAdmin only)
///
/// Returns all active memberships sorted by display_order
pub async fn list_memberships_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
) -> Result<Json<ListMembershipsResponse>, Error2> {
    let cli = &dynamo.client;

    // Verify user is a ServiceAdmin
    let _admin = verify_service_admin(user, cli).await?;

    // Query active memberships using GSI1
    let (memberships, _) = Membership::find_active(
        cli,
        "ACTIVE#true".to_string(),
        MembershipQueryOption::builder()
            .limit(100)
            .sk("ORDER".to_string()),
    )
    .await?;

    let total = memberships.len();
    let memberships: Vec<MembershipResponse> = memberships.into_iter().map(|m| m.into()).collect();

    Ok(Json(ListMembershipsResponse { memberships, total }))
}
