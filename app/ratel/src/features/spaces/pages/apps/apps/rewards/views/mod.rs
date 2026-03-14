// mod creator;
mod viewer;

use crate::features::spaces::pages::apps::apps::rewards::*;
use crate::features::spaces::space_common::hooks::use_space_role;

#[component]
pub fn HomePage(space_id: ReadSignal<SpacePartition>) -> Element {
    let role = use_space_role()();

    if role == SpaceUserRole::Creator || role == SpaceUserRole::Participant {
        rsx! {
            viewer::ViewerPage { space_id }
        }
    } else {
        rsx! {}
    }
}
