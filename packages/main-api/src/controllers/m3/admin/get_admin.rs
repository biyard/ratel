use super::list_admins::AdminUserResponse;
use crate::{AppState, Error, models::dynamo_tables::main::user::User, types::*};
use axum::{Json, extract::{Path, State}};
use bdk::prelude::*;

/// Get a specific admin user by ID (ServiceAdmin only)
pub async fn get_admin_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<AdminUserResponse>, Error> {
    let cli = &dynamo.client;

    let pk = Partition::User(user_id.clone());
    let user = User::get(cli, pk, Some(EntityType::User))
        .await?
        .ok_or(Error::NoUserFound)?;

    if !user.is_admin() {
        return Err(Error::UserNotAdmin);
    }

    Ok(Json(user.into()))
}
