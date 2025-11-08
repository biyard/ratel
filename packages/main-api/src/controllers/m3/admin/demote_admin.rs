use crate::{AppState, Error, models::dynamo_tables::main::user::User, types::*, utils::time::get_now_timestamp_millis};
use axum::{Json, extract::{Path, State}};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Default)]
pub struct DemoteAdminResponse {
    pub success: bool,
    pub message: String,
}

/// Demote an admin to regular user (ServiceAdmin only)
pub async fn demote_admin_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<DemoteAdminResponse>, Error> {
    let cli = &dynamo.client;

    let pk = Partition::User(user_id.clone());
    let user = User::get(cli, pk.clone(), Some(EntityType::User))
        .await?
        .ok_or(Error::NoUserFound)?;

    // Check if user is an admin
    if !user.is_admin() {
        return Err(Error::UserNotAdmin);
    }

    // Demote to individual user using updater
    User::updater(&pk, EntityType::User)
        .with_user_type(UserType::Individual)
        .with_updated_at(get_now_timestamp_millis())
        .execute(cli)
        .await?;

    Ok(Json(DemoteAdminResponse {
        success: true,
        message: format!("User {} has been demoted from admin", user_id),
    }))
}
