use bdk::prelude::*;

use crate::ConsensusVoteType;

#[api_model(table = artwork_certifications)]
pub struct ArtworkCertification {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = artworks)]
    pub artwork_id: i64,

    #[api_model(many_to_one = consensus)]
    pub consensus_id: i64,

    pub total_oracles: i64,
    pub total_votes: i64,
    pub approved_votes: i64,
    pub rejected_votes: i64,

    // Voter details stored as JSON
    #[api_model(type = JSONB)]
    pub voters: Vec<CertificationVoter>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, JsonSchema)]
pub struct CertificationVoter {
    pub nickname: String,
    pub vote_type: ConsensusVoteType,
    pub description: Option<String>,
}
