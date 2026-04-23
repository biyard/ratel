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
pub fn FollowActionPage(
    space_id: SpacePartition,
    follow_id: SpaceActionFollowEntityType,
) -> Element {
    let role = use_space_role()();
    let space_ctx = crate::features::spaces::space_common::providers::use_space_context();
    let is_admin = space_ctx.role().is_admin();
    let space_id_sig: ReadSignal<SpacePartition> = use_signal(|| space_id.clone()).into();
    let follow_id_sig: ReadSignal<SpaceActionFollowEntityType> =
        use_signal(|| follow_id.clone()).into();

    // Lightweight fetch of the underlying SpaceAction so we can decide
    // whether the follow action has started. After it starts the creator
    // should switch into the participant (viewer) view.
    let action_loader =
        use_loader(move || async move { get_follow(space_id_sig(), follow_id_sig()).await })?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        action_loader().status.as_ref(),
    );

    // Edit-mode override: creators can flip this from the settings
    // button inside the participant view to open the configuration UI
    // even after the action has started. Admins always start in edit mode.
    let edit_mode = use_context_provider(|| ActionEditMode(Signal::new(is_admin)));
    let show_creator_view = is_admin || !locked || edit_mode.0();

    let content = match (role, show_creator_view) {
        (SpaceUserRole::Creator, true) => rsx! {
            FollowCreatorPage { space_id: space_id_sig, follow_id: follow_id_sig }
        },
        // Admin in any role view → always show creator UI.
        _ if is_admin => rsx! {
            FollowCreatorPage { space_id: space_id_sig, follow_id: follow_id_sig }
        },
        // Default: creators after start, and all others, see the
        // viewer/participant experience.
        _ => rsx! {
            FollowViewerPage { space_id, follow_id }
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
