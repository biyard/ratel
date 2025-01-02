use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub nickname: String,
    pub profile_url: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserActionRequest {
    Signup(SignupRequest),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct User {
    pub created_at: u64,
    pub updated_at: u64,

    pub nickname: String,
    pub email: String,
    pub profile_url: String,
}
