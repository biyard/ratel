use dioxus::prelude::*;

mod admin_page;
mod viewer_page;

use admin_page::*;
use viewer_page::*;

#[component]
pub fn Home(teamname: String) -> Element {
    let is_owner = true;

    if is_owner {
        rsx! { AdminPage { teamname } }
    } else {
        rsx! { ViewerPage { teamname } }
    }
}
