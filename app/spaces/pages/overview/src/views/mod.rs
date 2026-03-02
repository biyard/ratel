use crate::*;

mod creator_page;
mod i18n;
mod viewer_page;

use creator_page::*;
use i18n::*;
use viewer_page::*;

use space_common::hooks::use_user_role;

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
