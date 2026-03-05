mod creator_page;
mod viewer_page;

use crate::{api::fetch_dashboard_extensions, *};
use creator_page::*;
use space_common::hooks::use_user_role;
use viewer_page::*;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();

    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorPage { space_id }
        },
        _ => rsx! {
            ViewerPage { space_id }
        },
    }
}
