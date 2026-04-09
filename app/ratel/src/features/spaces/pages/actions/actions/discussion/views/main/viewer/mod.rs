use super::*;
use crate::common::components::{Button, ButtonSize, ButtonStyle};
use crate::features::spaces::pages::actions::actions::discussion::components::DiscussionComments;
use crate::features::spaces::pages::actions::components::FullActionLayover;
use crate::features::spaces::pages::actions::gamification::components::quest_briefing::QuestBriefing;
use crate::features::spaces::pages::actions::gamification::hooks::use_quest_briefing;
use crate::features::spaces::pages::actions::gamification::types::{
    QuestNodeStatus, QuestNodeView,
};
use crate::features::spaces::pages::actions::types::SpaceActionType;
use crate::features::spaces::pages::apps::apps::file::components::FileCard;
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
    let (show_briefing, dismiss) = use_quest_briefing();
    let role = use_space_role()();
    let user = crate::features::spaces::hooks::use_user()?;
    let current_user_pk = user.read().as_ref().map(|u| u.pk.to_string());
    let ctx = use_discussion_context();
    let space = use_space().read().clone();
    let discussion_response = ctx.discussion();
    let discussion = discussion_response.post;
    let can_participate = discussion.status() == DiscussionStatus::InProgress;
    let chapter = crate::features::spaces::pages::actions::default_chapter_for_legacy_action(role);
    let can_comment = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        &chapter,
        true,
        true,
        space.status,
        space.join_anytime,
    ) && can_participate;
    let can_manage_comments = can_comment;
    let nav = navigator();

    if show_briefing {
        let node = QuestNodeView {
            id: discussion_id().to_string(),
            action_type: SpaceActionType::TopicDiscussion,
            title: discussion.title.clone(),
            base_points: 0,
            projected_xp: 0,
            status: QuestNodeStatus::Active,
            depends_on: vec![],
            chapter_id: String::new(),
            started_at: None,
            ended_at: None,
            quiz_result: None,
        };
        rsx! {
            QuestBriefing {
                node,
                on_begin: move |_| dismiss.call(()),
                on_cancel: move |_| {
                    nav.go_back();
                },
            }
        }
    } else {
        rsx! {
            FullActionLayover {
                content_class: "gap-5".to_string(),
                bottom_right: rsx! {
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "min-w-[120px]",
                        onclick: move |_| {
                            nav.push(format!("/spaces/{}/actions", space_id()));
                        },
                        {tr.back}
                    }
                },
                div { class: "w-full",
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
            if !discussion.files.is_empty() {
                div { class: "grid grid-cols-4 gap-2.5 max-desktop:grid-cols-3 max-tablet:grid-cols-2 max-mobile:grid-cols-1",
                    for file in discussion.files.iter() {
                        FileCard {
                            key: "{file.id}",
                            file: file.clone(),
                            editable: false,
                            on_delete: None,
                        }
                    }
                }
            }
            hr { class: "border-divider" }
        }
    }
}
