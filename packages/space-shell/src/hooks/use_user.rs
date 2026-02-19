use crate::controllers::user::get_user;
use common::use_query;
use dioxus::prelude::*;
use ratel_auth::models::user::User;
use std::collections::HashMap;

pub const USER_QUERY_KEY: &str = "User";

#[track_caller]
pub fn use_user(
) -> dioxus::prelude::Result<dioxus_fullstack::Loader<Option<User>>, dioxus_fullstack::Loading> {
    use_query(USER_QUERY_KEY, get_user)
}
