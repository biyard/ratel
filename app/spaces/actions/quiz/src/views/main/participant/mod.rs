mod i18n;
use crate::*;
use i18n::QuizParticipantTranslate;

#[component]
pub fn QuizParticipantPage(space_id: SpacePartition, quiz_id: SpaceQuizEntityType) -> Element {
    rsx! {
        div { "Quiz Participant Page" }
    }
}
