use super::*;
use crate::features::spaces::pages::actions::*;
mod creator_page;
mod participant_page;

use crate::features::spaces::space_common::hooks::use_space_role;
use creator_page::*;
use participant_page::*;

#[component]
pub fn SpaceActionsPage(space_id: SpacePartition) -> Element {
    let role = use_space_role()();

    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorActionPage { space_id }
        },
        SpaceUserRole::Participant | SpaceUserRole::Candidate | SpaceUserRole::Viewer => rsx! {
            ParticipantActionPage { space_id }
        },
    }
}
