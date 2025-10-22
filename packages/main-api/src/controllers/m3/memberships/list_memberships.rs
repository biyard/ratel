use crate::features::membership::dto::*;
use crate::types::ListItemsResponse;
use crate::{AppState, Error2, features::membership::*};
use axum::{Json, extract::State};
use bdk::prelude::*;

/// List all available memberships (ServiceAdmin only)
///
/// Returns all active memberships sorted by display_order
pub async fn list_memberships_handler(
    State(AppState { dynamo, .. }): State<AppState>,
) -> Result<Json<ListItemsResponse<MembershipResponse>>, Error2> {
    let cli = &dynamo.client;

    // Query active memberships using GSI1
    let (memberships, bookmark) =
        Membership::find_active(cli, true, MembershipQueryOption::builder().limit(100)).await?;

    let memberships: Vec<MembershipResponse> = memberships.into_iter().map(|m| m.into()).collect();

    Ok(Json((memberships, bookmark).into()))
}
