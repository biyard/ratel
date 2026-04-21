use crate::common::components::image_upload_preview::{ImageUploadPreview, PendingImage};
use crate::common::components::mention_autocomplete::{
    MentionAutocomplete, MentionCandidate, MentionInsert,
};
use crate::common::components::paste_image_uploader;
use crate::common::components::CommentImageGrid;
use crate::common::hooks::use_interval;
use crate::common::utils::mention::{apply_mention_markup, parse_mention_segments, ContentSegment};
use crate::features::spaces::pages::actions::actions::discussion::controllers::{
    add_comment, delete_comment, get_discussion_detail, like_comment, list_comments, list_replies,
    reply_comment, update_comment, AddCommentRequest, LikeCommentRequest, ReplyCommentRequest,
    UpdateCommentRequest,
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

// Programmatic value changes from Dioxus's signal binding don't fire the
// `input` event, so the autogrow listener can't reset the stretched inline
// height after a submit clears the composer. This bridge lets the Rust
// side trigger the reset explicitly.
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

// Click-time breakpoint check for the mobile-only thread drill-down. The
// discussion arena's CSS tightens at 500px, so tapping Reply below that
// width swaps the comments panel over to the in-sheet thread view; above
// the breakpoint desktop keeps the inline toggle. `Window::inner_width`
// returns a `JsValue` (the only web_sys API on Window that doesn't
// require an extra feature flag), which we coerce to an `f64` and
// compare directly against the CSS breakpoint.
#[cfg(not(feature = "server"))]
fn is_arena_mobile_viewport() -> bool {
    let width_px = web_sys::window()
        .and_then(|w| w.inner_width().ok())
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    width_px > 0.0 && width_px <= 500.0
}

/// Routable wrapper for `DiscussionArenaPage`. Registered on
/// `/spaces/:space_id/discussions/:discussion_id` so external deep links
/// (e.g. notification CTAs without a target comment) land directly on the
/// discussion viewer. Arena-internal clicks continue to open the overlay via
/// `ActiveActionOverlaySignal`; both paths render the same component.
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

/// Variant of `SpaceDiscussionPage` that carries a `comment_id` path
/// parameter. The id flows down to `DiscussionArenaPage`, whose deep-link
/// effect scrolls to + highlights the matching `.comment-item`. Comment id
/// is in the path (not query/fragment) because Dioxus Router strips both
/// during URL normalization on hydration.
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

/// YouTube/Reddit-style thread drill-down for a single parent comment.
/// Mounted inside the mobile comments sheet (not a standalone route) so
/// opening replies stays within the current bottom-sheet context — the
/// sheet handle remains reachable and going "back" returns the user to
/// the discussion's comments list without popping the route stack.
/// Layout: parent pinned on top, scrollable replies below, composer at
/// the bottom.
///
/// Loaders for the parent comment and its replies live in the
/// `UseDiscussionArena` controller (built once at `DiscussionArenaPage`
/// mount), not here, so that `active_reply_thread` flipping to `Some`
/// triggers a background refresh on those loaders rather than mounting
/// fresh ones — the mount path was suspending up to the overlay's
/// `SuspenseBoundary` and unmounting the entire arena page during the
/// initial fetch (the "arena index briefly visible" flash).
#[component]
fn ReplyThreadView(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    on_close: EventHandler<()>,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let mut toast = use_toast();
    let role = use_space_role()();
    let space = use_space()();

    let UseDiscussionArena {
        mut parent_loader,
        mut replies_loader,
        ..
    } = use_discussion_arena(space_id, discussion_id)?;
    let parent = parent_loader();

    // Owned mutation signal seeded from the controller's reply loader.
    // Optimistic insert/edit/delete from `ReplyItem` and the composer
    // below mutates this signal in place; a `replies_loader` background
    // reload (after `active_reply_thread` flips to a different comment,
    // or after a parent refresh) re-syncs us via the effect below.
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

    // Discussion-level gate mirrors the main arena — non-participants see
    // the thread but cannot reply once the space has ended. Action-level
    // gate isn't re-checked here because this view is only reachable from
    // within the arena, which already enforced it.
    let is_space_open = space.status != Some(SpaceStatus::Finished);
    let can_respond = matches!(role, SpaceUserRole::Creator | SpaceUserRole::Participant);
    let can_comment = can_respond && is_space_open;

    let mut reply_text = use_signal(String::new);
    let mut reply_tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let mut reply_pending_images: Signal<Vec<PendingImage>> = use_signal(Vec::new);

    // Mention autocomplete (lazy load + debounce) — same pattern as the
    // main composer. `None` query means no network fetch until the user
    // engages the textarea.
    let mut mention_query_raw: Signal<Option<String>> = use_signal(|| None);
    let mut mention_query: Signal<Option<String>> = use_signal(|| None);
    use_effect(move || {
        let v = mention_query_raw();
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_millis(150)).await;
            if *mention_query_raw.peek() == v {
                mention_query.set(v);
            }
        });
    });
    let members_loader = use_loader(move || async move {
        match mention_query() {
            None => Ok(ListResponse::<
                crate::features::spaces::space_common::controllers::SpaceMemberResponse,
            >::default()),
            Some(q) => list_space_members(space_id(), None, Some(q)).await,
        }
    })?;
    let members_memo = use_memo(move || {
        members_loader()
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
            .collect::<Vec<_>>()
    });
    let members: ReadSignal<Vec<MentionCandidate>> = members_memo.into();

    let on_mention_query_change = move |q: Option<String>| {
        mention_query_raw.set(q);
    };
    let on_composer_focus = move || {
        if mention_query_raw.peek().is_none() {
            mention_query_raw.set(Some(String::new()));
        }
    };

    // Optimistic like state for the parent — mirrors `CommentItem`. The
    // signals seed empty and pick up the real values via the effect
    // below once the cached loader resolves (loader may still be on the
    // previous value at mount time because the controller-level reload
    // is in flight). After seeding, optimistic toggles own the UI; we
    // re-sync only when the parent's `sk` changes (e.g., user opens a
    // different thread).
    let mut parent_liked = use_signal(|| false);
    let mut parent_likes = use_signal(|| 0i64);
    let mut parent_like_processing = use_signal(|| false);
    let mut last_synced_parent_sk: Signal<String> = use_signal(String::new);
    use_effect(move || {
        let p = parent_loader();
        let sk = p.sk.to_string();
        if !sk.is_empty() && sk != last_synced_parent_sk() {
            parent_liked.set(p.liked);
            parent_likes.set(p.likes as i64);
            last_synced_parent_sk.set(sk);
        }
    });
    let parent_sk_for_like = parent.sk.clone();
    let on_parent_like = move |_| {
        let target_sk: SpacePostCommentTargetEntityType =
            parent_sk_for_like.clone().into();
        async move {
            if parent_like_processing() {
                return;
            }
            let next = !parent_liked();
            let prev_liked = parent_liked();
            let prev_likes = parent_likes();
            parent_liked.set(next);
            parent_likes.set((prev_likes + if next { 1 } else { -1 }).max(0));
            parent_like_processing.set(true);

            let req = LikeCommentRequest { like: next };
            match like_comment(space_id(), discussion_id(), target_sk, req).await {
                Ok(_) => {}
                Err(err) => {
                    parent_liked.set(prev_liked);
                    parent_likes.set(prev_likes);
                    tracing::error!("Failed to toggle parent like: {:?}", err);
                    toast.error(err);
                }
            }
            parent_like_processing.set(false);
        }
    };

    // Priority: parent author first, then reply authors newest-first.
    // Same helper the inline reply composer uses. Read `parent_loader`
    // inside the memo so the priority rebuilds once the cached loader
    // resolves with the real parent (it may have been on the default
    // value at mount time, since the controller-level loader refreshes
    // in the background after `active_reply_thread` flips).
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

    let parent_for_submit = parent.clone();
    let on_submit_reply = move |_| {
        let parent = parent_for_submit.clone();
        async move {
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
            let comment_sk_entity: SpacePostCommentEntityType =
                parent.sk.clone().try_into().unwrap_or_default();
            let req = ReplyCommentRequest { content, images };
            match reply_comment(space_id(), discussion_id(), comment_sk_entity, req).await {
                Ok(new_reply) => {
                    let mut current = replies();
                    current.insert(0, new_reply);
                    replies.set(current);
                    reply_text.set(String::new());
                    reply_tracked_mentions.set(Vec::new());
                    reply_pending_images.set(Vec::new());
                    parent_loader.restart();
                    toast.info(tr.reply_success);
                }
                Err(err) => {
                    tracing::error!("Failed to post reply: {:?}", err);
                    toast.error(err);
                }
            }
        }
    };

    let parent_time_ago = format_time_ago(parent.created_at);

    rsx! {
        div { class: "reply-thread", "data-testid": "reply-thread",
            // Header: back + title
            div { class: "reply-thread__header",
                button {
                    class: "reply-thread__icon-btn",
                    "data-testid": "reply-thread-back",
                    aria_label: "{tr.replies_back_aria}",
                    onclick: move |_| on_close.call(()),
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
                span { class: "reply-thread__title", "{tr.replies_page_title}" }
            }

            // Scroll area: parent comment on top, replies below.
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
                            div { class: "comment-item__text",
                                for segment in parse_mention_segments(&parent.content) {
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
                            CommentImageGrid { images: parent.images.clone() }
                            div { class: "comment-item__actions",
                                button {
                                    class: if parent_liked() { "comment-action comment-action--liked" } else { "comment-action" },
                                    disabled: parent_like_processing(),
                                    onclick: on_parent_like,
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: if parent_liked() { "currentColor" } else { "none" },
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" }
                                    }
                                    span { "{parent_likes()}" }
                                }
                            }
                        }
                    }
                }

                div { class: "reply-thread__list",
                    for reply in replies().iter() {
                        ReplyItem {
                            key: "{reply.sk}",
                            reply: reply.clone(),
                            space_id,
                            discussion_id,
                            replies,
                            on_parent_refresh: move |_| parent_loader.restart(),
                            can_comment,
                        }
                    }
                }
            }

            // Pinned composer at the bottom.
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
    // When `Some`, `data-deep-link` is set on the matching `.comment-item`
    // and the deep-link effect scrolls it into view. Sourced from the
    // `Route::SpaceDiscussionCommentPage` path parameter; arena-overlay
    // entries pass `None`.
    #[props(default)] target_comment_id: Option<String>,
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
        list_comments(space_id(), discussion_id(), None, None)
    })?;
    let comments_data = comments_loader();

    // Short-polling: every 5s fetch comments newer than the most recent one we
    // have already rendered and keep them in a local signal. Render appends
    // them AFTER the loader's base page (deduped by `sk`) so existing items'
    // positions never shift — avoids Dioxus reorder-induced DOM errors, scroll
    // jumps, and focus loss.
    let mut polled_new: Signal<Vec<DiscussionCommentResponse>> = use_signal(Vec::new);
    let mut last_seen_at: Signal<i64> = use_signal(move || {
        comments_loader()
            .items
            .iter()
            .map(|c| c.created_at)
            .max()
            .unwrap_or_else(crate::common::utils::time::get_now_timestamp)
    });
    use_interval(5000, move || {
        let since = last_seen_at();
        tracing::info!("[arena discussion poll] tick since={}", since);
        spawn(async move {
            match list_comments(space_id(), discussion_id(), None, Some(since)).await {
                Ok(resp) => {
                    if resp.items.is_empty() {
                        return;
                    }
                    let new_max = resp
                        .items
                        .iter()
                        .map(|c| c.created_at)
                        .max()
                        .unwrap_or(since);
                    polled_new.with_mut(|list| {
                        // server returns newest-first; reverse so chronological
                        // order (older → newer) is preserved when appending.
                        for item in resp.items.into_iter().rev() {
                            if !list.iter().any(|x| x.sk == item.sk) {
                                list.push(item);
                            }
                        }
                    });
                    if new_max > since {
                        last_seen_at.set(new_max);
                    }
                }
                Err(e) => {
                    tracing::debug!("arena comment poll failed: {:?}", e);
                }
            }
        });
    });

    // Merge priority: the base page (returned by the loader, ordered by the
    // server's canonical GSI) is the source of truth for content. Polled items
    // that haven't yet been picked up by a restart are appended at the end.
    // This way a loader restart after an edit immediately reflects the new
    // content — the polled list may still carry the stale snapshot with the
    // same sk, but base wins and the polled duplicate is filtered out.
    let comments: Vec<DiscussionCommentResponse> = {
        let polled = polled_new();
        let base = comments_data.items.clone();
        let base_sks: std::collections::HashSet<String> =
            base.iter().map(|c| c.sk.to_string()).collect();
        base.into_iter()
            .chain(
                polled
                    .into_iter()
                    .filter(|p| !base_sks.contains(&p.sk.to_string())),
            )
            .collect()
    };

    // Overlay is provided by `SpaceIndexPage` when this renders as an arena
    // overlay. Under the standalone `Route::SpaceDiscussionPage`, there is no
    // provider — `on_back` falls back to `nav.go_back()` in that case.
    let overlay_ctx: Option<ActiveActionOverlaySignal> = try_consume_context();
    let nav = use_navigator();

    // Path-based deep-link target. When set (via the
    // `SpaceDiscussionCommentPage` route variant), the matching
    // `.comment-item` gets `data-deep-link="true"` (drives the CSS pulse)
    // and is scrolled into view once it appears in the DOM. Signal-driven
    // so Dioxus's next diff doesn't clobber the attribute. Single success
    // per mount via the `done` guard; retries on each loader tick until
    // the target renders (handles unpolled comments).
    let deep_link_target: Signal<Option<String>> = use_signal(|| target_comment_id.clone());
    let mut deep_link_done: Signal<bool> = use_signal(|| target_comment_id.is_none());
    // Call `use_effect` unconditionally so hook indices match between the
    // server (SSR) and web (hydration) builds. Only the body touches
    // browser APIs, so gate those with `#[cfg(feature = "web")]`.
    use_effect(move || {
        let _ = comments_loader();
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
                // Not rendered yet (unpolled, or a reply — Phase 2). Retry
                // on next comments_loader tick.
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

    // Per-discussion controller. Builds and provides `UseDiscussionArena`
    // on first call; descendants (`CommentItem`, `ReplyThreadView`)
    // reuse the cached instance. `active_reply_thread` and the parent /
    // replies loaders for the in-sheet thread view live here so that
    // tapping Reply triggers a background refresh on existing loaders
    // rather than mounting new ones (which would suspend up to the
    // overlay boundary and flash the arena index through).
    let arena = use_discussion_arena(space_id, discussion_id)?;
    let active_reply_thread = arena.active_reply_thread;
    let mut sheet_expanded = arena.sheet_expanded;

    // Mention autocomplete uses a lazy load + server-side search pattern:
    //  * `mention_query_raw` tracks the live query from MentionAutocomplete
    //    (and is seeded by textarea focus so the dropdown is warm before the
    //    user types `@`).
    //  * `mention_query` is the debounced version that actually drives the
    //    loader, to avoid hammering the server on every keystroke.
    // A `None` value means no fetch has been requested yet, so we skip the
    // network entirely for viewers who never engage the composer.
    let mut mention_query_raw: Signal<Option<String>> = use_signal(|| None);
    let mut mention_query: Signal<Option<String>> = use_signal(|| None);

    use_effect(move || {
        let v = mention_query_raw();
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_millis(150)).await;
            if *mention_query_raw.peek() == v {
                mention_query.set(v);
            }
        });
    });

    let members_loader = use_loader(move || async move {
        match mention_query() {
            None => Ok(ListResponse::<
                crate::features::spaces::space_common::controllers::SpaceMemberResponse,
            >::default()),
            Some(q) => list_space_members(space_id(), None, Some(q)).await,
        }
    })?;

    let members_memo = use_memo(move || {
        members_loader()
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
            .collect::<Vec<_>>()
    });
    let members: ReadSignal<Vec<MentionCandidate>> = members_memo.into();

    let on_mention_query_change = move |q: Option<String>| {
        mention_query_raw.set(q);
    };

    let on_composer_focus = move || {
        if mention_query_raw.peek().is_none() {
            mention_query_raw.set(Some(String::new()));
        }
    };

    // Priority roster for the top-level composer: everyone who has posted
    // in this discussion, plus anyone they mentioned. Rebuilds whenever
    // the loader or poll buffer changes so new comments surface the new
    // voices right away.
    let top_priority = use_memo(move || {
        let base = comments_loader().items.clone();
        let polled = polled_new();
        let base_sks: std::collections::HashSet<String> =
            base.iter().map(|c| c.sk.to_string()).collect();
        let tuples: Vec<(String, String)> = base
            .iter()
            .chain(
                polled
                    .iter()
                    .filter(|p| !base_sks.contains(&p.sk.to_string())),
            )
            .map(|c| (c.author_pk.to_string(), c.content.clone()))
            .collect();
        let refs: Vec<(&str, &str)> = tuples
            .iter()
            .map(|(a, c)| (a.as_str(), c.as_str()))
            .collect();
        crate::common::utils::mention::build_mention_priority(None, &refs)
    });
    let top_priority: ReadSignal<Vec<String>> = top_priority.into();

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

    let on_submit_comment = move |_| async move {
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
        let req = AddCommentRequest { content, images };
        match add_comment(space_id(), discussion_id(), req).await {
            Ok(_) => {
                comments_loader.restart();
                space_ctx.actions.restart();
                comment_text.set(String::new());
                tracked_mentions.set(Vec::new());
                pending_images.set(Vec::new());
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
        document::Script { r#type: "module", src: asset!("./script.js") }

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
                div {
                    class: "comments-panel",
                    id: "discussion-comments-sheet",
                    "data-expanded": sheet_expanded(),
                    // Sheet handle (visible on mobile only). Owned by
                    // Dioxus so the expand state survives re-renders that
                    // happen when the thread drill-down swaps panel
                    // content. Tapping toggles the signal; the `data-
                    // expanded` attribute drives the CSS translate.
                    div {
                        class: "sheet-handle",
                        onclick: move |_| sheet_expanded.toggle(),
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

                    // Mobile thread drill-down swaps the panel content over
                    // to a single-comment thread view. Keeps the user inside
                    // the bottom sheet (no route change) so the sheet handle
                    // stays reachable and "Back" returns to the comments
                    // list without popping the route stack.
                    if let Some(thread_id) = active_reply_thread() {
                        ReplyThreadView {
                            key: "{thread_id}",
                            space_id,
                            discussion_id,
                            on_close: {
                                let mut active = active_reply_thread;
                                move |_| active.set(None)
                            },
                        }
                    } else {
                        // Desktop header
                        div { class: "comments-panel__header",
                            span { class: "comments-panel__title", "{tr.comments_title}" }
                            span { class: "comments-panel__count", "{post.comments}" }
                        }

                        // Comment Input
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
                                        polled_new,
                                        deep_link_target,
                                        on_mention_query_change,
                                        on_composer_focus,
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

// ── Comment Composer ────────────────────────────────
//
// Shared input for the top-level comment box and the per-comment reply box.
// Both flows need the same MentionAutocomplete, mention-insert handling and
// submit semantics, so they live here once. DOM classes and structure are
// intentionally left identical to the pre-refactor markup so CSS and
// Playwright selectors (`.comment-input__*`, `.reply-input__*`) keep working
// without changes.
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
    // Parent gets the active `@` query (debounced further upstream) so it
    // can run the server-side mention search. `None` clears the dropdown.
    #[props(default)] on_mention_query_change: EventHandler<Option<String>>,
    // Fires when the textarea first gains focus. Parent uses this to warm
    // the mention list before the user types `@`, so the dropdown is ready
    // without paying a network round-trip on the first keystroke.
    #[props(default)] on_composer_focus: EventHandler<()>,
    // Priority-ordered user pks for mention ranking (thread participants
    // first). Empty = fall back to server's native order.
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

    // Ctrl/Cmd+Enter submits; plain Enter and Shift+Enter fall through to
    // the textarea's default newline. MentionAutocomplete skips modifier-
    // qualified Enter/Tab so the dropdown doesn't swallow the submit key.
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

    // Single markup for both the top-level composer (at the top of the
    // comments panel) and the reply composer (under a specific comment).
    // `data-compact="true"` lets CSS branch on the two contexts — the reply
    // variant picks up a top separator (it sits at the bottom of a thread)
    // while the top-level variant keeps the bottom separator from the
    // comments list below it. The `.comment-input__*` hook classes on the
    // top-level textarea/button stay in place so existing Playwright
    // selectors keep resolving.
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
                        // Only the reply composer auto-focuses on mount —
                        // the top-level composer is always visible and
                        // grabbing focus on every render would steal it
                        // from whatever the user is reading.
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

// ── Comment Item ────────────────────────────────────
#[component]
fn CommentItem(
    comment: DiscussionCommentResponse,
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    can_comment: bool,
    members: ReadSignal<Vec<MentionCandidate>>,
    comments_loader: Loader<ListResponse<DiscussionCommentResponse>>,
    polled_new: Signal<Vec<DiscussionCommentResponse>>,
    deep_link_target: ReadSignal<Option<String>>,
    on_mention_query_change: EventHandler<Option<String>>,
    on_composer_focus: EventHandler<()>,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let mut comments_loader = comments_loader;
    let mut polled_new = polled_new;
    let mut toast = use_toast();

    let mut show_replies = use_signal(|| false);
    let mut reply_text = use_signal(String::new);
    let mut reply_pending_images: Signal<Vec<PendingImage>> = use_signal(Vec::new);
    let mut reply_tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let mut replies: Signal<Vec<DiscussionCommentResponse>> = use_signal(Vec::new);

    let time_ago = format_time_ago(comment.created_at);
    let comment_sk = use_signal(|| comment.sk.clone());
    let reply_count = comment.replies;

    // Priority roster for the reply composer under this comment. Parent
    // author always leads (that's who the user is answering), then the
    // loaded replies newest-first, then mentions already in the thread.
    // Scoped to this CommentItem so different threads rank independently.
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

    // DOM id for deep-linking. Match the URL fragment format
    // (`#<uuid>`) used by mention notification CTAs so the fragment
    // scroller in `DiscussionArenaPage` can resolve us.
    let comment_dom_id: String = SpacePostCommentEntityType::try_from(comment.sk.clone())
        .map(|e| e.0)
        .unwrap_or_else(|_| comment.sk.to_string());

    // Optimistic like state — mirrors the reply path. Local signals own the
    // visible state so toggling feels instant; server mutations reconcile
    // via `polled_new` patch so a subsequent `comments_loader.restart()`
    // (post/reply/delete) doesn't flip the indicator back to a stale value.
    let mut liked = use_signal(|| comment.liked);
    let mut likes = use_signal(|| comment.likes as i64);
    let mut like_processing = use_signal(|| false);

    // Ownership: only the author sees the edit/delete menu. Server-side
    // update/delete controllers also verify `comment.author_pk == user.pk`,
    // so the UI gate is a UX affordance, not a security boundary.
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

    let on_like = move |_| async move {
        if like_processing() {
            return;
        }
        let next = !liked();
        let prev_liked = liked();
        let prev_likes = likes();
        liked.set(next);
        likes.set((prev_likes + if next { 1 } else { -1 }).max(0));
        like_processing.set(true);

        let target_sk: SpacePostCommentTargetEntityType = comment_sk().into();
        let req = LikeCommentRequest { like: next };
        match like_comment(space_id(), discussion_id(), target_sk, req).await {
            Ok(_) => {
                // Patch any poll-cached copy so a later `comments_loader`
                // merge doesn't fight the optimistic state. Base pages come
                // fresh from the server after a restart, so they already
                // reflect the new like count.
                let sk = comment_sk();
                polled_new.with_mut(|list| {
                    for item in list.iter_mut() {
                        if item.sk == sk {
                            item.liked = next;
                            item.likes = likes().max(0) as u64;
                        }
                    }
                });
            }
            Err(err) => {
                liked.set(prev_liked);
                likes.set(prev_likes);
                tracing::error!("Failed to toggle like: {:?}", err);
                toast.error(err);
            }
        }
        like_processing.set(false);
    };

    // Pull the per-discussion controller out of context — the parent
    // `DiscussionArenaPage` already built it, so this is a cheap lookup.
    let arena = use_discussion_arena(space_id, discussion_id)?;
    let mut active_reply_thread = arena.active_reply_thread;
    let mut sheet_expanded = arena.sheet_expanded;
    let on_toggle_replies = move |_| async move {
        // Mobile: swap the comments panel over to the thread view instead
        // of expanding inline — the narrow column can't comfortably fit
        // parent + thread + composer together. Staying inside the
        // bottom-sheet (vs navigating to a new route) keeps the sheet
        // handle reachable and lets "Back" return to the comments list
        // without popping the route stack. Force-expand the sheet so the
        // swap is visible in case the user tapped Reply before the sheet
        // was manually opened. Breakpoint matches the CSS media query
        // that tightens the arena layout.
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
        let comment_sk_entity: SpacePostCommentEntityType =
            comment_sk().try_into().unwrap_or_default();
        let req = ReplyCommentRequest { content, images };
        match reply_comment(space_id(), discussion_id(), comment_sk_entity, req).await {
            Ok(new_reply) => {
                let mut current = replies();
                current.insert(0, new_reply);
                replies.set(current);
                reply_text.set(String::new());
                reply_tracked_mentions.set(Vec::new());
                reply_pending_images.set(Vec::new());
                comments_loader.restart();
                toast.info(tr.reply_success);
            }
            Err(err) => {
                tracing::error!("Failed to post reply: {:?}", err);
                toast.error(err);
            }
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

    let on_edit_save = move |_| async move {
        let new_text = edit_text().trim().to_string();
        if new_text.is_empty() {
            return;
        }
        let target_sk: SpacePostCommentTargetEntityType = comment_sk().into();
        let req = UpdateCommentRequest {
            content: new_text.clone(),
            images: None,
        };
        match update_comment(space_id(), discussion_id(), target_sk, req).await {
            Ok(_) => {
                // Reflect the edit immediately: base (refreshed via restart)
                // wins on content, but any stale poll-cached copy of the same
                // sk would still override rendering for a frame if present.
                // Patch the polled entry's content so nothing shows the old
                // text even momentarily.
                let sk = comment_sk();
                polled_new.with_mut(|list| {
                    for item in list.iter_mut() {
                        if item.sk == sk {
                            item.content = new_text.clone();
                        }
                    }
                });
                editing.set(false);
                comments_loader.restart();
                toast.info(tr.edit_success);
            }
            Err(err) => {
                tracing::error!("Failed to update comment: {:?}", err);
                toast.error(err);
            }
        }
    };

    let on_delete = move |_| async move {
        let target_sk: SpacePostCommentTargetEntityType = comment_sk().into();
        menu_open.set(false);
        match delete_comment(space_id(), discussion_id(), target_sk).await {
            Ok(_) => {
                // Scrub the polled cache — list_comments(since=...) only
                // returns comments by created_at, so a deleted item is never
                // re-observed by polling; without this, the stale copy would
                // linger in the polled tail forever.
                let sk = comment_sk();
                polled_new.with_mut(|list| list.retain(|c| c.sk != sk));
                comments_loader.restart();
                toast.info(tr.delete_success);
            }
            Err(err) => {
                tracing::error!("Failed to delete comment: {:?}", err);
                toast.error(err);
            }
        }
    };

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
                        CommentImageGrid { images: comment.images.clone() }
                    }
                    if !editing() {
                        div { class: "comment-item__actions",
                            button {
                                class: if liked() { "comment-action comment-action--liked" } else { "comment-action" },
                                disabled: like_processing(),
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
                        ReplyItem {
                            key: "{reply.sk}",
                            reply: reply.clone(),
                            space_id,
                            discussion_id,
                            replies,
                            on_parent_refresh: move |_| comments_loader.restart(),
                            can_comment,
                        }
                    }
                }

                // Reply input
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

// ── Helpers ──────────────────────────────────────────

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

// ── Reply Item ──────────────────────────────────────
// Same edit/delete affordances as CommentItem, but scoped to a single reply.
// Replies live in a parent-owned `replies: Signal<Vec<...>>` (not the global
// comments_loader), so mutations patch that signal locally and also fire
// `on_parent_refresh` so callers (discussion page, dedicated replies page)
// can refresh whatever view holds the parent's `reply_count`.
#[component]
fn ReplyItem(
    reply: DiscussionCommentResponse,
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    replies: Signal<Vec<DiscussionCommentResponse>>,
    on_parent_refresh: EventHandler<()>,
    can_comment: bool,
) -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let mut toast = use_toast();
    let mut replies = replies;

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

    let mut liked = use_signal(|| reply.liked);
    let mut likes = use_signal(|| reply.likes as i64);
    let mut like_processing = use_signal(|| false);

    let on_like = move |_| async move {
        if like_processing() {
            return;
        }
        let next = !liked();
        let prev_liked = liked();
        let prev_likes = likes();
        liked.set(next);
        likes.set((prev_likes + if next { 1 } else { -1 }).max(0));
        like_processing.set(true);

        let target_sk: SpacePostCommentTargetEntityType = reply_sk().into();
        let req = LikeCommentRequest { like: next };
        match like_comment(space_id(), discussion_id(), target_sk, req).await {
            Ok(_) => {
                let sk = reply_sk();
                replies.with_mut(|list| {
                    for r in list.iter_mut() {
                        if r.sk == sk {
                            r.liked = next;
                            r.likes = likes().max(0) as u64;
                        }
                    }
                });
            }
            Err(err) => {
                liked.set(prev_liked);
                likes.set(prev_likes);
                tracing::error!("Failed to toggle reply like: {:?}", err);
                toast.error(err);
            }
        }
        like_processing.set(false);
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

    let on_edit_save = move |_| async move {
        let new_text = edit_text().trim().to_string();
        if new_text.is_empty() {
            return;
        }
        let target_sk: SpacePostCommentTargetEntityType = reply_sk().into();
        let req = UpdateCommentRequest {
            content: new_text.clone(),
            images: None,
        };
        match update_comment(space_id(), discussion_id(), target_sk, req).await {
            Ok(_) => {
                // Patch the local replies list optimistically — replies are
                // held in the parent's signal (not comments_loader), so
                // there's no "base page" to fall back to here.
                let sk = reply_sk();
                replies.with_mut(|list| {
                    for r in list.iter_mut() {
                        if r.sk == sk {
                            r.content = new_text.clone();
                        }
                    }
                });
                editing.set(false);
                toast.info(tr.edit_success);
            }
            Err(err) => {
                tracing::error!("Failed to update reply: {:?}", err);
                toast.error(err);
            }
        }
    };

    let on_delete = move |_| async move {
        let target_sk: SpacePostCommentTargetEntityType = reply_sk().into();
        menu_open.set(false);
        match delete_comment(space_id(), discussion_id(), target_sk).await {
            Ok(_) => {
                let sk = reply_sk();
                replies.with_mut(|list| list.retain(|r| r.sk != sk));
                // Server decrements the parent's `replies` counter — let
                // the host refresh whatever holds that count (comments
                // loader on the discussion page, parent loader on the
                // dedicated replies page).
                on_parent_refresh.call(());
                toast.info(tr.delete_success);
            }
            Err(err) => {
                tracing::error!("Failed to delete reply: {:?}", err);
                toast.error(err);
            }
        }
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
                    CommentImageGrid { images: reply.images.clone() }
                    div { class: "comment-item__actions",
                        button {
                            class: if liked() { "comment-action comment-action--liked" } else { "comment-action" },
                            disabled: like_processing(),
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
}
