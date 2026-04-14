use crate::common::components::mention_autocomplete::{
    MentionAutocomplete, MentionCandidate, MentionInsert,
};
use crate::common::utils::mention::{apply_mention_markup, parse_mention_segments, ContentSegment};
use crate::features::spaces::pages::actions::actions::discussion::controllers::{
    add_comment, get_discussion_detail, like_comment, list_comments, list_replies, reply_comment,
    AddCommentRequest, LikeCommentRequest, ReplyCommentRequest,
};
use crate::features::spaces::pages::actions::actions::discussion::{
    DiscussionCommentResponse, DiscussionStatus, SpacePostCommentTargetEntityType,
};
use crate::features::spaces::pages::index::action_pages::discussion::*;
use crate::features::spaces::pages::index::action_pages::quiz::ActiveActionOverlaySignal;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::controllers::list_space_members;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use crate::features::spaces::space_common::providers::use_space_context;

#[component]
pub fn DiscussionArenaPage(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let mut toast = use_toast();
    let mut space_ctx = use_space_context();
    let role = use_space_role()();
    let space = use_space()();

    let disc_loader = use_loader(move || {
        get_discussion_detail(space_id(), discussion_id())
    })?;
    let disc = disc_loader();
    let post = disc.post.clone();
    let space_action = disc.space_action.clone();

    let status = post.status();
    let is_in_progress = status == DiscussionStatus::InProgress;
    let can_respond = matches!(role, SpaceUserRole::Creator | SpaceUserRole::Participant);
    let can_execute = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        space_action.prerequisite,
        space.status,
        space.join_anytime,
    );
    let can_comment = can_respond && can_execute && is_in_progress;

    let mut comments_loader = use_loader(move || {
        list_comments(space_id(), discussion_id(), None)
    })?;
    let comments_data = comments_loader();
    let comments = comments_data.items.clone();

    let mut overlay: ActiveActionOverlaySignal = use_context();

    let mut comment_text = use_signal(String::new);
    let mut tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);

    let members_loader = use_loader(move || {
        list_space_members(space_id(), None)
    })?;
    let members: Vec<MentionCandidate> = members_loader()
        .items
        .into_iter()
        .map(|m| {
            let pk: Partition = m.user_id.clone().into();
            MentionCandidate {
                user_pk: pk.to_string(),
                display_name: m.display_name,
                username: m.username,
                profile_url: m.profile_url,
            }
        })
        .collect();
    let members = use_signal(|| members);

    let status_text = match status {
        DiscussionStatus::InProgress => tr.status_in_progress.to_string(),
        DiscussionStatus::NotStarted => tr.status_not_started.to_string(),
        DiscussionStatus::Finish => tr.status_finished.to_string(),
    };
    let status_class = match status {
        DiscussionStatus::InProgress => "topbar__status topbar__status--active",
        DiscussionStatus::NotStarted => "topbar__status topbar__status--not-started",
        DiscussionStatus::Finish => "topbar__status topbar__status--ended",
    };

    let created_date = {
        let secs = post.created_at / 1000;
        let dt = chrono::DateTime::from_timestamp(secs, 0).unwrap_or_default();
        dt.format("%b %d, %Y").to_string()
    };

    let on_back = move |_| {
        overlay.0.set(None);
    };

    let on_submit_comment = move |_| async move {
        let raw_text = comment_text().trim().to_string();
        if raw_text.is_empty() {
            return;
        }
        let content = apply_mention_markup(&raw_text, &tracked_mentions.read());
        let req = AddCommentRequest {
            content,
            images: vec![],
        };
        match add_comment(space_id(), discussion_id(), req).await {
            Ok(_) => {
                comments_loader.restart();
                space_ctx.actions.restart();
                comment_text.set(String::new());
                tracked_mentions.set(Vec::new());
                toast.info(tr.comment_success);
            }
            Err(err) => {
                tracing::error!("Failed to post comment: {:?}", err);
                toast.error(err);
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "discussion-arena",

            // ── Top bar ──────────────────────────────
            div { class: "topbar",
                div { class: "topbar__left",
                    button {
                        class: "topbar__back",
                        "data-testid": "discussion-arena-back",
                        onclick: on_back,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M19 12H5" }
                            path { d: "m12 19-7-7 7-7" }
                        }
                    }
                    span { class: "topbar__title", "{post.title}" }
                }
                div { class: "topbar__right",
                    span { class: "{status_class}", "{status_text}" }
                }
            }

            // ── Banners ──────────────────────────────
            if status == DiscussionStatus::Finish {
                div { class: "disc-banner disc-banner--warning", "{tr.discussion_ended}" }
            }
            if status == DiscussionStatus::NotStarted {
                div { class: "disc-banner disc-banner--info", "{tr.discussion_not_started}" }
            }

            // ── Split Layout ─────────────────────────
            div { class: "discussion-layout",

                // Left: Discussion Content
                div { class: "discussion-main",
                    div { class: "discussion-main__inner",

                        // Header
                        div { class: "disc-header",
                            span { class: "disc-header__type",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                                }
                                "{tr.discussion_label}"
                            }
                            h1 { class: "disc-header__title", "{post.title}" }
                            div { class: "disc-header__meta",
                                div { class: "disc-header__author",
                                    img {
                                        class: "disc-header__avatar",
                                        src: "{post.author_profile_url}",
                                        alt: "{post.author_display_name}",
                                    }
                                    span { class: "disc-header__author-name",
                                        "{post.author_display_name}"
                                    }
                                }
                                span { class: "disc-header__separator" }
                                span { class: "disc-header__date", "{created_date}" }
                                if !post.category_name.is_empty() {
                                    span { class: "disc-header__type", "{post.category_name}" }
                                }
                            }
                        }

                        // Body
                        if !post.html_contents.is_empty() {
                            div { class: "disc-body",
                                div {
                                    class: "disc-body__content",
                                    dangerous_inner_html: "{post.html_contents}",
                                }
                            }
                        }

                        // Files
                        if !post.files.is_empty() {
                            div { class: "disc-files",
                                span { class: "disc-files__label", "{tr.attachments_label}" }
                                div { class: "disc-files__grid",
                                    for file in post.files.iter() {
                                        a {
                                            class: "file-card",
                                            key: "{file.id}",
                                            href: file.url.clone().unwrap_or_default(),
                                            target: "_blank",
                                            download: "{file.name}",
                                            div { class: "file-card__icon",
                                                svg {
                                                    view_box: "0 0 24 24",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_width: "2",
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                                    polyline { points: "14 2 14 8 20 8" }
                                                }
                                            }
                                            div { class: "file-card__info",
                                                div { class: "file-card__name", "{file.name}" }
                                                div { class: "file-card__size", "{file.size}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Right: Comments Panel (bottom sheet on mobile)
                div { class: "comments-panel", id: "discussion-comments-sheet",
                    // Sheet handle (visible on mobile only)
                    div { class: "sheet-handle",
                        div { class: "sheet-handle__bar" }
                        div { class: "sheet-handle__row",
                            div { class: "sheet-handle__left",
                                span { class: "sheet-handle__title", "{tr.comments_title}" }
                                span { class: "sheet-handle__count", "{post.comments}" }
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
                    // Desktop header
                    div { class: "comments-panel__header",
                        span { class: "comments-panel__title", "{tr.comments_title}" }
                        span { class: "comments-panel__count", "{post.comments}" }
                    }

                    // Comment Input
                    if can_comment {
                        div { class: "comment-input",
                            div { class: "comment-input__wrapper",
                                div { class: "comment-input__body",
                                    MentionAutocomplete {
                                        text: comment_text,
                                        on_select: move |insert: MentionInsert| {
                                            let mut val = comment_text();
                                            if insert.start_offset <= val.len()
                                                && insert.end_offset <= val.len()
                                            {
                                                val.replace_range(
                                                    insert.start_offset..insert.end_offset,
                                                    &insert.display_text,
                                                );
                                                comment_text.set(val);
                                                tracked_mentions.write().push((
                                                    insert.display_name,
                                                    insert.user_pk,
                                                ));
                                            }
                                        },
                                        members,
                                        textarea {
                                            class: "comment-input__textarea",
                                            placeholder: "{tr.comment_placeholder}",
                                            rows: "2",
                                            value: "{comment_text}",
                                            oninput: move |e| {
                                                comment_text.set(e.value());
                                            },
                                        }
                                    }
                                    div { class: "comment-input__footer",
                                        button {
                                            class: "comment-input__submit",
                                            disabled: comment_text().trim().is_empty(),
                                            onclick: on_submit_comment,
                                            "{tr.post_btn}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Scrollable comments
                    div { class: "comments-scroll",
                        div { class: "comment-list",
                            for comment in comments.iter() {
                                CommentItem {
                                    key: "{comment.sk}",
                                    comment: comment.clone(),
                                    space_id,
                                    discussion_id,
                                    can_comment,
                                    members,
                                    comments_loader,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Comment Item ────────────────────────────────────
#[component]
fn CommentItem(
    comment: DiscussionCommentResponse,
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    can_comment: bool,
    members: ReadSignal<Vec<MentionCandidate>>,
    comments_loader: Loader<ListResponse<DiscussionCommentResponse>>,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let mut comments_loader = comments_loader;
    let mut toast = use_toast();

    let mut show_replies = use_signal(|| false);
    let mut reply_text = use_signal(String::new);
    let mut reply_tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let mut replies: Signal<Vec<DiscussionCommentResponse>> = use_signal(Vec::new);

    let time_ago = format_time_ago(comment.created_at);
    let comment_sk = use_signal(|| comment.sk.clone());
    let liked = comment.liked;
    let likes = comment.likes;
    let reply_count = comment.replies;

    let on_like = move |_| async move {
        let target_sk: SpacePostCommentTargetEntityType = comment_sk().into();
        let req = LikeCommentRequest { like: !liked };
        match like_comment(space_id(), discussion_id(), target_sk, req).await {
            Ok(_) => {
                comments_loader.restart();
            }
            Err(err) => {
                tracing::error!("Failed to toggle like: {:?}", err);
                toast.error(err);
            }
        }
    };

    let on_toggle_replies = move |_| async move {
        let next = !show_replies();
        show_replies.set(next);
        if next && replies().is_empty() && reply_count > 0 {
            let comment_sk_entity: SpacePostCommentEntityType =
                comment_sk().try_into().unwrap_or_default();
            match list_replies(space_id(), discussion_id(), comment_sk_entity, None).await {
                Ok(data) => {
                    replies.set(data.items);
                }
                Err(err) => {
                    tracing::error!("Failed to load replies: {:?}", err);
                }
            }
        }
    };

    let on_submit_reply = move |_| async move {
        let raw_text = reply_text().trim().to_string();
        if raw_text.is_empty() {
            return;
        }
        let content = apply_mention_markup(&raw_text, &reply_tracked_mentions.read());
        let comment_sk_entity: SpacePostCommentEntityType =
            comment_sk().try_into().unwrap_or_default();
        let req = ReplyCommentRequest {
            content,
            images: vec![],
        };
        match reply_comment(space_id(), discussion_id(), comment_sk_entity, req).await {
            Ok(new_reply) => {
                let mut current = replies();
                current.insert(0, new_reply);
                replies.set(current);
                reply_text.set(String::new());
                reply_tracked_mentions.set(Vec::new());
                comments_loader.restart();
                toast.info(tr.reply_success);
            }
            Err(err) => {
                tracing::error!("Failed to post reply: {:?}", err);
                toast.error(err);
            }
        }
    };

    rsx! {
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
                                span { class: "font-medium text-primary", "@{display_name}" }
                            },
                        }
                    }
                }
                div { class: "comment-item__actions",
                    button {
                        class: if liked { "comment-action comment-action--liked" } else { "comment-action" },
                        onclick: on_like,
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
                    if can_comment {
                        button {
                            class: "comment-action comment-action--reply",
                            onclick: on_toggle_replies,
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
        }

        // Replies toggle
        if reply_count > 0 {
            button { class: "reply-toggle", onclick: on_toggle_replies,
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

        // Replies list
        if show_replies() {
            div { class: "comment-replies",
                for reply in replies().iter() {
                    div { class: "comment-item", key: "{reply.sk}",
                        img {
                            class: "comment-item__avatar",
                            src: "{reply.author_profile_url}",
                            alt: "{reply.author_display_name}",
                        }
                        div { class: "comment-item__body",
                            div { class: "comment-item__top",
                                span { class: "comment-item__name", "{reply.author_display_name}" }
                                span { class: "comment-item__time",
                                    "{format_time_ago(reply.created_at)}"
                                }
                            }
                            div { class: "comment-item__text",
                                for segment in parse_mention_segments(&reply.content) {
                                    match segment {
                                        ContentSegment::Text(t) => rsx! {
                                            span { "{t}" }
                                        },
                                        ContentSegment::Mention { display_name, .. } => rsx! {
                                            span { class: "font-medium text-primary", "@{display_name}" }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Reply input
            if can_comment {
                div { class: "reply-input",
                    MentionAutocomplete {
                        text: reply_text,
                        on_select: move |insert: MentionInsert| {
                            let mut val = reply_text();
                            if insert.start_offset <= val.len()
                                && insert.end_offset <= val.len()
                            {
                                val.replace_range(
                                    insert.start_offset..insert.end_offset,
                                    &insert.display_text,
                                );
                                reply_text.set(val);
                                reply_tracked_mentions.write().push((
                                    insert.display_name,
                                    insert.user_pk,
                                ));
                            }
                        },
                        members,
                        input {
                            class: "reply-input__field",
                            placeholder: "{tr.reply_placeholder}",
                            value: "{reply_text}",
                            oninput: move |e| {
                                reply_text.set(e.value());
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
            }
        }
    }
}

// ── Helpers ──────────────────────────────────────────

fn format_time_ago(timestamp_millis: i64) -> String {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff_secs = (now - timestamp_millis) / 1000;

    if diff_secs < 60 {
        "just now".to_string()
    } else if diff_secs < 3600 {
        let mins = diff_secs / 60;
        format!("{}m ago", mins)
    } else if diff_secs < 86400 {
        let hours = diff_secs / 3600;
        format!("{}h ago", hours)
    } else {
        let days = diff_secs / 86400;
        format!("{}d ago", days)
    }
}
