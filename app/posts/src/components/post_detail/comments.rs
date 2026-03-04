use super::utils::time_ago;
use super::PostDetailTranslate;
use crate::controllers::comments::add_comment::add_comment_handler;
use crate::controllers::comments::like_comment::like_comment_handler;
use crate::controllers::comments::reply_to_comment::reply_to_comment_handler;
use crate::controllers::dto::*;
use crate::*;
use common::components::TiptapEditor;
use common::components::{Button, ButtonStyle};
use dioxus::prelude::*;

#[component]
pub fn CommentSection(detail: PostDetailResponse, post_pk: String) -> Element {
    let t: PostDetailTranslate = use_translate();
    let comment_count = detail.post.as_ref().map(|p| p.comments).unwrap_or(0);
    let mut expand_comment = use_signal(|| false);
    let mut comment_text = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);

    let comments: Vec<PostCommentResponse> = detail
        .comments
        .iter()
        .filter(|c| c.parent_comment_sk.is_none())
        .cloned()
        .collect();

    let reply_comments: Vec<PostCommentResponse> = detail
        .comments
        .iter()
        .filter(|c| c.parent_comment_sk.is_some())
        .cloned()
        .collect();

    let reply_label = t.reply;
    let replies_label = t.replies;
    let count_text = use_memo(move || {
        let label = if comment_count == 1 {
            reply_label
        } else {
            replies_label
        };
        format!("{} {}", comment_count, label)
    });

    rsx! {
        div { id: "comments", class: "flex flex-col gap-2.5",
            div { class: "flex flex-row text-text-primary gap-2",
                icons::chat::SquareChat { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                span { class: "text-base/6 font-medium", {count_text()} }
            }
            if !expand_comment() {
                button {
                    class: "flex flex-row w-full px-3.5 py-3 gap-2 bg-write-comment-box-b border border-write-comment-box-border items-center rounded-lg hover-bg-write-comment-box-bg/80 hover:border-primary/50 transition-all duration-200 cursor-pointer group",
                    onclick: move |_| {
                        expand_comment.set(true);
                    },
                    span { class: "inline-flex items-center gap-2",
                        icons::chat::SquareChat { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        span { class: "text-write-comment-box-text text-[15px]/[24px] font-medium",
                            {t.share_your_thoughts}
                        }
                    }
                }
            }
            if expand_comment() {
                div { class: "flex flex-col gap-2 p-4 border border-card-enable-border rounded-lg bg-card-bg-secondary",
                    textarea {
                        class: "w-full min-h-[80px] p-2 bg-transparent text-text-primary border border-divider rounded resize-none focus:outline-none focus:border-primary",
                        placeholder: t.share_your_thoughts,
                        value: comment_text(),
                        oninput: move |e| {
                            comment_text.set(e.value());
                        },
                    }
                    div { class: "flex flex-row justify-end gap-2",
                        Button {
                            style: ButtonStyle::Outline,
                            onclick: move |_| {
                                expand_comment.set(false);
                                comment_text.set(String::new());
                            },
                            {t.cancel}
                        }
                        Button {
                            style: ButtonStyle::Primary,
                            disabled: comment_text.read().trim().is_empty() || *is_submitting.read(),
                            onclick: {
                                let pk = post_pk.clone();
                                move |_| {
                                    let content = comment_text.read().clone();
                                    if content.trim().is_empty() || *is_submitting.read() {
                                        return;
                                    }
                                    is_submitting.set(true);
                                    let pk = pk.clone();
                                    spawn(async move {
                                        let _ = add_comment_handler(pk.parse().unwrap(), content).await;
                                        comment_text.set(String::new());
                                        expand_comment.set(false);
                                        is_submitting.set(false);
                                        use_navigator()
                                            .push(crate::Route::PostDetail {
                                                post_pk: pk,
                                            });
                                    });
                                }
                            },
                            {t.comment_button}
                        }
                    }
                }
            }
            for comment in comments {
                {
                    let replies: Vec<PostCommentResponse> = reply_comments
                        .iter()
                        .filter(|r| {
                            if let Some(parent) = &r.parent_comment_sk {
                                *parent == comment.sk
                            } else {
                                false
                            }
                        })
                        .cloned()
                        .collect();
                    rsx! {
                        CommentItem {
                            key: "{comment.sk}",
                            comment: comment.clone(),
                            post_pk: post_pk.clone(),
                            replies,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentItem(
    comment: PostCommentResponse,
    post_pk: String,
    replies: Vec<PostCommentResponse>,
) -> Element {
    let t: PostDetailTranslate = use_translate();
    let mut optimistic_liked = use_signal(|| comment.liked);
    let mut optimistic_likes = use_signal(|| comment.likes as i64);
    let mut is_processing = use_signal(|| false);
    let mut show_reply = use_signal(|| false);
    let mut show_replies = use_signal(|| false);
    let mut reply_text = use_signal(|| String::new());
    let mut is_reply_submitting = use_signal(|| false);

    let comment_sk_for_like = comment.sk.clone();
    let comment_sk_for_reply = comment.sk.clone();

    let img_class = "object-cover object-top w-10 h-10 rounded-full";

    let updated_secs = comment.updated_at * 1000;
    let reply_label = t.reply;
    let replies_label = t.replies;
    let reply_text_label = use_memo(move || {
        let label = if comment.replies <= 1 {
            reply_label
        } else {
            replies_label
        };
        format!("{} {}", comment.replies, label)
    });

    rsx! {
        div { class: "flex flex-col gap-3 pb-4",
            div { class: "flex flex-row justify-between items-start w-full",
                div { class: "flex flex-row gap-2 items-center",
                    if !comment.author_profile_url.is_empty() {
                        img {
                            src: comment.author_profile_url.clone(),
                            alt: comment.author_display_name.clone(),
                            class: img_class,
                        }
                    } else {
                        div { class: "w-10 h-10 rounded-full bg-profile-bg" }
                    }
                    div { class: "flex flex-col gap-[2px]",
                        div { class: "font-semibold text-text-primary text-[15px] leading-[15px]",
                            {comment.author_display_name}
                        }
                        div { class: "font-semibold text-xs leading-[20px] text-text-primary",
                            {time_ago(updated_secs)}
                        }
                    }
                }
                div {}
            }
            div { class: "flex flex-col gap-3",
                TiptapEditor {
                    class: "w-full bg-transparent",
                    content: comment.content.clone(),
                    editable: false,
                }
            }
            div { class: "flex flex-row gap-2 justify-between items-center w-full",
                div { class: "flex flex-row gap-5",
                    button {
                        aria_label: "Expand Replies",
                        class: "flex flex-row gap-2 justify-center items-center disabled:cursor-not-allowed text-primary",
                        disabled: comment.replies == 0,
                        onclick: move |_| {
                            let next = !*show_replies.read();
                            show_replies.set(next);
                        },
                        span { class: "inline-flex items-center gap-2 text-primary",
                            {reply_text_label()}
                            if comment.replies > 0 {
                                icons::arrows::ChevronDown { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            }
                        }
                    }
                    button {
                        aria_label: t.reply_button,
                        class: "flex gap-2 justify-center items-center cursor-pointer text-text-primary",
                        onclick: move |_| {
                            let current = *show_reply.read();
                            show_reply.set(!current);
                        },
                        span { class: "inline-flex items-center gap-2 text-text-primary",
                            icons::arrows::BendArrowRight { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            {t.reply}
                        }
                    }
                }
                Button {
                    aria_label: "Like Comment",
                    style: ButtonStyle::Outline,
                    disabled: *is_processing.read(),
                    onclick: {
                        let pk = post_pk.clone();
                        let sk = comment_sk_for_like.clone();
                        move |_| {
                            if *is_processing.read() {
                                return;
                            }
                            let new_like = !*optimistic_liked.read();
                            let prev = *optimistic_likes.read();
                            let delta: i64 = if new_like { 1 } else { -1 };

                            optimistic_liked.set(new_like);
                            optimistic_likes.set((prev + delta).max(0));
                            is_processing.set(true);

                            let pk = pk.clone();
                            let sk = sk.clone();
                            spawn(async move {
                                let _ = like_comment_handler(pk.parse().unwrap(), sk, new_like).await;
                                is_processing.set(false);
                            });
                        }
                    },
                    span { class: "inline-flex items-center gap-2",
                        if optimistic_liked() {
                            icons::emoji::ThumbsUp { class: "w-6 h-6 [&>path]:fill-primary [&>path]:stroke-icon-primary" }
                        } else {
                            icons::emoji::ThumbsUp { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        }
                        div { class: "font-medium text-base/[24px] text-comment-icon-text",
                            {optimistic_likes().to_string()}
                        }
                    }
                }
            }
            if *show_reply.read() {
                div { class: "flex flex-col w-full bg-comment-box-bg border rounded-lg border-primary max-w-desktop mt-2",
                    div { class: "flex flex-row justify-between items-center px-3 pt-3",
                        span { class: "text-sm font-medium text-text-primary", {t.write_comment} }
                        Button {
                            aria_label: t.cancel,
                            style: ButtonStyle::Outline,
                            onclick: move |_| {
                                show_reply.set(false);
                                reply_text.set(String::new());
                            },
                            icons::arrows::DoubleArrowDown { class: "w-5 h-5 [&>path]:stroke-text-primary" }
                        }
                    }
                    div { class: "flex-1 w-full rounded-md transition-colors cursor-text hover:bg-foreground/5",
                        TiptapEditor {
                            class: "border-none",
                            content: reply_text.read().clone(),
                            editable: true,
                            placeholder: t.contents_hint,
                            on_content_change: move |val| {
                                reply_text.set(val);
                            },
                        }
                    }
                    div { class: "flex flex-row gap-2 justify-end items-center px-3 pt-3 pb-3 border-t border-divider",
                        Button {
                            style: ButtonStyle::Primary,
                            disabled: reply_text.read().trim().is_empty() || *is_reply_submitting.read(),
                            onclick: {
                                let pk = post_pk.clone();
                                let sk = comment_sk_for_reply.clone();
                                move |_| {
                                    let content = reply_text.read().clone();
                                    if content.trim().is_empty() || *is_reply_submitting.read() {
                                        return;
                                    }
                                    is_reply_submitting.set(true);
                                    let pk = pk.clone();
                                    let sk = sk.clone();
                                    spawn(async move {
                                        let _ = reply_to_comment_handler(pk.parse().unwrap(), sk, content).await;
                                        reply_text.set(String::new());
                                        show_reply.set(false);
                                        is_reply_submitting.set(false);
                                        use_navigator()
                                            .push(crate::Route::PostDetail {
                                                post_pk: pk,
                                            });
                                    });
                                }
                            },
                            span { class: "inline-flex items-center gap-2",
                                icons::chat::SquareChat { class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                                if is_reply_submitting() {
                                    {t.publishing}
                                } else {
                                    {t.publish}
                                }
                            }
                        }
                    }
                }
            }
            if show_replies() && !replies.is_empty() {
                div { class: "flex flex-col gap-2.5",
                    for reply in &replies {
                        ReplyItem { key: "{reply.sk}", reply: reply.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ReplyItem(reply: PostCommentResponse) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 p-5 rounded-lg bg-reply-box border border-transparent",
            div { class: "flex flex-row gap-2 items-center",
                if !reply.author_profile_url.is_empty() {
                    img {
                        src: reply.author_profile_url.clone(),
                        alt: reply.author_display_name.clone(),
                        class: "rounded-full object-cover object-top w-10 h-10",
                    }
                } else {
                    div { class: "rounded-full w-10 h-10 bg-profile-bg" }
                }
                div { class: "flex flex-col gap-[2px]",
                    div { class: "font-semibold text-title-text text-[15px] leading-[15px]",
                        {reply.author_display_name}
                    }
                }
            }
            TiptapEditor {
                class: "w-full bg-transparent",
                content: reply.content.clone(),
                editable: false,
            }
        }
    }
}
