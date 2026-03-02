use crate::*;
mod creator_page;
mod participant_page;

use creator_page::*;
use participant_page::*;
use space_common::hooks::use_user_role;

#[component]
pub fn MainPage(space_id: SpacePartition) -> Element {
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();

    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorActionPage { space_id }
        },
        SpaceUserRole::Participant | SpaceUserRole::Candidate | SpaceUserRole::Viewer => rsx! {
            ParticipantActionPage { space_id }
        },
    }
}
