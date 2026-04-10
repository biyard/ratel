mod attachments;
mod comments_panel;
mod content_body;
mod disc_header;
mod i18n;

pub use attachments::DiscussionAttachments;
pub use comments_panel::CommentsPanel;
pub use content_body::DiscussionContentBody;
pub use disc_header::DiscHeader;
pub use i18n::DiscussionViewerTranslate;

use super::*;
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};

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
    let space = use_space()();

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

    let status = discussion.status();
    let (status_class, status_text) = match status {
        DiscussionStatus::InProgress => ("topbar__status topbar__status--active", tr.status_in_progress),
        DiscussionStatus::NotStarted => ("topbar__status topbar__status--not-started", tr.status_not_started),
        DiscussionStatus::Finish => ("topbar__status topbar__status--finished", tr.status_finished),
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "discussion-arena",
            // Top Bar
            div { class: "topbar",
                div { class: "topbar__left",
                    button {
                        class: "topbar__back",
                        "aria-label": "{tr.back_to_actions}",
                        onclick: move |_| {
                            let nav = use_navigator();
                            nav.go_back();
                        },
                        lucide_dioxus::ArrowLeft { class: "w-[18px] h-[18px]" }
                    }
                    if !space.logo.is_empty() {
                        img {
                            class: "topbar__space-logo",
                            src: "{space.logo}",
                            alt: "{space.title}",
                        }
                    }
                    span { class: "topbar__space-name", "{space.title}" }
                }
                div { class: "topbar__right",
                    span { class: "{status_class}", "{status_text}" }
                }
            }

            // Split Layout
            div { class: "discussion-layout",
                // Left: Discussion Content
                div { class: "discussion-main",
                    div { class: "discussion-main__inner",
                        DiscHeader { discussion: discussion.clone() }

                        DiscussionContentBody { html_contents: discussion.html_contents.clone() }

                        DiscussionAttachments { files: discussion.files.clone() }
                    }
                }

                // Right: Comments Panel
                CommentsPanel {
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
