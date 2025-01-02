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
    id: String,
    r#type: String,
    crated_at: u64,
    updated_at: u64,
    deleted_at: Option<u64>,

    name: Option<String>,

    // Indexes, if deleted_at is set, all values of indexes must be empty.
    gsi1: String,
    gsi2: String,
}
