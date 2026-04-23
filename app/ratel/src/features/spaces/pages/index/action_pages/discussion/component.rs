use crate::common::components::image_upload_preview::{ImageUploadPreview, PendingImage};
use crate::common::components::mention_autocomplete::{
    MentionAutocomplete, MentionCandidate, MentionInsert,
};
use crate::common::components::paste_image_uploader;
use crate::common::components::CommentImageGrid;
use crate::common::utils::mention::{apply_mention_markup, parse_mention_segments, ContentSegment};
use crate::features::spaces::pages::actions::actions::discussion::controllers::list_replies;
use crate::features::spaces::pages::actions::actions::discussion::{
    DiscussionCommentResponse, DiscussionStatus, SpacePostCommentTargetEntityType,
};
use crate::features::spaces::pages::index::action_pages::discussion::*;
use crate::features::spaces::pages::index::action_pages::quiz::ActiveActionOverlaySignal;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use crate::features::spaces::space_common::providers::use_space_context;

// `value=""` from Rust doesn't fire `input`, so the JS autogrow listener
// won't shrink the textarea on its own — call into JS to reset.
#[cfg(not(feature = "server"))]
use crate::common::wasm_bindgen::prelude::*;

#[cfg(not(feature = "server"))]
#[wasm_bindgen(module = "/src/features/spaces/pages/index/action_pages/discussion/script.js")]
extern "C" {
    #[wasm_bindgen(js_name = resetComposerHeight)]
    fn reset_composer_height_js();
}

fn reset_composer_height() {
    #[cfg(not(feature = "server"))]
    reset_composer_height_js();
}

// Matches the 500px CSS breakpoint that switches the arena into the
// bottom-sheet layout.
#[cfg(not(feature = "server"))]
fn is_arena_mobile_viewport() -> bool {
    let width_px = web_sys::window()
        .and_then(|w| w.inner_width().ok())
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    width_px > 0.0 && width_px <= 500.0
}

#[component]
pub fn SpaceDiscussionPage(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
) -> Element {
    let space_id_sig: ReadSignal<SpacePartition> = use_signal(|| space_id.clone()).into();
    let discussion_id_sig: ReadSignal<SpacePostEntityType> =
        use_signal(|| discussion_id.clone()).into();
    rsx! {
        DiscussionArenaPage {
            space_id: space_id_sig,
            discussion_id: discussion_id_sig,
            target_comment_id: None,
        }
    }
}

/// Deep-link variant — `comment_id` drives the scroll-to + pulse effect
/// in `DiscussionArenaPage`. Path param (not query/fragment) because
/// Dioxus Router strips both on hydration.
#[component]
pub fn SpaceDiscussionCommentPage(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
    comment_id: String,
) -> Element {
    let space_id_sig: ReadSignal<SpacePartition> = use_signal(|| space_id.clone()).into();
    let discussion_id_sig: ReadSignal<SpacePostEntityType> =
        use_signal(|| discussion_id.clone()).into();
    rsx! {
        DiscussionArenaPage {
            space_id: space_id_sig,
            discussion_id: discussion_id_sig,
            target_comment_id: Some(comment_id),
        }
    }
}

/// In-sheet thread view for a single parent comment + its replies.
/// Reads parent/replies loaders from the controller so swapping into
/// the thread is a background refresh, not a fresh mount that would
/// suspend up to the overlay boundary.
#[component]
fn ReplyThreadView(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let role = use_space_role()();
    let space = use_space()();

    let arena = use_discussion_arena(space_id, discussion_id)?;
    let UseDiscussionArena {
        mut parent_loader,
        mut replies_loader,
        members,
        mut mention_query_raw,
        mut like_comment,
        ..
    } = arena;
    let parent = parent_loader();

    // Local mirror of the loader so optimistic mutations have a writable
    // source. Re-synced when the loader's identity changes.
    let mut replies: Signal<Vec<DiscussionCommentResponse>> =
        use_signal(move || replies_loader().items);
    use_effect(move || {
        let fresh = replies_loader().items;
        if replies.peek().len() != fresh.len() {
            replies.set(fresh);
        } else if !replies
            .peek()
            .iter()
            .zip(fresh.iter())
            .all(|(a, b)| a.sk == b.sk)
        {
            replies.set(fresh);
        }
    });

    let is_space_open = space.status != Some(SpaceStatus::Finished);
    let can_respond = matches!(role, SpaceUserRole::Creator | SpaceUserRole::Participant);
    let can_comment = can_respond && is_space_open;

    let mut reply_text = use_signal(String::new);
    let mut reply_tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let mut reply_pending_images: Signal<Vec<PendingImage>> = use_signal(Vec::new);

    let on_mention_query_change = move |q: Option<String>| {
        mention_query_raw.set(q);
    };
    let on_composer_focus = move || {
        if mention_query_raw.peek().is_none() {
            mention_query_raw.set(Some(String::new()));
        }
    };

    let parent_sk_for_like = parent.sk.clone();
    let on_parent_like = move |_| {
        let sk_str = parent_sk_for_like.to_string();
        let next = !arena.effective_liked(&parent_loader());
        like_comment.call(sk_str, next);
    };

    // Read `parent_loader` inside so the memo rebuilds when the cached
    // loader resolves with the real parent.
    let reply_priority = use_memo(move || {
        let p = parent_loader();
        let parent_author_pk = p.author_pk.to_string();
        let parent_content = p.content.clone();
        let replies_vec = replies.read();
        let tuples: Vec<(String, String)> = replies_vec
            .iter()
            .rev()
            .map(|r| (r.author_pk.to_string(), r.content.clone()))
            .collect();
        let refs: Vec<(&str, &str)> = tuples
            .iter()
            .map(|(a, c)| (a.as_str(), c.as_str()))
            .collect();
        crate::common::utils::mention::build_mention_priority(
            Some((parent_author_pk.as_str(), parent_content.as_str())),
            &refs,
        )
    });
    let reply_priority: ReadSignal<Vec<String>> = reply_priority.into();

    let parent_sk_for_submit = parent.sk.to_string();
    let mut reply_comment_action = arena.reply_comment;
    let on_submit_reply = move |_| {
        let raw_text = reply_text().trim().to_string();
        let images: Vec<String> = reply_pending_images
            .read()
            .iter()
            .filter_map(|img| img.remote_url.clone())
            .collect();
        if raw_text.is_empty() && images.is_empty() {
            return;
        }
        let content = apply_mention_markup(&raw_text, &reply_tracked_mentions.read());
        reply_comment_action.call(parent_sk_for_submit.clone(), content, images);
        reply_text.set(String::new());
        reply_tracked_mentions.set(Vec::new());
        reply_pending_images.set(Vec::new());
    };

    let parent_time_ago = format_time_ago(parent.created_at);

    rsx! {
        div { class: "reply-thread", "data-testid": "reply-thread",
            div { class: "reply-thread__scroll",
                div { class: "reply-thread__parent",
                    div { class: "comment-item",
                        img {
                            class: "comment-item__avatar",
                            src: "{parent.author_profile_url}",
                            alt: "{parent.author_display_name}",
                        }
                        div { class: "comment-item__body",
                            div { class: "comment-item__top",
                                span { class: "comment-item__name", "{parent.author_display_name}" }
                                span { class: "comment-item__time", "{parent_time_ago}" }
                            }
                            CommentText { content: parent.content.clone() }
                            CommentImageGrid { images: parent.images.clone() }
                            div { class: "comment-item__actions",
                                button {
                                    class: "comment-action",
                                    "aria-pressed": arena.effective_liked(&parent),
                                    disabled: like_comment.pending(),
                                    onclick: on_parent_like,
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: if arena.effective_liked(&parent) { "currentColor" } else { "none" },
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" }
                                    }
                                    span { "{arena.effective_likes(&parent)}" }
                                }
                            }
                        }
                    }
                }

                div { class: "reply-thread__list",
                    for reply in replies().iter().filter(|r| !arena.is_deleted(r)) {
                        ReplyItem {
                            key: "{reply.sk}",
                            reply: reply.clone(),
                            space_id,
                            discussion_id,
                            replies,
                            can_comment,
                        }
                    }
                }
            }

            if can_comment {
                div { class: "reply-thread__composer",
                    CommentComposer {
                        text: reply_text,
                        tracked_mentions: reply_tracked_mentions,
                        pending_images: reply_pending_images,
                        members,
                        on_submit: on_submit_reply,
                        placeholder: tr.reply_placeholder.to_string(),
                        compact: true,
                        disabled: reply_text().trim().is_empty()
                                                                                                                                                                                                                                                                                                                                                    && reply_pending_images.read().is_empty(),
                        on_mention_query_change,
                        on_composer_focus,
                        priority_user_pks: reply_priority,
                    }
                }
            }
        }
    }
}

#[component]
pub fn DiscussionArenaPage(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    #[props(default)] target_comment_id: Option<String>,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let mut space_ctx = use_space_context();
    let role = use_space_role()();
    let space = use_space()();

    let arena = use_discussion_arena(space_id, discussion_id)?;
    let mut comments_query = arena.comments_query;
    let polled_new = arena.polled_new;
    let mut active_reply_thread = arena.active_reply_thread;
    let mut sheet_expanded = arena.sheet_expanded;
    let mut mention_query_raw = arena.mention_query_raw;
    let members = arena.members;
    let top_priority = arena.top_priority;
    let sort_tick = arena.sort_tick;

    let disc = arena.disc_loader.clone()();
    let post = disc.post.clone();
    let space_action = disc.space_action.clone();

    let status = crate::features::spaces::pages::actions::actions::discussion::SpacePost::status_from(
        space_action.status.as_ref(),
    );
    let is_in_progress = status == DiscussionStatus::InProgress;
    let can_respond = matches!(role, SpaceUserRole::Creator | SpaceUserRole::Participant);
    let can_execute = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        space_action.prerequisite,
        space.status,
        space_action.status.as_ref(),
        true,
        space.join_anytime,
    );
    // Creators can comment on their own discussion regardless of publish state
    // (preview / authoring). Non-Creators require in-progress status.
    let is_creator = matches!(role, SpaceUserRole::Creator);
    let can_comment = can_respond && can_execute && (is_creator || is_in_progress);

    // Re-runs whenever:
    //  - `comments_query` accumulates new pages or refresh resolves
    //  - `polled_new` gains a new entry
    //  - `sort_tick` ticks (every 5s, drives time-decay reorder)
    let comments: Memo<Vec<DiscussionCommentResponse>> = use_memo(move || {
        // Touch sort_tick so the memo re-evaluates each tick.
        let _ = sort_tick();
        let now = crate::common::utils::time::get_now_timestamp();
        merge_and_rank_comments(comments_query.items(), polled_new(), now)
    });

    // `overlay_ctx` is only present when mounted as the arena overlay;
    // `on_back` falls back to `nav.go_back()` for the standalone route.
    let overlay_ctx: Option<ActiveActionOverlaySignal> = try_consume_context();
    let nav = use_navigator();

    let deep_link_target: Signal<Option<String>> = use_signal(|| target_comment_id.clone());
    let mut deep_link_done: Signal<bool> = use_signal(|| target_comment_id.is_none());
    // Hook indices must match between SSR and hydration; gate only the
    // browser-API body, not the `use_effect` call itself.
    use_effect(move || {
        let _ = comments_query.items();
        if deep_link_done() {
            return;
        }
        let Some(target_id) = deep_link_target() else {
            deep_link_done.set(true);
            return;
        };
        #[cfg(feature = "web")]
        {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };
            let Some(el) = document.get_element_by_id(&target_id) else {
                return;
            };
            let opts = web_sys::ScrollIntoViewOptions::new();
            opts.set_behavior(web_sys::ScrollBehavior::Smooth);
            opts.set_block(web_sys::ScrollLogicalPosition::Center);
            let _ = el.scroll_into_view_with_scroll_into_view_options(&opts);
            deep_link_done.set(true);
        }
        #[cfg(not(feature = "web"))]
        {
            let _ = target_id;
        }
    });

    let mut comment_text = use_signal(String::new);
    let mut tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let mut pending_images: Signal<Vec<PendingImage>> = use_signal(Vec::new);

    let on_mention_query_change = move |q: Option<String>| {
        mention_query_raw.set(q);
    };
    let on_composer_focus = move || {
        if mention_query_raw.peek().is_none() {
            mention_query_raw.set(Some(String::new()));
        }
    };

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
        if let Some(mut o) = overlay_ctx {
            o.0.set(None);
        } else {
            nav.replace(Route::SpaceIndexPage {
                space_id: space_id(),
            });
        }
    };

    let mut add_comment_action = arena.add_comment;
    let mut on_submit_comment = move |_| {
        let raw_text = comment_text().trim().to_string();
        let images: Vec<String> = pending_images
            .read()
            .iter()
            .filter_map(|img| img.remote_url.clone())
            .collect();
        if raw_text.is_empty() && images.is_empty() {
            return;
        }
        let content = apply_mention_markup(&raw_text, &tracked_mentions.read());
        add_comment_action.call(content, images);
        space_ctx.actions.restart();
        comment_text.set(String::new());
        tracked_mentions.set(Vec::new());
        pending_images.set(Vec::new());
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { r#type: "module", src: asset!("./script.js") }

        div {
            class: "discussion-arena",
            "data-testid": "discussion-arena-overlay",
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

            if status == DiscussionStatus::Finish {
                div { class: "disc-banner disc-banner--warning", "{tr.discussion_ended}" }
            }
            if status == DiscussionStatus::NotStarted {
                div { class: "disc-banner disc-banner--info", "{tr.discussion_not_started}" }
            }

            div { class: "discussion-layout",
                div { class: "discussion-main",
                    div { class: "discussion-main__inner",
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

                        if !post.html_contents.is_empty() {
                            div { class: "disc-body",
                                div {
                                    class: "disc-body__content",
                                    dangerous_inner_html: "{post.html_contents}",
                                }
                            }
                        }

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

                // Drag handle for resizing the comments panel horizontally.
                // Width is JS-owned (continuous mouse value) — Dioxus never
                // sets `style` on the panel so the inline width survives
                // re-renders. Hidden on mobile via media query (panel becomes
                // a bottom sheet).
                div {
                    class: "comments-panel__resizer",
                    id: "comments-panel-resizer",
                    role: "separator",
                    aria_label: "Resize comments panel",
                    aria_orientation: "vertical",
                }

                // Sheet handle expand state is Dioxus-owned (data-expanded)
                // because the JS-owned class was being clobbered by panel
                // re-renders.
                div {
                    class: "comments-panel",
                    id: "discussion-comments-sheet",
                    "data-expanded": sheet_expanded(),
                    div {
                        class: "sheet-handle",
                        onclick: move |_| sheet_expanded.toggle(),
                        div { class: "sheet-handle__bar" }
                        div { class: "sheet-handle__row",
                            if active_reply_thread().is_some() {
                                button {
                                    class: "sheet-handle__back",
                                    "data-testid": "reply-thread-back",
                                    aria_label: "{tr.replies_back_aria}",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        active_reply_thread.set(None);
                                    },
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        polyline { points: "15 18 9 12 15 6" }
                                    }
                                }
                            } else {
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
                    }

                    if let Some(thread_id) = active_reply_thread() {
                        ReplyThreadView {
                            key: "{thread_id}",
                            space_id,
                            discussion_id,
                        }
                    } else {
                        div { class: "comments-panel__body",
                            div { class: "comments-panel__header",
                                span { class: "comments-panel__title", "{tr.comments_title}" }
                                span { class: "comments-panel__count", "{post.comments}" }
                            }

                            if can_comment {
                                CommentComposer {
                                    text: comment_text,
                                    tracked_mentions,
                                    pending_images,
                                    members,
                                    on_submit: move |_| on_submit_comment(()),
                                    placeholder: tr.comment_placeholder.to_string(),
                                    disabled: comment_text().trim().is_empty()
                                                                                                                                                                                                                                                                                                                                                                                                        && pending_images.read().is_empty(),
                                    on_mention_query_change,
                                    on_composer_focus,
                                    priority_user_pks: top_priority,
                                }
                            }

                            div { class: "comments-scroll",
                                div { class: "comment-list",
                                    for comment in comments().iter().filter(|c| !arena.is_deleted(c)) {
                                        CommentItem {
                                            key: "{comment.sk}",
                                            comment: comment.clone(),
                                            space_id,
                                            discussion_id,
                                            can_comment,
                                            deep_link_target,
                                        }
                                    }
                                    {comments_query.more_element()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Shared composer for the top-level box and per-comment reply box.
/// `compact` toggles the nested-context chrome via `data-compact`.
#[component]
fn CommentComposer(
    text: Signal<String>,
    tracked_mentions: Signal<Vec<(String, String)>>,
    pending_images: Signal<Vec<PendingImage>>,
    members: ReadSignal<Vec<MentionCandidate>>,
    on_submit: EventHandler<()>,
    placeholder: String,
    #[props(default)] compact: bool,
    disabled: bool,
    #[props(default)] on_mention_query_change: EventHandler<Option<String>>,
    #[props(default)] on_composer_focus: EventHandler<()>,
    #[props(default)] priority_user_pks: ReadSignal<Vec<String>>,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();

    let on_paste = move |evt: ClipboardEvent| {
        #[cfg(feature = "web")]
        paste_image_uploader::handle_paste_event(&evt, pending_images);
        #[cfg(not(feature = "web"))]
        let _ = evt;
    };

    let on_mention_select = move |insert: MentionInsert| {
        let mut val = text();
        if insert.start_offset <= val.len() && insert.end_offset <= val.len() {
            val.replace_range(
                insert.start_offset..insert.end_offset,
                &insert.display_text,
            );
            text.set(val);
            tracked_mentions
                .write()
                .push((insert.display_name, insert.user_pk));
        }
    };

    use_effect(move || {
        if text().is_empty() {
            reset_composer_height();
        }
    });

    // Ctrl/Cmd+Enter submits; plain Enter falls through to default newline.
    let on_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter
            && (evt.modifiers().contains(Modifiers::CONTROL)
                || evt.modifiers().contains(Modifiers::META))
        {
            evt.prevent_default();
            evt.stop_propagation();
            on_submit.call(());
        }
    };

    // Keep `.comment-input__*` hook classes on the top-level variant so
    // existing Playwright selectors keep resolving.
    let textarea_class = if compact {
        "reply-input__field"
    } else {
        "reply-input__field comment-input__textarea"
    };
    let send_class = if compact {
        "reply-input__send"
    } else {
        "reply-input__send comment-input__submit"
    };
    rsx! {
        div {
            class: "comment-input",
            "data-compact": compact,
            onpaste: on_paste,
            ImageUploadPreview { images: pending_images }
            div { class: "reply-input",
                MentionAutocomplete {
                    text,
                    on_select: on_mention_select,
                    members,
                    on_query_change: move |q| on_mention_query_change.call(q),
                    priority_user_pks,
                    textarea {
                        class: "{textarea_class}",
                        placeholder: "{placeholder}",
                        rows: "1",
                        value: "{text}",
                        oninput: move |e| {
                            text.set(e.value());
                        },
                        onkeydown: on_keydown,
                        onfocus: move |_| on_composer_focus.call(()),
                        onmounted: move |e: MountedEvent| async move {
                            if compact {
                                let _ = e.set_focus(true).await;
                            }
                        },
                    }
                }
                button {
                    class: "{send_class}",
                    disabled,
                    onclick: move |_| on_submit.call(()),
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

/// Renders comment content with mention highlighting and a "Show more"
/// toggle when the rendered text exceeds 3 visual lines. Used by the
/// parent in `ReplyThreadView`, top-level `CommentItem`, and `ReplyItem`
/// so the truncation behavior stays consistent everywhere.
///
/// Truncation is purely visual (CSS `-webkit-line-clamp: 3` when
/// `data-expanded="false"`); JS measures `scrollHeight > clientHeight`
/// and sets `data-truncatable="true"` to reveal the toggle button. This
/// matches what the user actually sees regardless of viewport width or
/// language, instead of a brittle character count.
#[component]
fn CommentText(content: String) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let mut expanded = use_signal(|| false);

    rsx! {
        div { class: "comment-text", "data-expanded": expanded(),
            div { class: "comment-item__text",
                for segment in parse_mention_segments(&content) {
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
            // Always rendered; CSS hides it unless JS sets
            // `data-truncatable="true"` on the wrapper after measuring.
            button {
                class: "comment-item__expand",
                onclick: move |_| expanded.toggle(),
                if expanded() {
                    "{tr.show_less}"
                } else {
                    "{tr.show_more}"
                }
            }
        }
    }
}

#[component]
fn CommentItem(
    comment: DiscussionCommentResponse,
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    can_comment: bool,
    deep_link_target: ReadSignal<Option<String>>,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();

    let arena = use_discussion_arena(space_id, discussion_id)?;
    let UseDiscussionArena {
        mut polled_new,
        members,
        mut mention_query_raw,
        mut active_reply_thread,
        mut sheet_expanded,
        ..
    } = arena;
    let on_mention_query_change = move |q: Option<String>| {
        mention_query_raw.set(q);
    };
    let on_composer_focus = move || {
        if mention_query_raw.peek().is_none() {
            mention_query_raw.set(Some(String::new()));
        }
    };

    let mut show_replies = use_signal(|| false);
    let mut reply_text = use_signal(String::new);
    let mut reply_pending_images: Signal<Vec<PendingImage>> = use_signal(Vec::new);
    let mut reply_tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let mut replies: Signal<Vec<DiscussionCommentResponse>> = use_signal(Vec::new);

    let time_ago = format_time_ago(comment.created_at);
    let comment_sk = use_signal(|| comment.sk.clone());
    let reply_count = comment.replies;

    let parent_author_pk = comment.author_pk.to_string();
    let parent_content = comment.content.clone();
    let reply_priority = use_memo(move || {
        let replies_vec = replies.read();
        let tuples: Vec<(String, String)> = replies_vec
            .iter()
            .rev()
            .map(|r| (r.author_pk.to_string(), r.content.clone()))
            .collect();
        let refs: Vec<(&str, &str)> = tuples
            .iter()
            .map(|(a, c)| (a.as_str(), c.as_str()))
            .collect();
        crate::common::utils::mention::build_mention_priority(
            Some((parent_author_pk.as_str(), parent_content.as_str())),
            &refs,
        )
    });
    let reply_priority: ReadSignal<Vec<String>> = reply_priority.into();

    // DOM id matches the deep-link URL fragment format.
    let comment_dom_id: String = SpacePostCommentEntityType::try_from(comment.sk.clone())
        .map(|e| e.0)
        .unwrap_or_else(|_| comment.sk.to_string());

    let mut like_comment_action = arena.like_comment;
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let is_own = user_ctx
        .read()
        .user
        .as_ref()
        .map(|u| u.pk == comment.author_pk)
        .unwrap_or(false);

    let mut menu_open = use_signal(|| false);
    let mut editing = use_signal(|| false);
    let mut edit_text = use_signal(|| comment.content.clone());
    let original_content = comment.content.clone();

    let comment_for_like = comment.clone();
    let on_like = move |_| {
        let sk_str = comment_for_like.sk.to_string();
        let next = !arena.effective_liked(&comment_for_like);
        like_comment_action.call(sk_str, next);
    };

    let on_toggle_replies = move |_| async move {
        // Mobile swaps the panel into the in-sheet thread view instead
        // of expanding inline. Force-expand so the swap is visible if
        // the user tapped Reply on a collapsed sheet.
        #[cfg(not(feature = "server"))]
        if is_arena_mobile_viewport() {
            let comment_id = SpacePostCommentEntityType::try_from(comment_sk())
                .map(|e| e.0)
                .unwrap_or_default();
            active_reply_thread.set(Some(comment_id));
            sheet_expanded.set(true);
            return;
        }
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

    let mut reply_comment_action = arena.reply_comment;
    let on_submit_reply = move |_| async move {
        let raw_text = reply_text().trim().to_string();
        let images: Vec<String> = reply_pending_images
            .read()
            .iter()
            .filter_map(|img| img.remote_url.clone())
            .collect();
        if raw_text.is_empty() && images.is_empty() {
            return;
        }
        let content = apply_mention_markup(&raw_text, &reply_tracked_mentions.read());
        reply_comment_action
            .call(comment_sk().to_string(), content, images)
            .await;
        reply_text.set(String::new());
        reply_tracked_mentions.set(Vec::new());
        reply_pending_images.set(Vec::new());
        // Inline replies are a local fetch — pull the fresh page so the
        // new reply shows up without waiting for a full loader restart.
        let comment_sk_entity: SpacePostCommentEntityType =
            comment_sk().try_into().unwrap_or_default();
        if let Ok(data) = list_replies(space_id(), discussion_id(), comment_sk_entity, None).await {
            replies.set(data.items);
        }
    };

    let start_edit_content = original_content.clone();
    let on_edit_start = move |_| {
        edit_text.set(start_edit_content.clone());
        editing.set(true);
        menu_open.set(false);
    };

    let on_edit_cancel = move |_| {
        editing.set(false);
    };

    let mut update_comment_action = arena.update_comment;
    let mut delete_comment_action = arena.delete_comment;
    let on_edit_save = move |_| {
        let new_text = edit_text().trim().to_string();
        if new_text.is_empty() {
            return;
        }
        editing.set(false);
        update_comment_action.call(comment_sk().to_string(), new_text);
    };

    let on_delete = move |_| {
        menu_open.set(false);
        delete_comment_action.call(comment_sk().to_string());
    };

    // Hoist overlay reads — each render loop can hit 3+ sites per
    // comment, and `effective_*` hashes `sk.to_string()` every call.
    let liked = arena.effective_liked(&comment);
    let likes_count = arena.effective_likes(&comment);
    let effective_text = arena.effective_content(&comment);

    rsx! {
        div { class: "comment-entry",
            div {
                class: "comment-item",
                id: "{comment_dom_id}",
                "data-deep-link": if deep_link_target().as_deref() == Some(comment_dom_id.as_str()) { "true" } else { "false" },
                img {
                    class: "comment-item__avatar",
                    src: "{comment.author_profile_url}",
                    alt: "{comment.author_display_name}",
                }
                div { class: "comment-item__body",
                    div { class: "comment-item__top",
                        span { class: "comment-item__name", "{comment.author_display_name}" }
                        span { class: "comment-item__time", "{time_ago}" }
                        if is_own && can_comment && !editing() {
                            div { class: "comment-menu",
                                button {
                                    class: "comment-menu__trigger",
                                    "data-testid": "comment-menu-trigger",
                                    aria_label: "{tr.more_options}",
                                    onclick: move |_| menu_open.set(!menu_open()),
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        circle { cx: "5", cy: "12", r: "1.6" }
                                        circle { cx: "12", cy: "12", r: "1.6" }
                                        circle { cx: "19", cy: "12", r: "1.6" }
                                    }
                                }
                                if menu_open() {
                                    div { class: "comment-menu__dropdown",
                                        button {
                                            class: "comment-menu__item",
                                            "data-testid": "comment-menu-edit",
                                            onclick: on_edit_start,
                                            "{tr.edit_btn}"
                                        }
                                        button {
                                            class: "comment-menu__item comment-menu__item--danger",
                                            "data-testid": "comment-menu-delete",
                                            onclick: on_delete,
                                            "{tr.delete_btn}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if editing() {
                        div { class: "comment-item__edit",
                            textarea {
                                class: "comment-item__edit-input",
                                "data-testid": "comment-edit-input",
                                rows: "2",
                                value: "{edit_text}",
                                oninput: move |e| {
                                    edit_text.set(e.value());
                                },
                            }
                            div { class: "comment-item__edit-actions",
                                button {
                                    class: "comment-item__edit-cancel",
                                    "data-testid": "comment-edit-cancel",
                                    onclick: on_edit_cancel,
                                    "{tr.cancel_btn}"
                                }
                                button {
                                    class: "comment-item__edit-save",
                                    "data-testid": "comment-edit-save",
                                    disabled: edit_text().trim().is_empty(),
                                    onclick: on_edit_save,
                                    "{tr.save_btn}"
                                }
                            }
                        }
                    } else {
                        CommentText { content: effective_text.clone() }
                        CommentImageGrid { images: comment.images.clone() }
                    }
                    if !editing() {
                        div { class: "comment-item__actions",
                            button {
                                class: "comment-action",
                                "aria-pressed": liked,
                                disabled: like_comment_action.pending(),
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
                                span { "{likes_count}" }
                            }
                            if can_comment {
                                button {
                                    class: "comment-action comment-action--reply",
                                    "data-testid": "comment-action-reply",
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
            }

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

            if show_replies() {
                div { class: "comment-replies",
                    for reply in replies().iter().filter(|r| !arena.is_deleted(r)) {
                        ReplyItem {
                            key: "{reply.sk}",
                            reply: reply.clone(),
                            space_id,
                            discussion_id,
                            replies,
                            can_comment,
                        }
                    }
                }

                if can_comment {
                    CommentComposer {
                        text: reply_text,
                        tracked_mentions: reply_tracked_mentions,
                        pending_images: reply_pending_images,
                        members,
                        on_submit: move |_| on_submit_reply(()),
                        placeholder: tr.reply_placeholder.to_string(),
                        compact: true,
                        disabled: reply_text().trim().is_empty()
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    && reply_pending_images.read().is_empty(),
                        on_mention_query_change,
                        on_composer_focus,
                        priority_user_pks: reply_priority,
                    }
                }
            }
        }
    }
}

fn format_time_ago(timestamp: i64) -> String {
    let timestamp_millis = if timestamp.abs() < 1_000_000_000_000 {
        timestamp.saturating_mul(1000)
    } else {
        timestamp
    };
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

/// Reply variant of `CommentItem` for per-comment inline / thread-view
/// lists. Mutations route through the controller actions.
#[component]
fn ReplyItem(
    reply: DiscussionCommentResponse,
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    replies: Signal<Vec<DiscussionCommentResponse>>,
    can_comment: bool,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let mut replies = replies;

    let arena = use_discussion_arena(space_id, discussion_id)?;
    let mut like_comment_action = arena.like_comment;

    let user_ctx = crate::features::auth::hooks::use_user_context();
    let is_own = user_ctx
        .read()
        .user
        .as_ref()
        .map(|u| u.pk == reply.author_pk)
        .unwrap_or(false);

    let reply_sk = use_signal(|| reply.sk.clone());
    let mut menu_open = use_signal(|| false);
    let mut editing = use_signal(|| false);
    let mut edit_text = use_signal(|| reply.content.clone());
    let original_content = reply.content.clone();
    let time_ago = format_time_ago(reply.created_at);

    let reply_for_like = reply.clone();
    let on_like = move |_| {
        let sk_str = reply_for_like.sk.to_string();
        let next = !arena.effective_liked(&reply_for_like);
        like_comment_action.call(sk_str, next);
    };

    let start_edit_content = original_content.clone();
    let on_edit_start = move |_| {
        edit_text.set(start_edit_content.clone());
        editing.set(true);
        menu_open.set(false);
    };

    let on_edit_cancel = move |_| {
        editing.set(false);
    };

    let mut update_comment_action = arena.update_comment;
    let mut delete_comment_action = arena.delete_comment;
    let on_edit_save = move |_| {
        let new_text = edit_text().trim().to_string();
        if new_text.is_empty() {
            return;
        }
        editing.set(false);
        update_comment_action.call(reply_sk().to_string(), new_text);
    };

    let on_delete = move |_| {
        menu_open.set(false);
        delete_comment_action.call(reply_sk().to_string());
    };

    let liked = arena.effective_liked(&reply);
    let likes_count = arena.effective_likes(&reply);
    let effective_text = arena.effective_content(&reply);

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
                    if is_own && can_comment && !editing() {
                        div { class: "comment-menu",
                            button {
                                class: "comment-menu__trigger",
                                "data-testid": "comment-menu-trigger",
                                aria_label: "{tr.more_options}",
                                onclick: move |_| menu_open.set(!menu_open()),
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    circle { cx: "5", cy: "12", r: "1.6" }
                                    circle { cx: "12", cy: "12", r: "1.6" }
                                    circle { cx: "19", cy: "12", r: "1.6" }
                                }
                            }
                            if menu_open() {
                                div { class: "comment-menu__dropdown",
                                    button {
                                        class: "comment-menu__item",
                                        "data-testid": "comment-menu-edit",
                                        onclick: on_edit_start,
                                        "{tr.edit_btn}"
                                    }
                                    button {
                                        class: "comment-menu__item comment-menu__item--danger",
                                        "data-testid": "comment-menu-delete",
                                        onclick: on_delete,
                                        "{tr.delete_btn}"
                                    }
                                }
                            }
                        }
                    }
                }
                if editing() {
                    div { class: "comment-item__edit",
                        textarea {
                            class: "comment-item__edit-input",
                            "data-testid": "comment-edit-input",
                            rows: "2",
                            value: "{edit_text}",
                            oninput: move |e| {
                                edit_text.set(e.value());
                            },
                        }
                        div { class: "comment-item__edit-actions",
                            button {
                                class: "comment-item__edit-cancel",
                                "data-testid": "comment-edit-cancel",
                                onclick: on_edit_cancel,
                                "{tr.cancel_btn}"
                            }
                            button {
                                class: "comment-item__edit-save",
                                "data-testid": "comment-edit-save",
                                disabled: edit_text().trim().is_empty(),
                                onclick: on_edit_save,
                                "{tr.save_btn}"
                            }
                        }
                    }
                } else {
                    CommentText { content: effective_text.clone() }
                    CommentImageGrid { images: reply.images.clone() }
                    div { class: "comment-item__actions",
                        button {
                            class: "comment-action",
                            "aria-pressed": liked,
                            disabled: like_comment_action.pending(),
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
                            span { "{likes_count}" }
                        }
                    }
                }
            }
        }
    }
}
