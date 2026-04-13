use crate::features::spaces::controllers::user::get_user;
use dioxus::prelude::*;
use dioxus::fullstack::{Loader, Loading};
use crate::features::auth::models::user::User;

#[track_caller]
pub fn use_user() -> dioxus::prelude::Result<Loader<Option<User>>, Loading> {
    use_loader(get_user)
}
