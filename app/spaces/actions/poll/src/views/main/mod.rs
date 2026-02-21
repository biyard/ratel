use crate::*;
mod creator;
use creator::PollCreatorPage;

mod participant;
use participant::PollParticipantPage;

mod viewer;
use viewer::PollViewerPage;

use participant::*;
use viewer::*;

#[component]
pub fn MainPage(space_id: SpacePartition, poll_id: SpacePollEntityType) -> Element {
    // FIXME: Replace it to real role
    let role = SpaceUserRole::Creator;

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
