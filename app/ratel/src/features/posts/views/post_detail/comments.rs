//! Post comments panel — arena-styled port of the space-discussion
//! comments panel. Shares CSS class names (`comments-panel`, `sheet-handle`,
//! `comment-input`, `comment-list`, `comment-item`, `comment-replies`,
//! `reply-input`, `comment-action`) with
//! `features/spaces/pages/index/action_pages/discussion` so the visual
//! language is identical; the wiring just targets Post comment controllers
//! (`add_comment_handler`, `like_comment_handler`, `reply_to_comment_handler`)
//! instead of discussion ones. Edit/delete are intentionally omitted — the
//! post-comment backend has no `update_comment` / `delete_comment`
//! controllers yet; re-enable those affordances once those land.

use crate::common::components::mention_autocomplete::{
    MentionAutocomplete, MentionCandidate, MentionInsert,
};
use crate::common::utils::mention::{apply_mention_markup, parse_mention_segments, ContentSegment};
use crate::features::posts::controllers::comments::add_comment::{
    add_comment_handler, AddPostCommentRequest,
};
use crate::features::posts::controllers::comments::like_comment::like_comment_handler;
use crate::features::posts::controllers::comments::list_comments::list_comments_handler;
use crate::features::posts::controllers::comments::reply_to_comment::{
    reply_to_comment_handler, ReplyToPostCommentRequest,
};
use crate::features::posts::controllers::dto::{PostCommentResponse, PostDetailResponse};
use crate::features::posts::types::PostCommentTargetEntityType;
use crate::features::posts::*;
use crate::PostCommentEntityType;

use super::i18n::PostDetailSyndicatedTranslate;

#[component]
pub fn PostCommentsPanel(
    detail: PostDetailResponse,
    post_pk: FeedPartition,
    on_refresh: EventHandler<()>,
) -> Element {
    let tr: PostDetailSyndicatedTranslate = use_translate();

    let mut comment_text = use_signal(String::new);
    let mut tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let members: Signal<Vec<MentionCandidate>> = use_signal(Vec::new);
    let mut is_submitting = use_signal(|| false);

    let total = detail.post.as_ref().map(|p| p.comments).unwrap_or(0);

    // Top-level comments only — replies are fetched lazily inside CommentItem.
    let comments: Vec<PostCommentResponse> = {
        let mut out: Vec<PostCommentResponse> = Vec::new();
        for c in detail.comments.iter() {
            if c.parent_comment_sk.is_none() && !out.iter().any(|x| x.sk == c.sk) {
                out.push(c.clone());
            }
        }
        out
    };

    let submit_post_pk = post_pk.clone();
    let on_submit = move |_| {
        let submit_post_pk = submit_post_pk.clone();
        async move {
            let raw = comment_text().trim().to_string();
            if raw.is_empty() || is_submitting() {
                return;
            }
            is_submitting.set(true);
            let content = apply_mention_markup(&raw, &tracked_mentions.read());
            let req = AddPostCommentRequest {
                content,
                images: vec![],
            };
            match add_comment_handler(submit_post_pk, req).await {
                Ok(_) => {
                    comment_text.set(String::new());
                    tracked_mentions.set(Vec::new());
                    on_refresh.call(());
                }
                Err(e) => {
                    tracing::error!("post comment submit failed: {e}");
                }
            }
            is_submitting.set(false);
        }
    };

    rsx! {
        div { class: "comments-panel", id: "post-comments-sheet",
            div { class: "sheet-handle",
                div { class: "sheet-handle__bar" }
                div { class: "sheet-handle__row",
                    div { class: "sheet-handle__left",
                        span { class: "sheet-handle__title", "{tr.drawer_comments_title}" }
                        span { class: "sheet-handle__count", "{total}" }
                    }
                    svg {
                        class: "sheet-handle__chevron",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "6 9 12 15 18 9" }
                    }
                }
            }

            div { class: "comments-panel__header",
                span { class: "comments-panel__title", "{tr.drawer_comments_title}" }
                span { class: "comments-panel__count", "{total}" }
            }

            div { class: "comment-input",
                div { class: "comment-input__wrapper",
                    div { class: "comment-input__body",
                        MentionAutocomplete {
                            text: comment_text,
                            on_select: move |insert: MentionInsert| {
                                let mut val = comment_text();
                                if insert.start_offset <= val.len() && insert.end_offset <= val.len() {
                                    val.replace_range(
                                        insert.start_offset..insert.end_offset,
                                        &insert.display_text,
                                    );
                                    comment_text.set(val);
                                    tracked_mentions.write().push((insert.display_name, insert.user_pk));
                                }
                            },
                            members,
                            textarea {
                                class: "comment-input__textarea",
                                placeholder: "{tr.comment_placeholder}",
                                rows: "2",
                                value: "{comment_text}",
                                oninput: move |e| comment_text.set(e.value()),
                            }
                        }
                        div { class: "comment-input__footer",
                            button {
                                class: "comment-input__submit",
                                disabled: comment_text().trim().is_empty() || is_submitting(),
                                onclick: on_submit,
                                "{tr.post_btn}"
                            }
                        }
                    }
                }
            }

            div { class: "comments-scroll",
                div { class: "comment-list",
                    for comment in comments.iter() {
                        CommentItem {
                            key: "{comment.sk}",
                            comment: comment.clone(),
                            post_pk: post_pk.clone(),
                            on_refresh,
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
) -> Element {
    let tr: PostDetailSyndicatedTranslate = use_translate();

    let mut show_replies = use_signal(|| false);
    let mut reply_text = use_signal(String::new);
    let mut reply_tracked: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let members: Signal<Vec<MentionCandidate>> = use_signal(Vec::new);
    let mut replies: Signal<Vec<PostCommentResponse>> = use_signal(Vec::new);

    let time_ago = format_time_ago(comment.updated_at);
    let mut liked = use_signal(|| comment.liked);
    let mut likes = use_signal(|| comment.likes as i64);
    let reply_count = comment.replies;

    let comment_sk = comment.sk.clone();
    let like_post_pk = post_pk.clone();
    let like_sk = comment_sk.clone();
    let on_like = move |_| {
        let like_post_pk = like_post_pk.clone();
        let like_sk = like_sk.clone();
        async move {
            let next = !liked();
            let prev_likes = likes();
            liked.set(next);
            likes.set((prev_likes + if next { 1 } else { -1 }).max(0));
            match like_comment_handler(like_post_pk, like_sk, next).await {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("like post comment failed: {e}");
                    liked.set(!next);
                    likes.set(prev_likes);
                }
            }
        }
    };

    let reply_post_pk = post_pk.clone();
    let reply_sk = comment_sk.clone();
    let on_submit_reply = move |_| {
        let reply_post_pk = reply_post_pk.clone();
        let reply_sk = reply_sk.clone();
        async move {
            let raw = reply_text().trim().to_string();
            if raw.is_empty() {
                return;
            }
            let content = apply_mention_markup(&raw, &reply_tracked.read());
            // PostCommentTargetEntityType → EntityType → PostCommentEntityType
            // (no direct From between the two sub-partitions; go via the
            // shared EntityType enum.)
            let as_entity: EntityType = reply_sk.into();
            let parent_sk: PostCommentEntityType = as_entity.into();
            let req = ReplyToPostCommentRequest {
                content,
                images: vec![],
            };
            match reply_to_comment_handler(reply_post_pk, parent_sk, req).await {
                Ok(new_reply_model) => {
                    // Optimistically prepend to the local replies list so
                    // the new reply appears without waiting for the loader
                    // restart (which also refreshes the top-level count).
                    let new_reply: PostCommentResponse = (new_reply_model, false, false).into();
                    replies.with_mut(|list| list.insert(0, new_reply));
                    reply_text.set(String::new());
                    reply_tracked.set(Vec::new());
                    on_refresh.call(());
                }
                Err(e) => tracing::error!("reply post comment failed: {e}"),
            }
        }
    };

    // Lazy-load replies on first expand. The effect re-runs when
    // `show_replies` flips to true; `replies.is_empty()` guards against a
    // refetch loop when `replies.set(...)` fires below.
    let eff_post_pk = post_pk.clone();
    let eff_sk = comment_sk.clone();
    use_effect(move || {
        if show_replies() && replies.read().is_empty() && reply_count > 0 {
            let eff_post_pk = eff_post_pk.clone();
            let eff_sk = eff_sk.clone();
            spawn(async move {
                let as_entity: EntityType = eff_sk.into();
                let parent_id: PostCommentEntityType = as_entity.into();
                match list_comments_handler(eff_post_pk, parent_id, None).await {
                    Ok(resp) => replies.set(resp.items),
                    Err(e) => tracing::error!("list post replies failed: {e}"),
                }
            });
        }
    });

    rsx! {
        div { class: "comment-entry",
            div { class: "comment-item",
                img {
                    class: "comment-item__avatar",
                    src: "{comment.author_profile_url}",
                    alt: "{comment.author_display_name}",
                }
                div { class: "comment-item__body",
                    div { class: "comment-item__top",
                        span { class: "comment-item__name", "{comment.author_display_name}" }
                        span { class: "comment-item__time", "{time_ago}" }
                    }
                    div { class: "comment-item__text",
                        for segment in parse_mention_segments(&comment.content) {
                            match segment {
                                ContentSegment::Text(t) => rsx! {
                                    span { "{t}" }
                                },
                                ContentSegment::Mention { display_name, .. } => rsx! {
                                    span { class: "comment-item__mention", "@{display_name}" }
                                },
                            }
                        }
                    }
                    div { class: "comment-item__actions",
                        button {
                            class: if liked() { "comment-action comment-action--liked" } else { "comment-action" },
                            onclick: on_like,
                            svg {
                                view_box: "0 0 24 24",
                                fill: if liked() { "currentColor" } else { "none" },
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" }
                            }
                            span { "{likes()}" }
                        }
                        button {
                            class: "comment-action comment-action--reply",
                            onclick: move |_| {
                                let next = !show_replies();
                                show_replies.set(next);
                            },
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z" }
                            }
                            span { "{tr.reply_label}" }
                        }
                    }
                }
            }

            if reply_count > 0 {
                button {
                    class: "reply-toggle",
                    onclick: move |_| show_replies.set(!show_replies()),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        if show_replies() {
                            polyline { points: "18 15 12 9 6 15" }
                        } else {
                            polyline { points: "6 9 12 15 18 9" }
                        }
                    }
                    "{reply_count} {tr.replies_label}"
                }
            }

            if show_replies() {
                // Input FIRST so it's immediately visible after toggling —
                // otherwise long reply lists push the input below the fold
                // and the user has to scroll to type a reply.
                div { class: "reply-input",
                    MentionAutocomplete {
                        text: reply_text,
                        on_select: move |insert: MentionInsert| {
                            let mut val = reply_text();
                            if insert.start_offset <= val.len() && insert.end_offset <= val.len() {
                                val.replace_range(
                                    insert.start_offset..insert.end_offset,
                                    &insert.display_text,
                                );
                                reply_text.set(val);
                                reply_tracked.write().push((insert.display_name, insert.user_pk));
                            }
                        },
                        members,
                        input {
                            class: "reply-input__field",
                            placeholder: "{tr.reply_placeholder}",
                            value: "{reply_text}",
                            oninput: move |e| reply_text.set(e.value()),
                        }
                    }
                    button { class: "reply-input__send", onclick: on_submit_reply,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line {
                                x1: "22",
                                y1: "2",
                                x2: "11",
                                y2: "13",
                            }
                            polygon { points: "22 2 15 22 11 13 2 9 22 2" }
                        }
                    }
                }

                if !replies().is_empty() {
                    div { class: "comment-replies",
                        for reply in replies().iter() {
                            ReplyItem { key: "{reply.sk}", reply: reply.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ReplyItem(reply: PostCommentResponse) -> Element {
    let time_ago = format_time_ago(reply.updated_at);
    let liked = reply.liked;
    let likes = reply.likes;

    rsx! {
        div { class: "comment-item",
            img {
                class: "comment-item__avatar",
                src: "{reply.author_profile_url}",
                alt: "{reply.author_display_name}",
            }
            div { class: "comment-item__body",
                div { class: "comment-item__top",
                    span { class: "comment-item__name", "{reply.author_display_name}" }
                    span { class: "comment-item__time", "{time_ago}" }
                }
                div { class: "comment-item__text",
                    for segment in parse_mention_segments(&reply.content) {
                        match segment {
                            ContentSegment::Text(t) => rsx! {
                                span { "{t}" }
                            },
                            ContentSegment::Mention { display_name, .. } => rsx! {
                                span { class: "comment-item__mention", "@{display_name}" }
                            },
                        }
                    }
                }
                div { class: "comment-item__actions",
                    span { class: if liked { "comment-action comment-action--liked" } else { "comment-action" },
                        svg {
                            view_box: "0 0 24 24",
                            fill: if liked { "currentColor" } else { "none" },
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" }
                        }
                        span { "{likes}" }
                    }
                }
            }
        }
    }
}

fn format_time_ago(ts_ms_or_s: i64) -> String {
    let ts_ms = if ts_ms_or_s.abs() < 1_000_000_000_000 {
        ts_ms_or_s.saturating_mul(1000)
    } else {
        ts_ms_or_s
    };
    let now = chrono::Utc::now().timestamp_millis();
    let diff = ((now - ts_ms) / 1000).max(0);
    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else {
        format!("{}d ago", diff / 86400)
    }
}
