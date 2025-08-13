use bdk::prelude::*;

use crate::*;

#[api_model(table = oracle_consensus)]
pub struct OracleConsensus {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = dagits)]
    pub dagit_id: i64,

    #[api_model(many_to_one = artworks)]
    pub artwork_id: i64,

    #[api_model(one_to_many = oracle_votes, foreign_key = consensus_id, nested)]
    pub votes: Vec<OracleVote>,

    #[api_model(action = update, type = INTEGER)]
    pub result: OracleConsensusResult,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum OracleConsensusResult {
    #[default]
    Ongoing = 1,
    Accepted = 2,
    Rejected = 3,
}
