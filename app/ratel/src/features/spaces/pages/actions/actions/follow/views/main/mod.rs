use super::*;
mod creator;
use creator::FollowCreatorPage;

mod viewer;
use viewer::FollowViewerPage;

use crate::features::spaces::pages::actions::actions::follow::controllers::get_follow;
use crate::features::spaces::pages::actions::components::{ActionEditMode, SettingsSwitchButton};
use crate::features::spaces::space_common::hooks::use_space_role;
use viewer::*;

#[component]
pub fn FollowActionPage(space_id: SpacePartition, follow_id: SpaceActionFollowEntityType) -> Element {
    let role = use_space_role()();
    let space_id_sig: ReadSignal<SpacePartition> = use_signal(|| space_id.clone()).into();
    let follow_id_sig: ReadSignal<SpaceActionFollowEntityType> =
        use_signal(|| follow_id.clone()).into();

    // Lightweight fetch of the underlying SpaceAction so we can decide
    // whether the follow action has started. After it starts the creator
    // should switch into the participant (viewer) view.
    let action_loader = use_loader(move || async move {
        get_follow(space_id_sig(), follow_id_sig()).await
    })?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        action_loader().started_at,
    );

    // Edit-mode override: creators can flip this from the settings
    // button inside the participant view to open the configuration UI
    // even after the action has started.
    let edit_mode = use_context_provider(|| ActionEditMode(Signal::new(false)));
    let show_creator_view = !locked || edit_mode.0();

    let content = match (role, show_creator_view) {
        (SpaceUserRole::Creator, true) => rsx! {
            FollowCreatorPage { space_id, follow_id }
        },
        // Default: creators after start, and all others, see the
        // viewer/participant experience.
        _ => rsx! {
            FollowViewerPage { space_id, follow_id }
        },
    };

    rsx! {
        div { class: "flex flex-col flex-1 mx-auto w-full min-h-0 max-w-desktop",
            SettingsSwitchButton {}
            {content}
        }
    }
}
