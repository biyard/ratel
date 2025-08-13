use bdk::prelude::*;

#[api_model(table = oracle_votes)]
pub struct OracleVote {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = oracles)]
    pub oracle_id: i64,

    #[api_model(many_to_one = oracle_consensus)]
    pub consensus_id: i64,

    #[api_model(action = create, type = INTEGER)]
    pub vote: OracleVoteType,

    #[api_model(action = create)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum OracleVoteType {
    #[default]
    Accepted = 1,
    Rejected = 2,
    Abstained = 3,
}
