use bdk::prelude::*;
use validator::Validate;

use crate::{CryptoStance, Party, *};

#[derive(Validate)]
#[api_model(base = "/v1/president-candidates", table = president_candidates)]
pub struct PresidentCandidate {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub name: String,
    #[api_model(summary)]
    pub crypto_stance: CryptoStance,
    #[api_model(summary)]
    pub party: Party,

    #[api_model(summary, one_to_many = election_pledges, nested)]
    pub election_pledges: Vec<ElectionPledge>,
}
