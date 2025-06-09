use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/contracts", table = space_contracts)]
pub struct SpaceContract {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,
    pub name: String,
    pub contract_address: String,
    #[api_model(version = v0.1)]
    pub asset_dir: String,
}
