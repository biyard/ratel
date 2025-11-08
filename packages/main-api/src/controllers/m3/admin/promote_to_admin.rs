use super::list_admins::AdminUserResponse;
use crate::{
    AppState, Error, models::dynamo_tables::main::user::User, types::*,
    utils::time::get_now_timestamp_millis,
};
use axum::{Json, extract::State};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct PromoteToAdminRequest {
    pub email: String,
}

/// Promote a user to admin (ServiceAdmin only)
pub async fn promote_to_admin_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<PromoteToAdminRequest>,
) -> Result<Json<AdminUserResponse>, Error> {
    let cli = &dynamo.client;

    // Find user by email
    let (users, _) = User::find_by_email(cli, req.email, User::opt_one()).await?;

    let user = users.into_iter().next().ok_or(Error::NoUserFound)?;

    // Check if user is already an admin
    if user.is_admin() {
        return Err(Error::UserAlreadyAdmin);
    }

    // Promote to admin using updater
    let updated_user = User::updater(&user.pk, EntityType::User)
        .with_user_type(UserType::Admin)
        .with_updated_at(get_now_timestamp_millis())
        .execute(cli)
        .await?;

    Ok(Json(updated_user.into()))
}
