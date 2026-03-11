use crate::features::spaces::pages::actions::actions::poll::*;
mod creator;
use creator::PollCreatorPage;

mod participant;
use participant::PollParticipantPage;

mod viewer;
use viewer::PollViewerPage;

use crate::features::spaces::space_common::hooks::use_space_role;
use participant::*;
use viewer::*;

#[component]
pub fn PollActionPage(space_id: SpacePartition, poll_id: SpacePollEntityType) -> Element {
    let role = use_space_role()();

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
