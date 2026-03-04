mod creator;
mod viewer;

use crate::*;
use space_common::hooks::use_user_role;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();

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
