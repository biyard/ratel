use bdk::prelude::*;

#[api_model(table = dagit_oracles)]
pub struct DagitOracle {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,

    #[api_model(many_to_one = oracles)]
    pub oracle_id: i64,
}
