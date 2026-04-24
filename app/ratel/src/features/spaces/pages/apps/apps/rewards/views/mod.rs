// mod creator;
mod viewer;

use crate::features::spaces::pages::apps::apps::rewards::*;
use crate::features::spaces::space_common::providers::use_space_context;

/// Gated on the real (non-memo) role: the creator/participant check
/// stays stable regardless of the `current_role` preview toggle.
#[component]
pub fn HomePage(space_id: ReadSignal<SpacePartition>) -> Element {
    let mut ctx = use_space_context();
    let real_role = ctx.role();

    if real_role == SpaceUserRole::Creator || real_role == SpaceUserRole::Participant {
        rsx! {
            viewer::ViewerPage { space_id }
        }
    } else {
        rsx! {}
    }
}
