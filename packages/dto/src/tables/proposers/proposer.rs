use bdk::prelude::*;
use by_types::QueryResponse;

#[api_model(base = "/v1/proposers", table = proposers, iter_type = QueryResponse)]
pub struct Proposer {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,

    #[api_model(many_to_one = assembly_members)]
    pub member_id: i64,
    #[api_model(many_to_one = bills)]
    pub bill_id: i64,
    #[api_model(summary)]
    pub is_representative: bool,
}
