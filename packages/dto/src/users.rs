use std::fmt::Display;

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct SignupRequest {
    pub email: String,
    pub nickname: String,
    pub profile_url: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum UserActionRequest {
    Signup(SignupRequest),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct User {
    pub created_at: u64,
    pub updated_at: u64,

    pub nickname: String,
    pub email: String,
    pub profile_url: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum ReadActionType {
    CheckEmail,
    UserInfo,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct UserReadActionRequest {
    pub action: ReadActionType,
    pub email: Option<String>,
}

impl Display for UserReadActionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = serde_urlencoded::to_string(&self).unwrap();

        write!(f, "{query}")
    }
}
