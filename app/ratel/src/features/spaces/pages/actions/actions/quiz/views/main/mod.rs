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
use crate::features::spaces::space_common::providers::use_space_context;

#[component]
pub fn QuizActionPage(space_id: SpacePartition, quiz_id: SpaceQuizEntityType) -> Element {
    let role = use_space_role()();
    let space_ctx = use_space_context();
    let is_admin = space_ctx.role().is_admin();
    let space_id_sig: ReadSignal<SpacePartition> = use_signal(|| space_id.clone()).into();
    let quiz_id_sig: ReadSignal<SpaceQuizEntityType> = use_signal(|| quiz_id.clone()).into();

    // Lightweight standalone fetch to read the quiz's started_at — used
    // only to decide which view to render. The creator/participant pages
    // each set up their own full Context::init independently.
    let quiz_loader = use_loader(move || get_quiz(space_id_sig(), quiz_id_sig()))?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        quiz_loader().space_action.status.as_ref(),
    );

    // Edit-mode override: creators can flip this from the settings
    // button inside the participant view to open the configuration UI
    // even after the action has started. Admins always start in edit mode.
    let edit_mode = use_context_provider(|| ActionEditMode(Signal::new(is_admin)));
    let show_creator_view = is_admin || !locked || edit_mode.0();

    let content = match (role, show_creator_view) {
        (SpaceUserRole::Creator, true) => rsx! {
            QuizCreatorPage { space_id, quiz_id }
        },

        _ if is_admin => rsx! {
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
        div { class: if show_creator_view { "flex flex-col flex-1 w-full min-h-0" } else { "flex flex-col flex-1 mx-auto w-full min-h-0 max-w-desktop" },
            if !show_creator_view {
                SettingsSwitchButton {}
            }
            {content}
        }
    }
}
