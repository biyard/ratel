use crate::features::spaces::pages::actions::actions::quiz::*;
mod creator;
use creator::QuizCreatorPage;

mod participant;
use participant::QuizParticipantPage;

mod viewer;
use viewer::QuizViewerPage;

use crate::features::spaces::space_common::hooks::use_space_role;

#[component]
pub fn QuizActionPage(space_id: SpacePartition, quiz_id: SpaceQuizEntityType) -> Element {
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
