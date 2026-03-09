use crate::features::spaces::controllers::user::get_user;
use crate::common::use_query;
use dioxus::prelude::*;
use crate::features::auth::models::user::User;
use std::collections::HashMap;

pub const USER_QUERY_KEY: &[&str] = &["User"];

#[track_caller]
pub fn use_user(
) -> dioxus::prelude::Result<dioxus_fullstack::Loader<Option<User>>, dioxus_fullstack::Loading> {
    use_query(USER_QUERY_KEY, get_user)
}
