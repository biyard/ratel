use crate::features::spaces::pages::actions::actions::discussion::components::DiscussionComments;
use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionViewerTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;

/// Inline split-panel for comments (right side on desktop, stacked on mobile).
#[component]
pub fn CommentsPanel(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    can_comment: bool,
    can_manage_comments: bool,
    current_user_pk: Option<String>,
    comment_count: Signal<usize>,
) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();

    rsx! {
        div { class: "comments-panel",
            div { class: "comments-panel__header",
                span { class: "comments-panel__title", "{tr.comments}" }
                span { class: "comments-panel__count", "{comment_count()}" }
            }
            div { class: "comments-scroll",
                DiscussionComments {
                    space_id,
                    discussion_id,
                    can_comment,
                    can_manage_comments,
                    current_user_pk,
                    comment_count,
                }
            }
        }
    }
}
