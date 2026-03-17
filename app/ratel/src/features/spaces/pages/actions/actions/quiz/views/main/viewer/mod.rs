use super::QuizReadPage;
use crate::features::spaces::pages::actions::actions::quiz::*;

#[component]
pub fn QuizViewerPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
) -> Element {
    rsx! {
        QuizReadPage {
            space_id,
            quiz_id,
            can_respond: false,
        }
    }
}
