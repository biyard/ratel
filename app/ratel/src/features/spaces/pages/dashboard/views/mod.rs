mod creator_page;
mod viewer_page;

use crate::features::spaces::pages::dashboard::*;
use creator_page::*;
use crate::features::spaces::space_common::hooks::use_space_role;
use viewer_page::*;

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
