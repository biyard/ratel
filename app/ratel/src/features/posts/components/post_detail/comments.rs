use super::PostDetailTranslate;
use crate::common::components::{Button, ButtonShape, ButtonSize, ButtonStyle, TextArea};
use crate::common::hooks::use_infinite_query;
use crate::features::posts::controllers::comments::add_comment::add_comment_handler;
use crate::features::posts::controllers::comments::like_comment::like_comment_handler;
use crate::features::posts::types::PostCommentTargetEntityType;
use crate::features::posts::controllers::comments::list_comments::list_comments_handler;
use crate::features::posts::controllers::comments::reply_to_comment::reply_to_comment_handler;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::*;
use dioxus::prelude::*;

#[component]
pub fn CommentSection(
    detail: PostDetailResponse,
    post_pk: FeedPartition,
    on_refresh: EventHandler<()>,
) -> Element {
    let t: PostDetailTranslate = use_translate();
    let mut comment_count = use_signal(|| detail.post.as_ref().map(|p| p.comments).unwrap_or(0));
    let mut comment_input = use_signal(String::new);
    let mut is_submitting = use_signal(|| false);
    let post_pk_signal = use_signal(|| post_pk.clone());

    let comments: Vec<PostCommentResponse> = {
        let mut result: Vec<PostCommentResponse> = Vec::new();
        for c in detail.comments.iter() {
            if c.parent_comment_sk.is_none() && !result.iter().any(|r| r.sk == c.sk) {
                result.push(c.clone());
            }
        }
        result
    };

    rsx! {
        div { id: "comments", class: "flex flex-col gap-4",
            h2 { class: "text-lg font-bold text-text-primary",
                "{t.comments_title} ({comment_count()})"
            }
            div { class: "flex gap-2 items-end",
                TextArea {
                    class: "flex-1 min-h-10 resize-none rounded-[10px] border border-input-box-border bg-input-box-bg px-3 py-2 text-sm text-text-primary outline-none placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]",
                    placeholder: t.share_your_thoughts,
                    value: comment_input(),
                    oninput: move |e: Event<FormData>| comment_input.set(e.value()),
                    onkeydown: move |evt: KeyboardEvent| async move {
                        if evt.key() == Key::Enter
                            && (evt.modifiers().contains(Modifiers::CONTROL)
                                || evt.modifiers().contains(Modifiers::META))
                        {
                            evt.prevent_default();
                            let content = comment_input().trim().to_string();
                            if content.is_empty() || is_submitting() {
                                return;
                            }
                            is_submitting.set(true);
                            comment_input.set(String::new());
                            if add_comment_handler(post_pk_signal(), content).await.is_ok() {
                                comment_count.set(comment_count() + 1);
                                on_refresh.call(());
                            }
                            is_submitting.set(false);
                        }
                    },
                }
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Rounded,
                    size: ButtonSize::Icon,
                    class: "size-10 shrink-0 !p-0 inline-flex items-center justify-center",
                    disabled: comment_input().trim().is_empty() || is_submitting(),
                    onclick: move |_| async move {
                        let content = comment_input().trim().to_string();
                        if content.is_empty() || is_submitting() {
                            return;
                        }
                        is_submitting.set(true);
                        comment_input.set(String::new());
                        if add_comment_handler(post_pk_signal(), content).await.is_ok() {
                            comment_count.set(comment_count() + 1);
                            on_refresh.call(());
                        }
                        is_submitting.set(false);
                    },
                    if comment_input().trim().is_empty() {
                        span { class: "inline-flex items-center justify-center leading-none",
                            icons::chat::SquareChat { class: "size-5 [&>path]:stroke-btn-primary-disable-text [&>path]:fill-transparent" }
                        }
                    } else {
                        span { class: "inline-flex items-center justify-center leading-none",
                            icons::chat::SquareChat { class: "size-5 [&>path]:stroke-btn-primary-text [&>path]:fill-transparent" }
                        }
                    }
                }
            }
            div { class: "flex flex-col divide-y divide-divider",
                for comment in comments {
                    div {
                        key: "{comment.sk}",
                        class: "py-3 first:pt-0 last:pb-0",
                        CommentItem {
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
    let mut show_reply_input = use_signal(|| false);
    let mut show_replies = use_signal(|| false);
    let mut reply_text = use_signal(String::new);
    let mut is_reply_submitting = use_signal(|| false);

    let comment_sk_for_like = comment.sk.clone();
    let comment_sk_for_reply: PostCommentEntityType = {
        let et: EntityType = comment.sk.clone().into();
        et.into()
    };
    let post_pk_signal = use_signal(|| post_pk.clone());
    let comment_sk_signal = use_signal(|| comment_sk_for_reply.clone());

    let updated_millis = comment.updated_at * 1000;
    let comment_time = time_ago(updated_millis);
    let mut reply_count = use_signal(|| comment.replies);

    let mut replies = use_infinite_query(move |bookmark| {
        let post_pk = post_pk_signal();
        let comment_id = comment_sk_signal();
        async move { list_comments_handler(post_pk, comment_id, bookmark).await }
    })?;

    rsx! {
        div { class: "flex flex-col gap-3 rounded-xl bg-card px-4 py-3",
            // Header: author info
            div { class: "flex justify-between items-center",
                div { class: "flex gap-2 items-center text-sm",
                    if !comment.author_profile_url.is_empty() {
                        img {
                            class: "w-5 h-5 rounded-full object-cover",
                            src: "{comment.author_profile_url}",
                        }
                    }
                    span { class: "font-semibold text-text-primary",
                        {comment.author_display_name.clone()}
                    }
                    span { class: "text-xs text-text-secondary", "{comment_time}" }
                }
            }

            // Content
            p {
                class: "whitespace-pre-wrap break-words text-sm text-text-primary",
                dangerous_inner_html: "{comment.content}",
            }

            // Actions: replies toggle + like
            div { class: "flex items-center justify-between text-xs text-text-secondary",
                Button {
                    size: ButtonSize::Inline,
                    style: ButtonStyle::Text,
                    class: "inline-flex items-center text-text-secondary hover:text-primary",
                    onclick: move |_| {
                        let is_open = show_replies() || show_reply_input();
                        if is_open {
                            show_replies.set(false);
                            show_reply_input.set(false);
                        } else {
                            show_replies.set(true);
                            show_reply_input.set(true);
                        }
                    },
                    span { class: "inline-flex items-center gap-1 leading-none",
                        icons::chat::SquareChat { class: "size-4 shrink-0 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        span { class: "font-normal text-text-secondary text-[12px]",
                            "{reply_count()} {t.replies}"
                        }
                    }
                }
                // Like button
                Button {
                    size: ButtonSize::Inline,
                    style: ButtonStyle::Text,
                    class: if optimistic_liked() { "inline-flex items-center gap-1.5 text-sm text-primary hover:text-primary" } else { "inline-flex items-center gap-1.5 text-sm text-text-secondary hover:text-primary" },
                    disabled: is_processing(),
                    onclick: {
                        let pk = post_pk.clone();
                        let sk = comment_sk_for_like.clone();
                        move |_| {
                            let pk = pk.clone();
                            let sk = sk.clone();
                            async move {
                                if is_processing() {
                                    return;
                                }
                                let new_like = !optimistic_liked();
                                let prev = optimistic_likes();
                                let delta: i64 = if new_like { 1 } else { -1 };
                                optimistic_liked.set(new_like);
                                optimistic_likes.set((prev + delta).max(0));
                                is_processing.set(true);

                                if like_comment_handler(pk, sk, new_like).await.is_ok() {
                                    on_refresh.call(());
                                }
                                is_processing.set(false);
                            }
                        }
                    },
                    if optimistic_liked() {
                        icons::emoji::ThumbsUp { class: "size-4 [&>path]:fill-primary [&>path]:stroke-primary" }
                    } else {
                        icons::emoji::ThumbsUp { class: "size-4 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    }
                    span { "{optimistic_likes()}" }
                }
            }

            // Reply input
            if show_reply_input() {
                div { class: "mt-1 rounded-xl bg-card-bg-secondary p-3",
                    TextArea {
                        class: "h-20 w-full resize-none rounded-lg bg-input-box-bg border border-input-box-border px-3 py-2 text-sm text-text-primary outline-none placeholder:text-text-tertiary",
                        placeholder: t.contents_hint,
                        value: reply_text(),
                        oninput: move |e: Event<FormData>| reply_text.set(e.value()),
                        onkeydown: move |evt: KeyboardEvent| async move {
                            if evt.key() == Key::Enter
                                && (evt.modifiers().contains(Modifiers::CONTROL)
                                    || evt.modifiers().contains(Modifiers::META))
                            {
                                evt.prevent_default();
                                let content = reply_text().trim().to_string();
                                if content.is_empty() || is_reply_submitting() {
                                    return;
                                }
                                is_reply_submitting.set(true);
                                if reply_to_comment_handler(post_pk_signal(), comment_sk_signal(), content)
                                    .await
                                    .is_ok()
                                {
                                    reply_count.set(reply_count() + 1);
                                    on_comment_count_inc.call(());
                                    replies.refresh();
                                    show_replies.set(true);
                                }
                                reply_text.set(String::new());
                                is_reply_submitting.set(false);
                            }
                        },
                    }
                    div { class: "mt-2 flex justify-end",
                        Button {
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Rounded,
                            size: ButtonSize::Icon,
                            class: "size-10 !p-0 inline-flex items-center justify-center",
                            disabled: reply_text().trim().is_empty() || is_reply_submitting(),
                            onclick: move |_| async move {
                                let content = reply_text().trim().to_string();
                                if content.is_empty() || is_reply_submitting() {
                                    return;
                                }
                                is_reply_submitting.set(true);
                                if reply_to_comment_handler(post_pk_signal(), comment_sk_signal(), content)
                                    .await
                                    .is_ok()
                                {
                                    reply_count.set(reply_count() + 1);
                                    on_comment_count_inc.call(());
                                    replies.refresh();
                                    show_replies.set(true);
                                }
                                reply_text.set(String::new());
                                is_reply_submitting.set(false);
                            },
                            span { class: "inline-flex items-center justify-center leading-none",
                                icons::chat::SquareChat { class: "size-5 [&>path]:stroke-btn-primary-text [&>path]:fill-transparent" }
                            }
                        }
                    }
                }
            }

            // Replies
            if show_replies() && reply_count() > 0 {
                div { class: "ml-5 flex flex-col gap-2 pl-4",
                    for (idx , reply) in replies.clone().items().into_iter().enumerate() {
                        ReplyItem {
                            key: "reply-{idx}-{reply.updated_at}",
                            reply: reply.clone(),
                            post_pk: post_pk.clone(),
                            on_refresh: on_refresh.clone(),
                        }
                    }
                    if replies.has_more() {
                        {replies.more_element()}
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
    let reply_time = time_ago(reply.updated_at * 1000);

    rsx! {
        div { class: "flex flex-col gap-2 rounded-lg border border-divider bg-card px-3 py-2.5",
            div { class: "flex items-center gap-2 text-sm",
                if !reply.author_profile_url.is_empty() {
                    img {
                        class: "size-4 rounded-full object-cover",
                        src: "{reply.author_profile_url}",
                    }
                }
                span { class: "font-semibold text-text-primary", {reply.author_display_name.clone()} }
                span { class: "text-xs text-text-secondary", "{reply_time}" }
            }
            p {
                class: "whitespace-pre-wrap break-words text-sm text-text-primary",
                dangerous_inner_html: "{reply.content}",
            }
            div { class: "flex justify-end pt-1",
                Button {
                    size: ButtonSize::Inline,
                    style: ButtonStyle::Text,
                    class: if optimistic_liked() { "inline-flex items-center gap-1.5 text-sm text-primary" } else { "inline-flex items-center gap-1.5 text-sm text-text-secondary hover:text-primary" },
                    disabled: is_processing(),
                    onclick: {
                        let pk = post_pk.clone();
                        let sk = reply.sk.clone();
                        move |_| {
                            let pk = pk.clone();
                            let sk = sk.clone();
                            async move {
                                if is_processing() {
                                    return;
                                }
                                let new_like = !optimistic_liked();
                                let prev = optimistic_likes();
                                let delta: i64 = if new_like { 1 } else { -1 };
                                optimistic_liked.set(new_like);
                                optimistic_likes.set((prev + delta).max(0));
                                is_processing.set(true);

                                if like_comment_handler(pk, sk, new_like).await.is_ok() {
                                    on_refresh.call(());
                                }
                                is_processing.set(false);
                            }
                        }
                    },
                    if optimistic_liked() {
                        icons::emoji::ThumbsUp { class: "size-4 [&>path]:fill-primary [&>path]:stroke-primary" }
                    } else {
                        icons::emoji::ThumbsUp { class: "size-4 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    }
                    span { "{optimistic_likes()}" }
                }
            }
        }
    }
}
