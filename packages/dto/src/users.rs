use crate::*;
use by_macros::api_model;

#[cfg(feature = "server")]
use by_axum::aide;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[api_model(base = "/users/v1", read_action = user_info)]
pub struct User {
    pub created_at: u64,
    pub updated_at: u64,

    #[api_model(action = signup)]
    pub nickname: String,
    #[api_model(action = signup, read_action = check_email)]
    pub email: String,
    #[api_model(action = signup)]
    pub profile_url: String,
}
