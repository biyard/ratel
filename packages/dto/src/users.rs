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
    #[validate(custom(function = "validate_nickname"))]
    pub nickname: String,
    #[api_model(unique, read_action = by_principal)]
    pub principal: String,
    #[api_model(action = signup, read_action = [check_email, login], unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = signup, nullable)]
    #[validate(url)]
    pub profile_url: String,
}

fn validate_nickname(nickname: &str) -> std::result::Result<(), ValidationError> {
    lazy_static! {
        static ref NICKNAME_REGEX: regex::Regex =
            regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-_]{1,20}$").unwrap();
    }

    if !NICKNAME_REGEX.is_match(nickname) {
        return Err(ValidationError::new(
            "Nickname must be started with alphabet or number and only allow alphabet, number, hyphen and underscore, maximum 20 characters",
        ));
    }

    Ok(())
}
