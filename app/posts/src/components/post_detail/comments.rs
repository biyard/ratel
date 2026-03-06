use super::utils::time_ago;
use super::PostDetailTranslate;
use crate::controllers::comments::add_comment::add_comment_handler;
use crate::controllers::comments::like_comment::like_comment_handler;
use crate::controllers::comments::list_comments::list_comments_handler;
use crate::controllers::comments::reply_to_comment::reply_to_comment_handler;
use crate::controllers::dto::*;
use crate::*;
use common::components::TiptapEditor;
use common::components::{Button, ButtonSize, ButtonStyle, TextArea};
use common::hooks::use_infinite_query;
use dioxus::prelude::*;

#[component]
pub fn CommentSection(
    detail: PostDetailResponse,
    post_pk: FeedPartition,
    on_refresh: EventHandler<()>,
) -> Element {
    let t: PostDetailTranslate = use_translate();
    let mut comment_count = use_signal(|| detail.post.as_ref().map(|p| p.comments).unwrap_or(0));
    let mut expand_comment = use_signal(|| false);
    let mut comment_text = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);

    let comments: Vec<PostCommentResponse> = detail
        .comments
        .iter()
        .filter(|c| c.parent_comment_sk.is_none())
        .cloned()
        .collect();

    let reply_label = t.reply;
    let replies_label = t.replies;
    let count_text = use_memo(move || {
        let count = comment_count();
        let label = if count == 1 {
            reply_label
        } else {
            replies_label
        };
        format!("{} {}", count, label)
    });

    rsx! {
        div { id: "comments", class: "flex flex-col gap-2.5",
            div { class: "flex flex-row gap-2 text-text-primary",
                icons::chat::SquareChat { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                span { class: "font-medium text-base/6", {count_text()} }
            }
            if !expand_comment() {
                Button {
                    size: ButtonSize::Inline,
                    style: ButtonStyle::Text,
                    class: "flex flex-row gap-2 items-center py-3 px-3.5 w-full rounded-lg border transition-all duration-200 cursor-pointer bg-write-comment-box-bg border-write-comment-box-border group hover:bg-write-comment-box-bg/80 hover:border-primary/50"
                        .to_string(),
                    onclick: move |_| {
                        expand_comment.set(true);
                    },
                    span { class: "inline-flex gap-2 items-center",
                        icons::chat::SquareChat { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        span { class: "font-medium text-write-comment-box-text text-[15px]/[24px]",
                            {t.share_your_thoughts}
                        }
                    }
                }
            }
            if expand_comment() {
                div { class: "flex flex-col gap-2 p-4 rounded-lg border border-card-enable-border bg-card-bg-secondary",
                    TextArea {
                        class: "p-2 w-full bg-transparent rounded border resize-none focus:outline-none min-h-[80px] text-text-primary border-divider focus:border-primary"
                            .to_string(),
                        placeholder: t.share_your_thoughts.to_string(),
                        value: comment_text(),
                        oninput: move |e: FormEvent| {
                            comment_text.set(e.value());
                        },
                    }
                    div { class: "flex flex-row gap-2 justify-end",
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
                                    let on_refresh = on_refresh.clone();
                                    spawn(async move {
                                        if add_comment_handler(pk, content).await.is_ok() {
                                            comment_count.set(comment_count() + 1);
                                            on_refresh.call(());
                                        }
                                        comment_text.set(String::new());
                                        expand_comment.set(false);
                                        is_submitting.set(false);
                                    });
                                }
                            },
                            {t.comment_button}
                        }
                    }
                }
            }
            for comment in comments {
                CommentItem {
                    key: "{comment.sk}",
                    comment: comment.clone(),
                    post_pk: post_pk.clone(),
                    on_refresh: on_refresh.clone(),
                    on_comment_count_inc: {
                        let mut comment_count = comment_count.clone();
                        move || {
                            comment_count.set(comment_count() + 1);
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn CommentItem(
    comment: PostCommentResponse,
    post_pk: FeedPartition,
    on_refresh: EventHandler<()>,
    on_comment_count_inc: EventHandler<()>,
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
    let post_pk_signal = use_signal(|| post_pk.clone());
    let comment_sk_signal = use_signal(|| comment_sk_for_reply.clone());

    let img_class = "object-cover object-top w-10 h-10 rounded-full";

    let updated_secs = use_memo(move || comment.updated_at * 1000);
    let comment_replies = use_signal(|| comment.replies);
    let reply_label = t.reply;
    let replies_label = t.replies;
    let reply_text_label = use_memo(move || {
        let count = comment_replies();
        let label = if count <= 1 {
            reply_label
        } else {
            replies_label
        };
        format!("{} {}", count, label)
    });

    let replies = use_infinite_query(move |bookmark| {
        let post_pk = post_pk_signal();
        let comment_id = comment_sk_signal();
        async move { list_comments_handler(post_pk, comment_id, bookmark).await }
    })?;

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
                        div { class: "text-xs font-semibold leading-[20px] text-text-primary",
                            {time_ago(updated_secs())}
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
                    Button {
                        size: ButtonSize::Inline,
                        style: ButtonStyle::Text,
                        aria_label: "Expand Replies",
                        class: "flex flex-row gap-2 justify-center items-center disabled:cursor-not-allowed text-primary"
                            .to_string(),
                        disabled: comment_replies() == 0,
                        onclick: move |_| {
                            let next = !*show_replies.read();
                            show_replies.set(next);
                        },
                        span { class: "inline-flex gap-2 items-center text-primary",
                            {reply_text_label()}
                            if comment_replies() > 0 {
                                icons::arrows::ChevronDown { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            }
                        }
                    }
                    Button {
                        size: ButtonSize::Inline,
                        style: ButtonStyle::Text,
                        aria_label: t.reply_button,
                        class: "flex gap-2 justify-center items-center cursor-pointer text-text-primary"
                            .to_string(),
                        onclick: move |_| {
                            let current = *show_reply.read();
                            show_reply.set(!current);
                        },
                        span { class: "inline-flex gap-2 items-center text-text-primary",
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
                            if is_processing() {
                                return;
                            }
                            let new_like = !optimistic_liked();
                            let prev = optimistic_likes();
                            let delta: i64 = if new_like { 1 } else { -1 };

                            optimistic_liked.set(new_like);
                            optimistic_likes.set((prev + delta).max(0));
                            is_processing.set(true);

                            let pk = pk.clone();
                            let sk = sk.clone();
                            let on_refresh = on_refresh.clone();
                            spawn(async move {
                                if like_comment_handler(pk, sk, new_like).await.is_ok() {
                                    on_refresh.call(());
                                }
                                is_processing.set(false);
                            });
                        }
                    },
                    span { class: "inline-flex gap-2 items-center",
                        if optimistic_liked() {
                            icons::emoji::ThumbsUp { class: "size-5 [&>path]:fill-primary [&>path]:stroke-primary" }
                        } else {
                            icons::emoji::ThumbsUp { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        }
                        div { class: "font-medium text-base/[24px] text-comment-icon-text",
                            {optimistic_likes().to_string()}
                        }
                    }
                }
            }
            if *show_reply.read() {
                div { class: "flex flex-col mt-2 w-full rounded-lg border bg-comment-box-bg border-primary max-w-desktop",
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
                    div { class: "flex-1 w-full rounded-md transition-colors",
                        TextArea {
                            class: "p-2 m-1 w-full bg-transparent rounded border resize-none focus:outline-none min-h-[80px] text-text-primary border-divider focus:border-primary"
                                .to_string(),
                            placeholder: t.contents_hint.to_string(),
                            value: reply_text(),
                            oninput: move |e: FormEvent| {
                                reply_text.set(e.value());
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
                                let replies = replies.clone();
                                move |_| {
                                    let content = reply_text.read().clone();
                                    if content.trim().is_empty() || *is_reply_submitting.read() {
                                        return;
                                    }
                                    is_reply_submitting.set(true);
                                    let pk = pk.clone();
                                    let sk = sk.clone();
                                    let on_refresh = on_refresh.clone();
                                    let on_comment_count_inc = on_comment_count_inc.clone();
                                    let mut show_replies = show_replies.clone();
                                    let mut comment_replies = comment_replies.clone();
                                    let mut replies = replies.clone();
                                    spawn(async move {
                                        if reply_to_comment_handler(pk, sk, content).await.is_ok() {
                                            comment_replies.set(comment_replies() + 1);
                                            on_comment_count_inc.call(());
                                            on_refresh.call(());
                                            replies.restart();
                                            show_replies.set(true);
                                        }
                                        reply_text.set(String::new());
                                        show_reply.set(false);
                                        is_reply_submitting.set(false);
                                    });
                                }
                            },
                            span { class: "inline-flex gap-2 items-center",
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
            if show_replies() && comment_replies() > 0 {
                div { class: "flex flex-col gap-2.5",
                    for reply in replies.clone().items() {
                        ReplyItem {
                            key: "{reply.sk}",
                            reply: reply.clone(),
                            post_pk: post_pk.clone(),
                            on_refresh: on_refresh.clone(),
                        }
                    }
                    if replies.has_more() {
                        {
                            let mut v_next = replies.clone();
                            rsx! {
                                div { class: "flex justify-center",
                                    Button {
                                        style: ButtonStyle::Outline,
                                        disabled: replies.is_loading(),
                                        onclick: move |_| {
                                            v_next.next();
                                        },
                                        {t.load_more}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ReplyItem(
    reply: PostCommentResponse,
    post_pk: FeedPartition,
    on_refresh: EventHandler<()>,
) -> Element {
    let mut optimistic_liked = use_signal(|| reply.liked);
    let mut optimistic_likes = use_signal(|| reply.likes as i64);
    let mut is_processing = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col gap-2 p-5 rounded-lg border border-transparent bg-reply-box",
            div { class: "flex flex-row gap-2 items-center",
                if !reply.author_profile_url.is_empty() {
                    img {
                        src: reply.author_profile_url.clone(),
                        alt: reply.author_display_name.clone(),
                        class: "object-cover object-top w-10 h-10 rounded-full",
                    }
                } else {
                    div { class: "w-10 h-10 rounded-full bg-profile-bg" }
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
            div { class: "flex justify-end mt-2",
                Button {
                    aria_label: "Like Reply",
                    style: ButtonStyle::Outline,
                    disabled: *is_processing.read(),
                    onclick: {
                        let pk = post_pk.clone();
                        let sk = reply.sk.clone();
                        move |_| {
                            if is_processing() {
                                return;
                            }
                            let new_like = !optimistic_liked();
                            let prev = optimistic_likes();
                            let delta: i64 = if new_like { 1 } else { -1 };

                            optimistic_liked.set(new_like);
                            optimistic_likes.set((prev + delta).max(0));
                            is_processing.set(true);

                            let pk = pk.clone();
                            let sk = sk.clone();
                            let on_refresh = on_refresh.clone();
                            spawn(async move {
                                if like_comment_handler(pk, sk, new_like).await.is_ok() {
                                    on_refresh.call(());
                                }
                                is_processing.set(false);
                            });
                        }
                    },
                    span { class: "inline-flex gap-2 items-center",
                        if optimistic_liked() {
                            icons::emoji::ThumbsUp { class: "size-5 [&>path]:fill-primary [&>path]:stroke-primary" }
                        } else {
                            icons::emoji::ThumbsUp { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        }
                        div { class: "font-medium text-base/[24px] text-comment-icon-text",
                            {optimistic_likes().to_string()}
                        }
                    }
                }
            }
        }
    }
}
