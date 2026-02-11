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
) -> Element {
    // TODO: Fetch the user's role from the database

    // TODO: Fetch dashboard extension data from the database
    let role = SpaceUserRole::Creator;

    rsx! {
        {match role {
            SpaceUserRole::Creator => rsx! {
                    CreatorPage {
                        space_id,
                        extensions: crate::route::get_creator_extensions()
                    }
                },
                SpaceUserRole::Participant => rsx! {
                    ParticipantPage {
                        space_id,
                        extensions: crate::route::get_participant_extensions()
                    }
                },
                SpaceUserRole::Candidate => rsx! {
                    CandidatePage {
                        space_id,
                        extensions: crate::route::get_candidate_extensions()
                    }
                },
                SpaceUserRole::Viewer => rsx! {
                    ViewerPage {
                        space_id,
                        extensions: crate::route::get_viewer_extensions()
                    }
                },
            }}
    }
}
