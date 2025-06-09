use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(table = onboards)]
pub struct Onboard {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(indexed)]
    pub meta_id: i64,
}
