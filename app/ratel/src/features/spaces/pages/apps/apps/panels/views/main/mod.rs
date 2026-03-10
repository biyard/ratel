mod creator;
mod viewer;

use crate::features::spaces::pages::apps::apps::panels::*;
use crate::features::spaces::space_common::hooks::use_space_role;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role = use_space_role()();

    if role == SpaceUserRole::Creator {
        rsx! {
            creator::CreatorPage { space_id }
        }
    } else {
        rsx! {
            viewer::ViewerPage { space_id }
        }
    }
}
