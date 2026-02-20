use dioxus::prelude::*;
use ratel_auth::models::user::UserType;

mod admin_page;
mod viewer_page;

use admin_page::*;
use viewer_page::*;

#[component]
pub fn Home(username: String) -> Element {
    let user_type = UserType::Admin;

    match user_type {
        UserType::Admin => rsx! {
            AdminPage { username }
        },
        _ => rsx! {
            ViewerPage { username }
        },
    }
}
