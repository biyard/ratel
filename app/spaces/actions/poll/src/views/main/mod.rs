use crate::*;
mod creator;
use creator::PollCreatorPage;

mod participant;
use participant::PollParticipantPage;

mod viewer;
use viewer::PollViewerPage;

use participant::*;
use space_common::hooks::use_user_role;
use viewer::*;

#[component]
pub fn MainPage(space_id: SpacePartition, poll_id: SpacePollEntityType) -> Element {
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();

    match role {
        SpaceUserRole::Creator => rsx! {
            PollCreatorPage { space_id, poll_id }
        },
        SpaceUserRole::Participant | SpaceUserRole::Candidate => rsx! {
            PollParticipantPage { space_id, poll_id }
        },
        SpaceUserRole::Viewer => rsx! {
            PollViewerPage { space_id, poll_id }
        },
    }
}
