mod creator;
mod viewer;

use super::*;

use crate::features::spaces::pages::actions::components::{ActionEditMode, SettingsSwitchButton};
use crate::features::spaces::space_common::hooks::use_space_role;
use creator::CreatorMain;
use viewer::ViewerMain;

#[component]
pub fn DiscussionActionPage(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let role = use_space_role()();
    let space_ctx = crate::features::spaces::space_common::providers::use_space_context();
    let is_admin = space_ctx.role().is_admin();

    // `Context::init` already loads the discussion (and provides the
    // context to children). Use its data to detect whether the action
    // has started — past start, the creator switches into the viewer
    // (participant) experience.
    let ctx = Context::init(space_id, discussion_id)?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        ctx.discussion().space_action.status.as_ref(),
    );

    // Edit-mode override: creators can flip this from the settings
    // button inside the participant view to open the configuration UI
    // even after the action has started. Admins always start in edit mode.
    let edit_mode = use_context_provider(|| ActionEditMode(Signal::new(is_admin)));
    let show_creator_view = is_admin || !locked || edit_mode.0();

    let content = match (role, show_creator_view) {
        (SpaceUserRole::Creator, true) => rsx! {
            CreatorMain { space_id, discussion_id }
        },
        // Admin in any role view → always show creator UI.
        _ if is_admin => rsx! {
            CreatorMain { space_id, discussion_id }
        },
        // Default: creators after start, and all others (participants,
        // candidates, viewers), see the shared viewer experience.
        _ => rsx! {
            ViewerMain { space_id, discussion_id }
        },
    };

    rsx! {
        div { class: "flex flex-col flex-1 mx-auto w-full min-h-0 max-w-desktop",
            if !show_creator_view {
                SettingsSwitchButton {}
            }

            {content}
        }
    }
}
