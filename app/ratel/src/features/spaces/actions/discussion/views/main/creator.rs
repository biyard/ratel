use crate::features::spaces::actions::discussion::*;
use crate::features::spaces::space_common::types::{route::space_action_discussion_edit, space_page_actions_discussion_key};

use super::viewer::{DiscussionComments, DiscussionContent};

#[component]
pub fn CreatorMain(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
    discussion: SpacePost,
    comments: Vec<DiscussionCommentResponse>,
) -> Element {
    let nav = navigator();

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            div { class: "flex justify-between items-center",
                button {
                    class: "flex items-center gap-2 text-sm text-neutral-400 hover:text-white light:text-neutral-600 light:hover:text-neutral-900 transition-colors",
                    onclick: move |_| { nav.go_back(); },
                    "← Back"
                }
                button {
                    class: "px-4 py-2 rounded-lg border border-neutral-600 text-neutral-300 text-sm hover:bg-neutral-800 light:border-neutral-300 light:text-neutral-700 light:hover:bg-neutral-100",
                    onclick: {
                        let space_id = space_id.clone();
                        let discussion_id = discussion_id.clone();
                        let nav = nav.clone();
                        move |_| {
                            nav.push(space_action_discussion_edit(&space_id, &discussion_id));
                        }
                    },
                    "Edit"
                }
            }
            DiscussionContent { discussion: discussion.clone() }
            DiscussionComments {
                space_id: space_id.clone(),
                discussion_id: discussion_id.clone(),
                discussion,
                comments,
                can_comment: true,
                is_creator: true,
            }
        }
    }
}
