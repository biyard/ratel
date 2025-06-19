#![allow(unused_variables)]
use bdk::prelude::*;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users/verification", table = verifications)]
pub struct Verification {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(action = [send_verification_code, verify])]
    #[validate(email)]
    pub email: String,
    #[api_model(action = verify)]
    pub value: String,
    pub expired_at: i64,
    pub attemp_count: i32,
}
