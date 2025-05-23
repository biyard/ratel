use bdk::prelude::*;
use validator::Validate;

use crate::*;

#[derive(Validate)]
#[api_model(base = "/v1/presidential-candidates", table = presidential_candidates)]
pub struct PresidentialCandidate {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub name: String,
    #[api_model(summary)]
    pub image: String,
    #[api_model(summary, type = INTEGER)]
    pub crypto_stance: CryptoStance,
    #[api_model(summary, type = INTEGER)]
    pub party: Party,

    #[api_model(summary, one_to_many = election_pledges, foreign_key = presidential_candidate_id, nested)]
    pub election_pledges: Vec<ElectionPledge>,

    #[api_model(summary, version = v0.1)]
    pub description: String,

    #[api_model(summary, version = v0.1)]
    pub tags: String,
}
