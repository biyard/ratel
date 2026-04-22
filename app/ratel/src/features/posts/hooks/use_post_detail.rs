//! Hook + controller for the post detail page. Follows the Ratel hook
//! convention: one `try_use_context` early-return, `use_loader` for server
//! data, and `use_action` wrappers for every mutation so components
//! (PostDetail / PostCommentsPanel / CommentItem / ReplyItem) invoke
//! `hook.toggle_like.call(())` etc. without ever importing the
//! server-function handlers themselves.
//!
//! The hook is argument-free. The caller (PostDetail component) injects
//! the current `FeedPartition` via `use_context_provider` before calling
//! the hook — keeps the controller's shape independent of route state
//! (per hooks-and-actions rule 2: "do not accept args that change the
//! shape of the controller").

use crate::common::components::mention_autocomplete::MentionCandidate;
use crate::common::utils::mention::apply_mention_markup;
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
    /// Signal handles across scopes. Each `CommentItem` derives its
    /// local replies slice via a `memo` over this map keyed by its own sk.
    pub replies_by_comment: Signal<HashMap<String, Vec<PostCommentResponse>>>,
    /// Tracks which parent sks have already been fetched so the lazy-load
    /// effect doesn't re-run on every render.
    pub replies_loaded: Signal<HashSet<String>>,
    /// Per-comment (and per-reply) like state keyed by sk →
    /// `(liked, likes)`. The `toggle_comment_like` action owns the
    /// optimistic flip, the server call, and the rollback on error — all
    /// scoped to the hook. CommentItem/ReplyItem read this via a memo;
    /// no per-item signals get passed into the action body.
    pub comment_likes: Signal<HashMap<String, (bool, i64)>>,

    // Mutations
    pub toggle_like: Action<(), ()>,
    pub share: Action<(), bool>,
    pub submit_comment: Action<(), ()>,
    /// Server-side like toggle for one comment. Takes `(sk, new_liked)`.
    /// Flips the `comment_likes` map optimistically, calls the server,
    /// rolls back on error. Everything runs in the hook (root) scope —
    /// components just fire and observe the map via memo.
    pub toggle_comment_like: Action<(PostCommentTargetEntityType, bool), ()>,
    /// Reply submission. Arg is `(parent_sk, raw_content, mentions)`.
    /// On success, the new reply is prepended into `replies_by_comment`
    /// at `sk` and seeded into `comment_likes` with `(false, 0)`.
    pub submit_reply: Action<
        (
            PostCommentTargetEntityType,
            String,
            Vec<(String, String)>,
        ),
        (),
    >,
    /// Lazy-load replies for one comment. Populates `replies_by_comment`
    /// + seeds `comment_likes` for each reply, and marks the sk seen in
    /// `replies_loaded` so the CommentItem's effect doesn't re-fire.
    pub load_replies: Action<(PostCommentTargetEntityType,), ()>,
}

#[track_caller]
pub fn use_post_detail() -> std::result::Result<UsePostDetail, RenderError> {
    // 1. Cached instance wins — every consumer in this subtree shares one
    //    controller so toggling `comments_open` in the header propagates to
    //    the drawer without extra plumbing.
    if let Some(ctx) = try_use_context::<UsePostDetail>() {
        return Ok(ctx);
    }

    // 2. Read the route-provided post id from context. The `PostDetail`
    //    component is responsible for calling
    //    `use_context_provider(|| post_id)` before invoking this hook —
    //    keeps the hook's shape argument-free per rule 2.
    let post_id: FeedPartition = use_context();
    let post_id_signal: ReadSignal<FeedPartition> = use_signal(|| post_id).into();

    // 3. Server data — guaranteed resolved (non-`RenderError`) by the
    //    time we reach the action bodies because `?` suspends the caller
    //    until `detail` resolves.
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

    // Seed per-comment like state from the loaded detail so the first
    // render reflects server truth without an extra fetch.
    let comment_likes: Signal<HashMap<String, (bool, i64)>> = use_signal(|| {
        snapshot
            .comments
            .iter()
            .map(|c| (c.sk.to_string(), (c.liked, c.likes as i64)))
            .collect()
    });

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
            Ok(_) => Ok::<(), crate::common::Error>(()),
            Err(e) => {
                liked_sig.set(!next);
                like_count_sig.set(prev_count);
                Err(e.into())
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
        Ok::<bool, crate::common::Error>(ok)
    });

    let post_id_for_submit = post_id_signal;
    let mut comment_text_sig = comment_text;
    let mut tracked_mentions_sig = tracked_mentions;
    let mut is_submitting_sig = is_submitting;
    let submit_comment = use_action(move || async move {
        let raw = comment_text_sig().trim().to_string();
        if raw.is_empty() || is_submitting_sig() {
            return Ok::<(), crate::common::Error>(());
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
                Err(e.into())
            }
        }
    });

    let post_id_for_like_comment = post_id_signal;
    let mut comment_likes_for_toggle = comment_likes;
    let toggle_comment_like = use_action(
        move |target_sk: PostCommentTargetEntityType, next_liked: bool| async move {
            let pid = post_id_for_like_comment();
            let sk_str = target_sk.to_string();
            // Capture the prior state so we can roll back on error.
            let prev = comment_likes_for_toggle
                .read()
                .get(&sk_str)
                .copied()
                .unwrap_or((!next_liked, 0));
            let optimistic_count = (prev.1 + if next_liked { 1 } else { -1 }).max(0);
            comment_likes_for_toggle.with_mut(|map| {
                map.insert(sk_str.clone(), (next_liked, optimistic_count));
            });
            match like_comment_handler(pid, target_sk, next_liked).await {
                Ok(_) => Ok::<(), crate::common::Error>(()),
                Err(e) => {
                    // Server rejected the like — put the previous value
                    // back so the heart reverts.
                    comment_likes_for_toggle.with_mut(|map| {
                        map.insert(sk_str, prev);
                    });
                    tracing::error!("like post comment failed: {e}");
                    Err(e.into())
                }
            }
        },
    );

    let post_id_for_reply = post_id_signal;
    let mut replies_map_for_submit = replies_by_comment;
    let mut comment_likes_for_submit = comment_likes;
    let submit_reply = use_action(
        move |parent_sk: PostCommentTargetEntityType,
              raw: String,
              mentions: Vec<(String, String)>| async move {
            let raw = raw.trim().to_string();
            if raw.is_empty() {
                return Ok::<(), crate::common::Error>(());
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
                    // Prepend the new reply into the root-scope map and
                    // seed its like state so a subsequent click on the
                    // reply's heart has an entry to flip. A brand-new
                    // reply has `liked = false, likes = 0`.
                    let new_reply: PostCommentResponse =
                        (new_reply_model, false, false).into();
                    let reply_sk_str = new_reply.sk.to_string();
                    replies_map_for_submit.with_mut(|map| {
                        map.entry(sk_str).or_default().insert(0, new_reply);
                    });
                    comment_likes_for_submit.with_mut(|map| {
                        map.insert(reply_sk_str, (false, 0));
                    });
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
        },
    );

    let post_id_for_list = post_id_signal;
    let mut replies_map_for_load = replies_by_comment;
    let mut replies_loaded_sig = replies_loaded;
    let mut comment_likes_for_load = comment_likes;
    let load_replies = use_action(move |parent_sk: PostCommentTargetEntityType| async move {
        let sk_str = parent_sk.to_string();
        if replies_loaded_sig.read().contains(&sk_str) {
            return Ok::<(), crate::common::Error>(());
        }
        let as_entity: EntityType = parent_sk.into();
        let parent_id: crate::PostCommentEntityType = as_entity.into();
        match list_comments_handler(post_id_for_list(), parent_id, None).await {
            Ok(resp) => {
                // Seed each loaded reply's like state in the shared map
                // so the reply heart buttons start from server truth.
                comment_likes_for_load.with_mut(|map| {
                    for r in resp.items.iter() {
                        map.insert(r.sk.to_string(), (r.liked, r.likes as i64));
                    }
                });
                replies_map_for_load.with_mut(|map| {
                    map.insert(sk_str.clone(), resp.items);
                });
                replies_loaded_sig.with_mut(|s| {
                    s.insert(sk_str);
                });
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    });

    // NOTE: `use_context_provider` (not `provide_root_context`) — same
    // reason as `use_essence_sources`: signals above are owned by the
    // page scope that first called this hook, so caching the controller
    // at the root would outlive its signals on navigation. Scoping the
    // context to the PostDetail subtree ensures unmount cleans up both
    // together and re-entry rebuilds everything fresh.
    Ok(use_context_provider(|| UsePostDetail {
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
        comment_likes,
        toggle_like,
        share,
        submit_comment,
        toggle_comment_like,
        submit_reply,
        load_replies,
    }))
}
