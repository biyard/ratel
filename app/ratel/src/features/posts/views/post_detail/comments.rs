//! Post comments panel — arena-styled port of the space-discussion
//! comments panel. Shares CSS class names (`comments-panel`, `sheet-handle`,
//! `comment-input`, `comment-list`, `comment-item`, `comment-replies`,
//! `reply-input`, `comment-action`) with
//! `features/spaces/pages/index/action_pages/discussion` so the visual
//! language is identical.
//!
//! All mutations (submit comment, submit reply, toggle comment like) go
//! through `use_post_detail` actions — components never import the
//! underlying `_handler` server functions. `list_comments_handler` for
//! lazy-loading replies is a read, so it stays inline per the hook rule's
//! non-mutating-read exception.

use crate::common::components::mention_autocomplete::{MentionAutocomplete, MentionInsert};
use crate::common::utils::mention::{parse_mention_segments, ContentSegment};
use crate::features::posts::controllers::dto::PostCommentResponse;
use crate::features::posts::hooks::{use_post_detail, UsePostDetail};
use crate::features::posts::*;

use super::i18n::PostDetailSyndicatedTranslate;

#[component]
pub fn PostCommentsPanel(post_id: FeedPartition) -> Element {
    let tr: PostDetailSyndicatedTranslate = use_translate();

    let UsePostDetail {
        detail,
        mut comment_text,
        mut tracked_mentions,
        is_submitting,
        members,
        mut submit_comment,
        ..
    } = use_post_detail(post_id.clone())?;

    let snapshot = detail();
    let total = snapshot.post.as_ref().map(|p| p.comments).unwrap_or(0);

    // Top-level comments only — replies are fetched lazily inside
    // `CommentItem` when the user expands them.
    let comments: Vec<PostCommentResponse> = {
        let mut out: Vec<PostCommentResponse> = Vec::new();
        for c in snapshot.comments.iter() {
            if c.parent_comment_sk.is_none() && !out.iter().any(|x| x.sk == c.sk) {
                out.push(c.clone());
            }
        }
        out
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
                                onclick: move |_| submit_comment.call(),
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
                            post_id: post_id.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentItem(comment: PostCommentResponse, post_id: FeedPartition) -> Element {
    let tr: PostDetailSyndicatedTranslate = use_translate();

    let UsePostDetail {
        members,
        replies_by_comment,
        mut toggle_comment_like,
        mut submit_reply,
        mut load_replies,
        ..
    } = use_post_detail(post_id)?;

    // Per-item UI state that never crosses scope boundaries: local input
    // (text + tracked mentions), show/hide toggle, and per-item
    // liked/likes for optimistic heart clicks. The replies LIST itself
    // lives in the hook's shared `replies_by_comment` map so the
    // load_replies / submit_reply actions mutate it without pulling any
    // child-owned signals up into their root-scope bodies.
    let mut show_replies = use_signal(|| false);
    let mut reply_text = use_signal(String::new);
    let mut reply_tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);

    let time_ago = format_time_ago(comment.updated_at);
    let mut liked = use_signal(|| comment.liked);
    let mut likes = use_signal(|| comment.likes as i64);
    let reply_count = comment.replies;

    let comment_sk = comment.sk.clone();
    let sk_str = comment_sk.to_string();

    // Derived replies slice for THIS comment — reads from the shared map
    // keyed by sk. Re-renders when any action writes to that key.
    let sk_str_for_memo = sk_str.clone();
    let replies = use_memo(move || {
        replies_by_comment
            .read()
            .get(&sk_str_for_memo)
            .cloned()
            .unwrap_or_default()
    });

    // Lazy-load replies on first expand. `load_replies` is idempotent
    // (skips when the sk is already in `replies_loaded`), so re-opening
    // the same comment doesn't refetch.
    let load_sk = comment_sk.clone();
    use_effect(move || {
        if show_replies() && reply_count > 0 {
            load_replies.call(load_sk.clone());
        }
    });

    // Optimistic UI for like lives here — `liked`/`likes` are owned by
    // this component, flipped locally, then the shared action writes to
    // the server. Passing the signals INTO the action would cross scope
    // boundaries (the action runs in the root hook scope).
    let like_sk = comment_sk.clone();
    let on_like = move |_| {
        let next = !liked();
        let prev = likes();
        liked.set(next);
        likes.set((prev + if next { 1 } else { -1 }).max(0));
        toggle_comment_like.call(like_sk.clone(), next);
    };

    let reply_sk = comment_sk.clone();
    let on_submit_reply = move |_| {
        let raw = reply_text().trim().to_string();
        if raw.is_empty() {
            return;
        }
        let mentions = reply_tracked_mentions.read().clone();
        // Local optimistic clear — these signals stay in CommentItem's
        // scope and are not handed to the action.
        reply_text.set(String::new());
        reply_tracked_mentions.set(Vec::new());
        submit_reply.call(reply_sk.clone(), raw, mentions);
    };

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
                            onclick: move |_| show_replies.set(!show_replies()),
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
                // otherwise long reply lists push the input below the fold.
                div { class: "reply-input",
                    MentionAutocomplete {
                        text: reply_text,
                        on_select: {
                            let mut reply_text = reply_text;
                            let mut reply_tracked_mentions = reply_tracked_mentions;
                            move |insert: MentionInsert| {
                                let mut val = reply_text();
                                if insert.start_offset <= val.len() && insert.end_offset <= val.len() {
                                    val.replace_range(
                                        insert.start_offset..insert.end_offset,
                                        &insert.display_text,
                                    );
                                    reply_text.set(val);
                                    reply_tracked_mentions
                                        .write()
                                        .push((insert.display_name, insert.user_pk));
                                }
                            }
                        },
                        members,
                        input {
                            class: "reply-input__field",
                            placeholder: "{tr.reply_placeholder}",
                            value: "{reply_text}",
                            oninput: {
                                let mut reply_text = reply_text;
                                move |e: Event<FormData>| reply_text.set(e.value())
                            },
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
    let UsePostDetail {
        mut toggle_comment_like,
        ..
    } = try_use_context::<UsePostDetail>()
        .expect("use_post_detail must be initialized by a parent component");

    let time_ago = format_time_ago(reply.updated_at);
    // Local per-item signals owned by THIS ReplyItem's scope so the
    // optimistic flip doesn't cross scope boundaries.
    let mut liked = use_signal(|| reply.liked);
    let mut likes = use_signal(|| reply.likes as i64);

    let reply_sk = reply.sk.clone();
    let on_like = move |_| {
        let next = !liked();
        let prev = likes();
        liked.set(next);
        likes.set((prev + if next { 1 } else { -1 }).max(0));
        toggle_comment_like.call(reply_sk.clone(), next);
    };

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
