mod creator;
use creator::PollCreatorPage;

mod participant;
pub use participant::PollContent;
use participant::PollParticipantPage;

mod viewer;
use super::*;
use crate::features::spaces::pages::actions::components::{ActionEditMode, SettingsSwitchButton};
use viewer::PollViewerPage;

#[component]
pub fn PollActionPage(space_id: SpacePartition, poll_id: SpacePollEntityType) -> Element {
    let role = use_space_role()();
    let space_id: ReadSignal<SpacePartition> = use_signal(|| space_id).into();
    let poll_id: ReadSignal<SpacePollEntityType> = use_signal(|| poll_id).into();

    // Read the poll's lifecycle status before deciding which view to
    // render. We do a lightweight standalone fetch here (NOT Context::init)
    // so we don't conflict with the child's own context initialization.
    let key = crate::features::spaces::space_common::types::space_page_actions_poll_key(
        &space_id(),
        &poll_id(),
    );
    let poll_loader = use_query(&key, move || get_poll(space_id(), poll_id()))?;
    let locked = is_action_locked(use_space()().status, poll_loader().started_at);

    // Edit-mode override: creators land on the participant view once
    // the action is locked, but can flip this signal via the
    // `SettingsSwitchButton` (rendered inside the participant view) to
    // temporarily open the creator/configuration page.
    let edit_mode = use_context_provider(|| ActionEditMode(Signal::new(false)));
    let show_creator_view = !locked || edit_mode.0();

    let content = match (role, show_creator_view) {
        // Creator before the action starts (or edit mode on): configuration UI
        (SpaceUserRole::Creator, true) => rsx! {
            PollCreatorPage { space_id, poll_id }
        },

        // Creator after start/end: legacy viewer with submit enabled so they
        // can still respond. Settings button above toggles edit mode.
        (SpaceUserRole::Creator, false) => rsx! {
            PollContent { space_id, poll_id, can_respond: true }
        },

        // Participants and candidates see the new gamified viewer.
        (SpaceUserRole::Participant | SpaceUserRole::Candidate, _) => rsx! {
            PollParticipantPage { space_id, poll_id }
        },

        (SpaceUserRole::Viewer, _) => rsx! {
            PollViewerPage { space_id, poll_id }
        },
    };

    rsx! {
        div { class: if !show_creator_view { "flex flex-col flex-1 w-full h-full min-h-0" } else { "flex flex-col flex-1 mx-auto w-full min-h-0 max-w-desktop" },
            if !show_creator_view {
                SettingsSwitchButton {}
            }
            {content}
        }
    }
}
