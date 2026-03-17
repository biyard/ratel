mod i18n;

use super::QuizReadPage;
use crate::features::spaces::pages::actions::actions::quiz::*;
pub use i18n::QuizParticipantTranslate;

#[component]
pub fn QuizParticipantPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
) -> Element {
    rsx! {
        QuizReadPage {
            space_id,
            quiz_id,
            can_respond: true,
        }
    }
}
