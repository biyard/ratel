use bdk::prelude::*;

#[api_model(table = consensus_votes)]
pub struct ConsensusVote {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = oracles)]
    pub oracle_id: i64,

    #[api_model(many_to_one = consensus)]
    pub consensus_id: i64,

    #[api_model(type = INTEGER)]
    pub vote_type: ConsensusVoteType,

    pub description: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ConsensusVoteType {
    #[default]
    Approved = 1,
    Rejected = 2,
}
