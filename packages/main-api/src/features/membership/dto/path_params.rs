use axum::extract::Path;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct MembershipPathParam {
    pub membership_id: String,
}

pub type MembershipPath = Path<MembershipPathParam>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UserIdPathParam {
    pub user_id: String,
}

pub type UserIdPath = Path<UserIdPathParam>;
