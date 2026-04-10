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

    let ctx = Context::init(space_id, discussion_id)?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        ctx.discussion().space_action.started_at,
    );

    let edit_mode = use_context_provider(|| ActionEditMode(Signal::new(false)));
    let show_creator_view = !locked || edit_mode.0();

    let content = match (role, show_creator_view) {
        (SpaceUserRole::Creator, true) => rsx! {
            CreatorMain { space_id, discussion_id }
        },
        _ => rsx! {
            ViewerMain { space_id, discussion_id }
        },
    };

    let wrapper_class = if show_creator_view {
        "flex flex-col flex-1 mx-auto w-full min-h-0 max-w-desktop"
    } else {
        "flex flex-col flex-1 w-full min-h-0"
    };

    rsx! {
        div { class: "{wrapper_class}",
            if !show_creator_view {
                SettingsSwitchButton {}
            }

            {content}
        }
    }
}
