mod creator;
pub(crate) mod viewer;

use super::*;

use crate::features::spaces::space_common::hooks::use_space_role;
use crate::features::spaces::space_common::types::{
    space_page_actions_discussion_comments_key, space_page_actions_discussion_key,
};
use creator::CreatorMain;
use viewer::ViewerMain;

#[component]
pub fn DiscussionActionPage(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
) -> Element {
    let role = use_space_role()();
    let key = space_page_actions_discussion_key(&space_id, &discussion_id);
    let discussion_loader = use_query(&key, {
        let space_id = space_id.clone();
        let discussion_id = discussion_id.clone();
        move || get_discussion(space_id.clone(), discussion_id.clone())
    })?;

    let discussion = discussion_loader.read().clone();

    let comments_key = space_page_actions_discussion_comments_key(&space_id, &discussion_id);
    let comments_loader = use_query(&comments_key, {
        let space_id = space_id.clone();
        let discussion_id = discussion_id.clone();
        move || list_comments(space_id.clone(), discussion_id.clone(), None)
    })?;

    let comments = comments_loader.read().clone();

    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorMain { space_id, discussion_id }
        },
        _ => {
            let can_comment = matches!(role, SpaceUserRole::Creator | SpaceUserRole::Participant);

            rsx! {
                ViewerMain {
                    space_id,
                    discussion_id,
                    discussion,
                    comments,
                    can_comment,
                }
            }
        }
    }
}
