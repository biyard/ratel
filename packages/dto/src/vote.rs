#![allow(unused_variables, unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::*;
use by_types::QueryResponse;
use validator::ValidationError;

// If you want to know how to use Y macro, refer to https://github.com/biyard/rust-sdk/tree/main/packages/by-macros
#[api_model(base = "/v1/topics/:topic-id/votes", table = votes, iter_type=QueryResponse)]
pub struct Vote {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, Type = INTEGER, action = voting)]
    pub vote: VoteResult,

    #[api_model(summary, action = voting)]
    pub amount: i64,

    #[api_model(many_to_one = users, query_action = list_by_user_id)]
    pub user_id: i64,

    #[api_model(many_to_one = topics)]
    pub topic_id: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum VoteResult {
    #[default]
    Neutral = 0,
    Supportive = 1,
    Against = 2,
}
