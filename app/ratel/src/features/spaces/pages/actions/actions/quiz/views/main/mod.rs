use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::components::{ActionEditMode, SettingsSwitchButton};
mod quiz_read_page;
pub use quiz_read_page::QuizReadPage;
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
    let space_id_sig: ReadSignal<SpacePartition> = use_signal(|| space_id.clone()).into();
    let quiz_id_sig: ReadSignal<SpaceQuizEntityType> = use_signal(|| quiz_id.clone()).into();

    // Lightweight standalone fetch to read the quiz's started_at — used
    // only to decide which view to render. The creator/participant pages
    // each set up their own full Context::init independently.
    let key = crate::features::spaces::space_common::types::space_page_actions_quiz_key(
        &space_id_sig(),
        &quiz_id_sig(),
    );
    let quiz_loader = use_query(&key, move || get_quiz(space_id_sig(), quiz_id_sig()))?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        quiz_loader().started_at,
    );

    // Edit-mode override: creators can flip this from the settings
    // button inside the participant view to open the configuration UI
    // even after the action has started.
    let edit_mode = use_context_provider(|| ActionEditMode(Signal::new(false)));
    let show_creator_view = !locked || edit_mode.0();

    let content = match (role, show_creator_view) {
        (SpaceUserRole::Creator, true) => rsx! {
            QuizCreatorPage { space_id, quiz_id }
        },

        (SpaceUserRole::Creator, false)
        | (SpaceUserRole::Participant | SpaceUserRole::Candidate, _) => rsx! {
            QuizParticipantPage { space_id, quiz_id }
        },

        (SpaceUserRole::Viewer, _) => rsx! {
            QuizViewerPage { space_id, quiz_id }
        },
    };

    rsx! {
        div { class: "flex flex-col flex-1 mx-auto w-full min-h-0 max-w-desktop",
            SettingsSwitchButton {}
            {content}
        }
    }
}
