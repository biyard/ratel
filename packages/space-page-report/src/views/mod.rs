use crate::*;

mod candidate_page;
mod creator_page;
mod participant_page;
mod viewer_page;

use candidate_page::*;
use creator_page::*;
use participant_page::*;
use viewer_page::*;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role = use_server_future(move || async move { SpaceUserRole::Viewer })?.value();

    match role().unwrap_or_default() {
        SpaceUserRole::Creator => rsx! { CreatorPage { space_id } },
        SpaceUserRole::Participant => rsx! { ParticipantPage { space_id } },
        SpaceUserRole::Candidate => rsx! { CandidatePage { space_id } },
        SpaceUserRole::Viewer => rsx! { ViewerPage { space_id } },
    }
}
