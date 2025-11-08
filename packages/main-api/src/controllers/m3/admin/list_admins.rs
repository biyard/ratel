use crate::{AppState, Error, models::dynamo_tables::main::user::User, types::*};
use axum::{Json, extract::State};
use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
    Default,
)]
pub struct AdminUserResponse {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub profile_url: String,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub user_type: UserType,
}

impl From<User> for AdminUserResponse {
    fn from(user: User) -> Self {
        // Extract UUID from Partition::User(uuid)
        let user_id = match user.pk {
            Partition::User(uuid) => format!("User({})", uuid),
            _ => user.pk.to_string(), // Fallback
        };

        Self {
            user_id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            profile_url: user.profile_url,
            created_at: user.created_at,
            user_type: user.user_type,
        }
    }
}

/// List all admin users (ServiceAdmin only)
///
/// Note: This endpoint returns an empty list as a full table scan is not implemented.
/// To check if a user is an admin, use the GET /m3/admin/:user_id endpoint instead.
pub async fn list_admins_handler(
    State(AppState { dynamo, .. }): State<AppState>,
) -> Result<Json<ListItemsResponse<AdminUserResponse>>, Error> {
    let (admins, _bookmark) =
        User::find_by_user_type(&dynamo.client, UserType::Admin, User::opt_all()).await?;

    Ok(Json(ListItemsResponse {
        items: admins.into_iter().map(|user| user.into()).collect(),
        bookmark: None,
    }))
}
