mod creator;
mod viewer;

use super::*;

use crate::features::spaces::pages::actions::components::{ActionEditMode, SettingsSwitchButton};
use crate::features::spaces::space_common::hooks::use_space_role;
use crate::features::spaces::space_common::types::space_page_actions_discussion_comments_key;
use creator::CreatorMain;
use viewer::ViewerMain;

/// Wrapper types so two `Signal<bool>` can coexist in context without ambiguity.
#[derive(Clone, Copy)]
pub struct SideDrawerOpen(pub Signal<bool>);

#[derive(Clone, Copy)]
pub struct BottomDrawerOpen(pub Signal<bool>);

#[component]
pub fn DiscussionActionPage(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    // Drawer signals live ABOVE the suspend point — they survive re-mounts.
    use_context_provider(|| SideDrawerOpen(Signal::new(false)));
    use_context_provider(|| BottomDrawerOpen(Signal::new(false)));

    let role = use_space_role()();

    // `Context::init` already loads the discussion (and provides the
    // context to children). Use its data to detect whether the action
    // has started — past start, the creator switches into the viewer
    // (participant) experience.
    let ctx = Context::init(space_id, discussion_id)?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        ctx.discussion().space_action.started_at,
    );

    // Edit-mode override: creators can flip this from the settings
    // button inside the participant view to open the configuration UI
    // even after the action has started.
    let edit_mode = use_context_provider(|| ActionEditMode(Signal::new(false)));
    let show_creator_view = !locked || edit_mode.0();

    let content = match (role, show_creator_view) {
        (SpaceUserRole::Creator, true) => rsx! {
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
