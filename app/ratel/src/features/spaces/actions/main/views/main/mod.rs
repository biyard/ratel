use crate::features::spaces::actions::main::*;
mod creator_page;
mod participant_page;

use creator_page::*;
use participant_page::*;
use crate::features::spaces::space_common::hooks::use_space_role;

#[component]
pub fn MainPage(space_id: SpacePartition) -> Element {
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
