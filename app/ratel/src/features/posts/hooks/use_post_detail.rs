//! Hook + controller for the post detail page. Mirrors the Essence hook
//! pattern (`use_essence_sources`): one `try_use_context` early-return,
//! `use_loader` for the server payload, and `use_action` wrappers for every
//! mutation so components (PostDetail / PostCommentsPanel / CommentItem /
//! ReplyItem) invoke `hook.toggle_like.call(())` etc. without ever
//! importing the server-function handlers themselves.
//!
//! The hook is caller-driven: `use_post_detail(post_id)` builds the
//! controller the first time it's called in a subtree and re-exposes the
//! cached instance on every subsequent call. `post_id` is stored as a
//! `ReadSignal<FeedPartition>` inside the controller so per-comment
//! actions can read the current identifier without each consumer having
//! to thread it through props.

use crate::common::components::mention_autocomplete::MentionCandidate;
use crate::common::utils::mention::apply_mention_markup;
use crate::features::auth::hooks::use_user_context;
use crate::features::posts::controllers::comments::add_comment::{
    add_comment_handler, AddPostCommentRequest,
};
use crate::features::posts::controllers::comments::like_comment::like_comment_handler;
use crate::features::posts::controllers::comments::list_comments::list_comments_handler;
use crate::features::posts::controllers::comments::reply_to_comment::{
    reply_to_comment_handler, ReplyToPostCommentRequest,
};
use crate::features::posts::controllers::dto::{PostCommentResponse, PostDetailResponse};
use crate::features::posts::controllers::get_post::get_post_handler;
use crate::features::posts::controllers::like_post::like_post_handler;
use crate::features::posts::types::PostCommentTargetEntityType;
use crate::features::posts::*;
use dioxus_core::CapturedError;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, DioxusController)]
pub struct UsePostDetail {
    /// Post + top-level comments as returned by `get_post_handler`. The
    /// loader is the single source of truth for the hero card (`title`,
    /// `html_contents`, author), the action buttons' initial `liked` /
    /// `likes` / `comments` counters, and the top-level comment list.
    pub detail: Loader<PostDetailResponse>,
    pub post_id: ReadSignal<FeedPartition>,

    // Post-level UI state
    pub liked: Signal<bool>,
    pub like_count: Signal<i64>,
    pub comments_open: Signal<bool>,

    // Top-level comment composer state
    pub comment_text: Signal<String>,
    pub tracked_mentions: Signal<Vec<(String, String)>>,
    pub is_submitting: Signal<bool>,

    // Cached mention candidates — currently unused but kept on the
    // controller so future work that wires actual candidates (space
    // members, followers) has a single place to populate.
    pub members: Signal<Vec<MentionCandidate>>,

    /// Per-parent-comment reply lists, keyed by parent comment sk.
    /// Lives at the hook (root) scope so mutations from `load_replies` /
    /// `submit_reply` actions can read/write without pulling child-owned
    /// Signal handles across scopes (Dioxus warns against that pattern).
    /// Each `CommentItem` derives its local replies slice via a `memo`
    /// over this map keyed by its own sk.
    pub replies_by_comment: Signal<HashMap<String, Vec<PostCommentResponse>>>,
    /// Tracks which parent sks have already been fetched so the lazy-load
    /// effect doesn't re-run on every render.
    pub replies_loaded: Signal<HashSet<String>>,

    // Mutations
    pub toggle_like: Action<(), ()>,
    pub share: Action<(), bool>,
    pub submit_comment: Action<(), ()>,
    /// Server-side like toggle for one comment. Takes `(sk, new_liked)`.
    /// Optimistic UI is the caller's responsibility — each `CommentItem`
    /// owns its own `liked` / `likes` signals (per-item UI state),
    /// flips them locally, then fires this action. Passing the child's
    /// signals INTO the action would cross scope ownership (action lives
    /// in the root hook scope, signals live in the CommentItem scope) and
    /// Dioxus warns about exactly that pattern.
    pub toggle_comment_like: Action<(PostCommentTargetEntityType, bool), ()>,
    /// Reply submission. Arg is `(parent_sk, raw_content, mentions)`.
    /// On success, the new reply is prepended into `replies_by_comment`
    /// at `sk` — no caller signals passed in, so the action body runs
    /// entirely against root-scope state.
    pub submit_reply: Action<
        (
            PostCommentTargetEntityType,
            String,
            Vec<(String, String)>,
        ),
        (),
    >,
    /// Lazy-load replies for one comment. Populates `replies_by_comment`
    /// at the given sk and marks it seen in `replies_loaded` so the
    /// CommentItem's effect doesn't re-fire.
    pub load_replies: Action<(PostCommentTargetEntityType,), ()>,
}

#[track_caller]
pub fn use_post_detail(
    post_id: FeedPartition,
) -> std::result::Result<UsePostDetail, Loading> {
    // 1. Cached instance wins — every consumer in this subtree shares one
    //    controller so toggling `comments_open` in the header propagates to
    //    the drawer without extra plumbing.
    if let Some(ctx) = try_use_context::<UsePostDetail>() {
        return Ok(ctx);
    }

    // 2. Stash the incoming `post_id` as a ReadSignal so the actions can
    //    read the current value without each caller passing it in.
    let post_id_signal: ReadSignal<FeedPartition> =
        use_signal(|| post_id.clone()).into();

    // 3. Server data — guaranteed non-`Loading` by the time we reach the
    //    action bodies because `?` suspends the caller until `detail`
    //    resolves. Reading `post_id_signal()` inside the closure makes the
    //    loader reactive to future post-id changes (not used today but
    //    keeps the hook correct if someone wires a router-driven swap).
    let detail = use_loader(move || {
        let pid = post_id_signal();
        async move { get_post_handler(pid).await }
    })?;
    let snapshot = detail();

    // 4. Seed UI signals from the loaded detail so optimistic clicks don't
    //    wait for a refetch. `is_liked` is viewer-specific (server-side
    //    PostLike batch-read against the current user); `post.likes` is the
    //    aggregate counter on the Post row.
    let liked = use_signal(|| snapshot.is_liked);
    let like_count = use_signal(|| snapshot.post.as_ref().map(|p| p.likes).unwrap_or(0));
    let comments_open = use_signal(|| false);
    let comment_text = use_signal(String::new);
    let tracked_mentions: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let is_submitting = use_signal(|| false);
    let members: Signal<Vec<MentionCandidate>> = use_signal(Vec::new);
    let replies_by_comment: Signal<HashMap<String, Vec<PostCommentResponse>>> =
        use_signal(HashMap::new);
    let replies_loaded: Signal<HashSet<String>> = use_signal(HashSet::new);

    // 5. Mutations — each `use_action` body owns the post-mutation side
    //    effects (refetch, signal resets, toasts). Components just call
    //    these; they never see the underlying `_handler` functions.

    let mut detail_for_actions = detail;
    let mut liked_sig = liked;
    let mut like_count_sig = like_count;
    let post_id_for_like = post_id_signal;
    let toggle_like = use_action(move || async move {
        let pid = post_id_for_like();
        let next = !liked_sig();
        let prev_count = like_count_sig();
        // Optimistic flip so the heart turns red immediately — rollback
        // below if the server rejects.
        liked_sig.set(next);
        like_count_sig.set((prev_count + if next { 1 } else { -1 }).max(0));
        match like_post_handler(pid, next).await {
            Ok(_) => Ok::<(), CapturedError>(()),
            Err(e) => {
                liked_sig.set(!next);
                like_count_sig.set(prev_count);
                Err(CapturedError::from(e))
            }
        }
    });

    let mut toast = use_toast();
    let post_id_for_share = post_id_signal;
    let share = use_action(move || async move {
        let id = post_id_for_share().to_string();
        // Build the absolute URL at runtime so staging/prod/localhost all
        // produce the correct link, then ask the Clipboard API for a
        // copy. The eval echoes back a bool so the toast can distinguish
        // permission failures (e.g. non-HTTPS origin) from success.
        let js = format!(
            r#"(async function() {{
                var url = window.location.origin + '/posts/{id}';
                try {{
                    await navigator.clipboard.writeText(url);
                    dioxus.send(true);
                }} catch (e) {{
                    dioxus.send(false);
                }}
            }})();"#
        );
        let mut eval = document::eval(&js);
        let ok = eval.recv::<bool>().await.unwrap_or(false);
        if ok {
            toast.info("Link copied to clipboard");
        } else {
            toast.warn("Failed to copy link");
        }
        Ok::<bool, CapturedError>(ok)
    });

    let post_id_for_submit = post_id_signal;
    let mut comment_text_sig = comment_text;
    let mut tracked_mentions_sig = tracked_mentions;
    let mut is_submitting_sig = is_submitting;
    let submit_comment = use_action(move || async move {
        let raw = comment_text_sig().trim().to_string();
        if raw.is_empty() || is_submitting_sig() {
            return Ok::<(), CapturedError>(());
        }
        is_submitting_sig.set(true);
        let content = apply_mention_markup(&raw, &tracked_mentions_sig.read());
        let req = AddPostCommentRequest {
            content,
            images: vec![],
        };
        let result = add_comment_handler(post_id_for_submit(), req).await;
        match result {
            Ok(_) => {
                comment_text_sig.set(String::new());
                tracked_mentions_sig.set(Vec::new());
                detail_for_actions.restart();
                is_submitting_sig.set(false);
                Ok(())
            }
            Err(e) => {
                is_submitting_sig.set(false);
                Err(CapturedError::from(e))
            }
        }
    });

    let post_id_for_like_comment = post_id_signal;
    let toggle_comment_like = use_action(
        move |target_sk: PostCommentTargetEntityType, liked: bool| async move {
            let pid = post_id_for_like_comment();
            match like_comment_handler(pid, target_sk, liked).await {
                Ok(_) => Ok::<(), CapturedError>(()),
                Err(e) => {
                    tracing::error!("like post comment failed: {e}");
                    Err(CapturedError::from(e))
                }
            }
        },
    );

    let post_id_for_reply = post_id_signal;
    let mut replies_map_for_submit = replies_by_comment;
    let submit_reply = use_action(
        move |parent_sk: PostCommentTargetEntityType,
              raw: String,
              mentions: Vec<(String, String)>| async move {
            let raw = raw.trim().to_string();
            if raw.is_empty() {
                return Ok::<(), CapturedError>(());
            }
            let sk_str = parent_sk.to_string();
            let content = apply_mention_markup(&raw, &mentions);
            // PostCommentTargetEntityType → EntityType → PostCommentEntityType
            // (no direct From between the two sub-partitions; go via the
            // shared EntityType enum the DDB sk is really typed as.)
            let as_entity: EntityType = parent_sk.into();
            let parent_id: crate::PostCommentEntityType = as_entity.into();
            let req = ReplyToPostCommentRequest {
                content,
                images: vec![],
            };
            match reply_to_comment_handler(post_id_for_reply(), parent_id, req).await {
                Ok(new_reply_model) => {
                    // Prepend the new reply into the root-scope map —
                    // CommentItem's memo re-derives its slice from this
                    // map, so the UI updates without any child-owned
                    // signal crossing scopes into this action.
                    let new_reply: PostCommentResponse =
                        (new_reply_model, false, false).into();
                    replies_map_for_submit.with_mut(|map| {
                        map.entry(sk_str).or_default().insert(0, new_reply);
                    });
                    Ok(())
                }
                Err(e) => Err(CapturedError::from(e)),
            }
        },
    );

    let post_id_for_list = post_id_signal;
    let mut replies_map_for_load = replies_by_comment;
    let mut replies_loaded_sig = replies_loaded;
    let load_replies = use_action(move |parent_sk: PostCommentTargetEntityType| async move {
        let sk_str = parent_sk.to_string();
        if replies_loaded_sig.read().contains(&sk_str) {
            return Ok::<(), CapturedError>(());
        }
        let as_entity: EntityType = parent_sk.into();
        let parent_id: crate::PostCommentEntityType = as_entity.into();
        match list_comments_handler(post_id_for_list(), parent_id, None).await {
            Ok(resp) => {
                replies_map_for_load.with_mut(|map| {
                    map.insert(sk_str.clone(), resp.items);
                });
                replies_loaded_sig.with_mut(|s| {
                    s.insert(sk_str);
                });
                Ok(())
            }
            Err(e) => Err(CapturedError::from(e)),
        }
    });

    // `use_user_context` is read here so future work (e.g. showing an
    // author-only edit affordance inside comments) has the identity
    // handy; keeping the call at hook-scope avoids re-subscribing per
    // comment-item render.
    let _ = use_user_context();

    Ok(use_context_provider(move || UsePostDetail {
        detail,
        post_id: post_id_signal,
        liked,
        like_count,
        comments_open,
        comment_text,
        tracked_mentions,
        is_submitting,
        members,
        replies_by_comment,
        replies_loaded,
        toggle_like,
        share,
        submit_comment,
        toggle_comment_like,
        submit_reply,
        load_replies,
    }))
}
