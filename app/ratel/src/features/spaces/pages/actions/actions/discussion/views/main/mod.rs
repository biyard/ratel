mod creator;
mod viewer;

use super::*;

use crate::features::spaces::space_common::hooks::use_space_role;
use crate::features::spaces::space_common::types::space_page_actions_discussion_comments_key;
use creator::CreatorMain;
use viewer::ViewerMain;

#[component]
pub fn DiscussionActionPage(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let role = use_space_role()();

    let ctx = Context::init(space_id, discussion_id)?;

    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorMain { space_id, discussion_id }
        },
        _ => {
            rsx! {
                ViewerMain { space_id, discussion_id }
            }
        }
    }
}
