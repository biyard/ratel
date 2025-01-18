use crate::*;
use by_macros::api_model;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

// #[derive(Debug, Serialize, PartialEq, Eq, Clone, Deserialize)]
// #[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// pub struct SignupRequest {
//     pub email: String,
//     pub nickname: String,
//     pub profile_url: String,
// }

// #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// #[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// pub enum UserActionRequest {
//     Signup(SignupRequest),
// }

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[api_model(base = "/users/v1")]
pub struct User {
    pub created_at: u64,
    pub updated_at: u64,

    #[api_model(action = signup)]
    pub nickname: String,
    #[api_model(action = signup, read_action = [check_email, user_info])]
    pub email: String,
    #[api_model(action = signup)]
    pub profile_url: String,
}

// #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "kebab-case")]
// #[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// pub enum ReadActionType {
//     CheckEmail,
//     UserInfo,
// }

// #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, QueryDisplay)]
// #[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// pub struct UserReadActionRequest {
//     pub action: ReadActionType,
//     pub email: Option<String>,
// }
