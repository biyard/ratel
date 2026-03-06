use crate::*;
mod creator;
use creator::QuizCreatorPage;

mod participant;
use participant::QuizParticipantPage;

mod viewer;
use viewer::QuizViewerPage;

use participant::*;
use space_common::hooks::use_space_role;
use viewer::*;

#[component]
pub fn MainPage(space_id: SpacePartition, quiz_id: SpaceQuizEntityType) -> Element {
    let role = use_space_role()();

    match role {
        SpaceUserRole::Creator => rsx! {
            QuizCreatorPage { space_id, quiz_id }
        },
        SpaceUserRole::Participant | SpaceUserRole::Candidate => rsx! {
            QuizParticipantPage { space_id, quiz_id }
        },
        SpaceUserRole::Viewer => rsx! {
            QuizViewerPage { space_id, quiz_id }
        },
    }
}
