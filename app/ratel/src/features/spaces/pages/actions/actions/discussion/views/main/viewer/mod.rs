use super::*;
use crate::common::components::{Button, ButtonSize, ButtonStyle};
use crate::features::spaces::pages::actions::actions::discussion::components::DiscussionComments;
use crate::features::spaces::space_common::hooks::use_space;

translate! {
    DiscussionViewerTranslate;

    back: { en: "Back", ko: "뒤로" },
    untitled_discussion: { en: "Untitled Discussion", ko: "제목 없는 토론" },
    comments: { en: "Comments", ko: "댓글" },
    write_comment: { en: "Write a comment...", ko: "댓글을 입력하세요..." },
    edited: { en: "(Edited)", ko: "(수정)" },
    edit: { en: "Edit", ko: "수정" },
    delete: { en: "Delete", ko: "삭제" },
    cancel: { en: "Cancel", ko: "취소" },
    complete_edit: { en: "Save", ko: "수정 완료" },
    reply: { en: "Reply", ko: "답글" },
    write_reply: { en: "Write a reply...", ko: "답글을 입력하세요..." },
    responses: { en: "responses", ko: "응답" },
}

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
    let discussion = ctx.discussion().post;
    let is_space_active = matches!(
        space.status,
        Some(crate::common::SpaceStatus::Started | crate::common::SpaceStatus::InProgress)
    );
    let can_participate = discussion.status() == DiscussionStatus::InProgress;
    let can_comment =
        matches!(role, SpaceUserRole::Creator | SpaceUserRole::Participant)
            && is_space_active
            && can_participate;
    let can_manage_comments = can_comment;

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            div { class: "flex justify-between items-center",
                Button {
                    size: ButtonSize::Inline,
                    style: ButtonStyle::Text,
                    class: "inline-flex items-center gap-2 text-sm text-text-secondary hover:text-text-primary"
                        .to_string(),
                    onclick: move |_| {
                        navigator().go_back();
                    },
                    "← {tr.back}"
                }
            }
            DiscussionContent { discussion: discussion.clone() }
            DiscussionComments {
                space_id,
                discussion_id,
                can_comment,
                can_manage_comments,
                current_user_pk,
            }
        }
    }
}

#[component]
pub fn DiscussionContent(discussion: SpacePost) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    rsx! {
        div { class: "flex flex-col gap-5",
            h1 { class: "text-2xl font-bold text-text-primary",
                if discussion.title.is_empty() {
                    "{tr.untitled_discussion}"
                } else {
                    "{discussion.title}"
                }
            }
            div { class: "flex gap-3 items-center text-sm text-text-secondary",
                if !discussion.author_profile_url.is_empty() {
                    img {
                        class: "w-6 h-6 rounded-full",
                        src: "{discussion.author_profile_url}",
                    }
                }
                span { class: "font-medium", "{discussion.author_display_name}" }
                if !discussion.category_name.is_empty() {
                    span { class: "py-0.5 px-2 text-xs rounded bg-card text-text-secondary",
                        "{discussion.category_name}"
                    }
                }
            }
            if !discussion.html_contents.is_empty() {
                div {
                    class: "max-w-none prose prose-invert light:prose text-text-primary",
                    dangerous_inner_html: "{discussion.html_contents}",
                }
            }
            hr { class: "border-divider" }
        }
    }
}
