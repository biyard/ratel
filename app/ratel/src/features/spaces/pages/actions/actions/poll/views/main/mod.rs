mod creator;
use creator::PollCreatorPage;

mod participant;
pub use participant::PollContent;
use participant::PollParticipantPage;

mod viewer;
use super::*;
use viewer::PollViewerPage;

#[component]
pub fn PollActionPage(space_id: SpacePartition, poll_id: SpacePollEntityType) -> Element {
    let role = use_space_role()();
    let space_id = use_signal(|| space_id);
    let poll_id = use_signal(|| poll_id);

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
