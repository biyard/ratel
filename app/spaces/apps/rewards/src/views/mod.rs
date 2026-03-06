mod creator;
mod viewer;

use crate::*;
use space_common::hooks::use_space_role;

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
