use bdk::prelude::*;

use crate::*;
#[api_model(table = consensus)]
pub struct Consensus {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,

    #[api_model(many_to_one = artworks)]
    pub artwork_id: i64,

    pub total_oracles: i64,

    #[api_model(one_to_many = consensus_votes, foreign_key = consensus_id, nested)]
    pub votes: Vec<ConsensusVote>,

    #[api_model(type = INTEGER, nullable)]
    pub result: Option<ConsensusResult>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ConsensusResult {
    #[default]
    Accepted = 1,
    Rejected = 2,
}
