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
pub fn HomePage(
    space_id: SpacePartition,
    extensions: Vec<DashboardExtension>,
) -> Element {
    let role = use_server_future(move || async move { SpaceUserRole::Viewer })?.value();

    rsx! {
        {match role().unwrap_or_default() {
            SpaceUserRole::Creator => rsx! { CreatorPage { space_id, extensions: extensions.clone() } },
            SpaceUserRole::Participant => rsx! { ParticipantPage { space_id, extensions: extensions.clone() } },
            SpaceUserRole::Candidate => rsx! { CandidatePage { space_id, extensions: extensions.clone() } },
            SpaceUserRole::Viewer => rsx! { ViewerPage { space_id, extensions: extensions.clone() } },
        }}
    }
}
