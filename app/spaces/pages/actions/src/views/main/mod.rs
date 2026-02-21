use crate::*;
mod creator_page;
mod participant_page;

use creator_page::*;
use participant_page::*;

#[component]
pub fn MainPage(space_id: SpacePartition) -> Element {
    // FIXME: Replace it to real role
    let role = SpaceUserRole::Creator;

    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorActionPage { space_id }
        },
        SpaceUserRole::Participant | SpaceUserRole::Candidate | SpaceUserRole::Viewer => rsx! {
            ParticipantActionPage { space_id }
        },
    }
}
