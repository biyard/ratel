use crate::*;

mod creator_page;
mod i18n;
mod viewer_page;

use creator_page::*;
use i18n::*;
use viewer_page::*;

use space_common::hooks::use_space_role;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role = use_space_role()();

    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorPage { space_id }
        },
        _ => rsx! {
            ViewerPage { space_id }
        },
    }
}
