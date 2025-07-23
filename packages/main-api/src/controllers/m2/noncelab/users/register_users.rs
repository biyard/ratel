use bdk::prelude::*;
use by_axum::axum::{Json, extract::Path};
use dto::Result;

pub async fn register_users_by_noncelab_handler(
    Path(PathParams { user_id }): Path<PathParams>,
    Json(req): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>> {
    tracing::debug!("Registering user with ID: {}, request: {:?}", user_id, req);
    // TODO: implement logic and verify noncelab API token

    Ok(Default::default())
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct RegisterUserRequest {
    #[schemars(description = "User's display name shown publicly")]
    pub display_name: String,

    #[schemars(description = "Optional unique username (can be null)")]
    pub username: Option<String>,

    #[schemars(description = "Principal of ICP (Internet Computer Protocol)")]
    pub principal: String,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct RegisterUserResponse {
    #[schemars(description = "User ID in Ratel")]
    pub user_id: i64,
    #[schemars(description = "Principal of ICP (Internet Computer Protocol)")]
    pub principal: String,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PathParams {
    pub user_id: i64,
}
