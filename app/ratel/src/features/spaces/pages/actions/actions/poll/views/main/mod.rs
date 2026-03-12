mod creator;
use creator::PollCreatorPage;

mod participant;
use participant::PollParticipantPage;

mod viewer;
use viewer::PollViewerPage;

use participant::*;
use viewer::*;

use super::*;

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
