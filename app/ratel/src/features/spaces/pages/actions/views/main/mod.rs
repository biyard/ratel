use super::*;
mod creator_page;
mod participant_page;
mod viewer_page;

use crate::features::spaces::space_common::hooks::use_space_role;
use creator_page::*;
use participant_page::*;
use viewer_page::*;

#[component]
pub fn SpaceActionsPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let role = use_space_role();

    match role() {
        SpaceUserRole::Creator => rsx! {
            CreatorActionPage { space_id }
        },
        SpaceUserRole::Participant => rsx! {
            ParticipantPage { space_id }
        },

        SpaceUserRole::Candidate | SpaceUserRole::Viewer => rsx! {
            ViewerPage { space_id }
        },
    }
}
