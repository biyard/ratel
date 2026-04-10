mod attachments;
mod comments_drawer;
mod content_body;
mod i18n;
mod layout;
mod meta_line;
mod table_of_contents;
mod toc_context;

pub use attachments::DiscussionAttachments;
pub use comments_drawer::{CommentsSideDrawer, FloatingCommentsButton, CommentsBottomDrawer, CommentsBottomBar};
pub use content_body::DiscussionContentBody;
pub use i18n::DiscussionViewerTranslate;
pub use layout::NotionLayout;
pub use meta_line::DiscussionMetaLine;
pub use table_of_contents::DiscussionToc;
pub use toc_context::{DiscussionTocContext, TocEntry, use_discussion_toc_context};

use super::*;
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use layout::heading_count;

#[component]
pub fn ViewerMain(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    let role = use_space_role()();
    let user = crate::features::spaces::hooks::use_user()?;
    let current_user_pk = user.read().as_ref().map(|u| u.pk.to_string());
    let ctx = use_discussion_context();
    let space = use_space().read().clone();

    DiscussionTocContext::init();

    let discussion_response = ctx.discussion();
    let discussion = discussion_response.post;
    let can_participate = discussion.status() == DiscussionStatus::InProgress;
    let can_comment = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        discussion_response.space_action.prerequisite,
        space.status,
        space.join_anytime,
    ) && can_participate;
    let can_manage_comments = can_comment;
    let comment_count = use_signal(|| discussion.comments.max(0) as usize);

    let side_drawer_ctx = use_context::<super::SideDrawerOpen>();
    let bottom_drawer_ctx = use_context::<super::BottomDrawerOpen>();
    let side_drawer_open = side_drawer_ctx.0;
    let bottom_drawer_open = bottom_drawer_ctx.0;

    let title = if discussion.title.is_empty() {
        tr.untitled_discussion
    } else {
        &discussion.title
    };

    let has_toc = heading_count(&discussion.html_contents) >= 3;
    let grid_class = if has_toc {
        "grid grid-cols-1 gap-6 mx-auto w-full max-w-[1080px] px-4 py-6 md:px-6 md:py-8 desktop:px-8 desktop:grid-cols-[1fr_200px] desktop:gap-12"
    } else {
        "grid grid-cols-1 gap-6 mx-auto w-full max-w-[1080px] px-4 py-6 md:px-6 md:py-8 desktop:px-8"
    };

    rsx! {
        div { class: "{grid_class}",
            // Left column: content
            div { class: "flex flex-col gap-6 min-w-0",
                h1 { class: "text-2xl font-bold tracking-tight text-text-primary md:text-3xl desktop:text-4xl",
                    "{title}"
                }

                DiscussionMetaLine { discussion: discussion.clone() }

                DiscussionAttachments { files: discussion.files.clone() }

                NotionLayout { html_contents: discussion.html_contents.clone() }
            }

            // Right column: TOC (starts at same height as title)
            if has_toc {
                DiscussionToc {}
            }
        }

        // Desktop (>=800px): floating button + right side drawer
        FloatingCommentsButton { open: side_drawer_open, comment_count }
        CommentsSideDrawer {
            open: side_drawer_open,
            space_id,
            discussion_id,
            can_comment,
            can_manage_comments,
            current_user_pk: current_user_pk.clone(),
            comment_count,
        }

        // Mobile (<800px): bottom drawer + handle bar
        CommentsBottomDrawer {
            open: bottom_drawer_open,
            space_id,
            discussion_id,
            can_comment,
            can_manage_comments,
            current_user_pk,
            comment_count,
        }

        CommentsBottomBar { open: bottom_drawer_open, comment_count }
    }
}
