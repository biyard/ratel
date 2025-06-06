use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/space-contracts", table = space_contracts)]
pub struct SpaceContract {
    #[api_model(summary, primary_key, read_action = [find_by_id])]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = spaces)]
    pub space_id: i64,
    #[api_model(summary)]
    pub creator: String,
    #[api_model(summary)]
    pub contract_address: String,
}
