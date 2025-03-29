#![allow(unused)]
use crate::*;
use by_types::QueryResponse;

use bdk::prelude::*;
use lazy_static::lazy_static;
use validator::ValidationError;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", read_action = user_info, table = users, iter_type=QueryResponse)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = signup)]
    pub nickname: String,
    #[api_model(unique, read_action = by_principal)]
    pub principal: String,
    #[api_model(action = signup, read_action = [check_email, login], unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = signup, nullable)]
    #[validate(url)]
    pub profile_url: String,

    #[api_model(action = signup)]
    pub term_agreed: bool, // TODO: make it required (prod table schema)
    #[api_model(action = signup)]
    pub informed_agreed: bool, // TODO: add it prod table schema
}
