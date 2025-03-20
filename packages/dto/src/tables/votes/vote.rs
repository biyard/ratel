#![allow(unused)]
use bdk::prelude::*;
use by_types::QueryResponse;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/bills/:bill_id/votes", table = votes, iter_type = QueryResponse)]
pub struct Vote {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, type = INTEGER, action = voting)]
    pub selected: VoteOption,

    #[api_model(summary, many_to_one = bills)]
    pub bill_id: i64,

    // #[api_model(summary, many_to_one = assembly_members, action = voting, query_action = list_votes_by_member)]
    // pub member_id: i64,
    #[api_model(summary, many_to_one = users, action = voting, read_action = get_my_result)]
    pub user_id: i64,
}

#[derive(
    Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, dioxus_translate::Translate, Copy,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum VoteOption {
    #[default]
    #[translate(ko = "긍정", en = "Supportive")]
    Supportive = 1,
    #[translate(ko = "부정", en = "Against")]
    Against = 2,
}
