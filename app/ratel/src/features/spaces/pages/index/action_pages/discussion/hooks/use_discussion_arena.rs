use crate::common::components::mention_autocomplete::MentionCandidate;
use crate::common::hooks::use_interval;
use crate::common::*;
use crate::features::spaces::pages::actions::actions::discussion::controllers::{
    add_comment, delete_comment, get_comment, get_discussion_detail, like_comment, list_comments,
    list_replies, reply_comment, update_comment, AddCommentRequest, LikeCommentRequest,
    ReplyCommentRequest, UpdateCommentRequest,
};
use crate::features::spaces::pages::actions::actions::discussion::{
    DiscussionCommentResponse, DiscussionResponse, SpacePostCommentTargetEntityType,
};
use crate::features::spaces::space_common::controllers::{list_space_members, SpaceMemberResponse};
use dioxus::fullstack::Loader;
use std::str::FromStr;

#[derive(Clone, Copy, DioxusController)]
pub struct UseDiscussionArena {
    pub active_reply_thread: Signal<Option<String>>,
    pub sheet_expanded: Signal<bool>,
    pub disc_loader: Loader<DiscussionResponse>,
    pub comments_loader: Loader<ListResponse<DiscussionCommentResponse>>,
    pub parent_loader: Loader<DiscussionCommentResponse>,
    pub replies_loader: Loader<ListResponse<DiscussionCommentResponse>>,
    pub members_loader: Loader<ListResponse<SpaceMemberResponse>>,
    pub polled_new: Signal<Vec<DiscussionCommentResponse>>,
    pub last_seen_at: Signal<i64>,
    pub mention_query_raw: Signal<Option<String>>,
    pub mention_query: Signal<Option<String>>,
    pub members: ReadSignal<Vec<MentionCandidate>>,
    pub top_priority: ReadSignal<Vec<String>>,

    /// Optimistic like toggles keyed by comment sk. Tuple is
    /// `(liked_override, likes_delta)` — delta is relative to the
    /// base `comment.likes` from the loader/polled buffer so swings
    /// across rollbacks + repeat toggles stay consistent.
    pub like_overlays: Signal<std::collections::HashMap<String, (bool, i64)>>,

    /// Content patches written by an in-flight or just-resolved edit;
    /// read via `effective_content` so the pre-loader-restart frame
    /// doesn't flash the old text.
    pub content_overlays: Signal<std::collections::HashMap<String, String>>,

    /// Soft-delete set — components skip rendering sks in here until
    /// the next `comments_loader.restart()` drops them for real.
    pub deleted_sks: Signal<std::collections::HashSet<String>>,

    pub like_comment: Action<(String, bool), ()>,
    pub update_comment: Action<(String, String), ()>,
    pub delete_comment: Action<(String,), ()>,
    pub add_comment: Action<(String, Vec<String>), ()>,
    pub reply_comment: Action<(String, String, Vec<String>), ()>,
}

impl UseDiscussionArena {
    pub fn effective_liked(&self, comment: &DiscussionCommentResponse) -> bool {
        self.like_overlays
            .read()
            .get(&comment.sk.to_string())
            .map(|(l, _)| *l)
            .unwrap_or(comment.liked)
    }

    pub fn effective_likes(&self, comment: &DiscussionCommentResponse) -> i64 {
        let base = comment.likes as i64;
        // Derive the optimistic delta from `intent vs comment.liked` instead of
        // trusting the stored `delta` field. The stored delta accumulates across
        // toggles and stays positive after the action succeeds — so once a loader
        // refetch lands (e.g. `replies_loader.restart()` after posting a new reply,
        // or `parent_loader` re-running when the thread view reopens) and `base`
        // already reflects the like, the old overlay delta gets stacked on top
        // and the count shows +1 too high. Re-deriving from `liked` makes the
        // overlay self-cancel as soon as the server side catches up.
        let delta = self
            .like_overlays
            .read()
            .get(&comment.sk.to_string())
            .map(|(intent, _)| {
                if *intent == comment.liked {
                    0
                } else if *intent {
                    1
                } else {
                    -1
                }
            })
            .unwrap_or(0);
        (base + delta).max(0)
    }

    pub fn effective_content(&self, comment: &DiscussionCommentResponse) -> String {
        self.content_overlays
            .read()
            .get(&comment.sk.to_string())
            .cloned()
            .unwrap_or_else(|| comment.content.clone())
    }

    pub fn is_deleted(&self, comment: &DiscussionCommentResponse) -> bool {
        self.deleted_sks.read().contains(&comment.sk.to_string())
    }
}

#[track_caller]
pub fn use_discussion_arena(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> std::result::Result<UseDiscussionArena, RenderError> {
    if let Some(ctx) = try_use_context::<UseDiscussionArena>() {
        return Ok(ctx);
    }

    let active_reply_thread: Signal<Option<String>> = use_signal(|| None);
    let sheet_expanded: Signal<bool> = use_signal(|| false);

    let disc_loader = use_loader(move || async move {
        get_discussion_detail(space_id(), discussion_id()).await
    })?;

    let mut comments_loader = use_loader(move || async move {
        list_comments(space_id(), discussion_id(), None, None).await
    })?;

    // Default-on-None keeps the initial mount synchronous and turns
    // future flips into background refreshes, so `?` never suspends up
    // to the overlay's SuspenseBoundary mid-session.
    let mut parent_loader = use_loader(move || async move {
        let Some(id) = active_reply_thread() else {
            return Ok::<DiscussionCommentResponse, crate::common::Error>(
                DiscussionCommentResponse::default(),
            );
        };
        get_comment(
            space_id(),
            discussion_id(),
            SpacePostCommentEntityType(id),
        )
        .await
    })?;

    let mut replies_loader = use_loader(move || async move {
        let Some(id) = active_reply_thread() else {
            return Ok::<ListResponse<DiscussionCommentResponse>, crate::common::Error>(
                ListResponse::<DiscussionCommentResponse>::default(),
            );
        };
        list_replies(
            space_id(),
            discussion_id(),
            SpacePostCommentEntityType(id),
            None,
        )
        .await
    })?;

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
                        // Reverse the newest-first server order so chronological
                        // append keeps existing items at stable indices.
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
            None => Ok(ListResponse::<SpaceMemberResponse>::default()),
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

    let mut like_overlays: Signal<std::collections::HashMap<String, (bool, i64)>> =
        use_signal(std::collections::HashMap::new);
    let mut content_overlays: Signal<std::collections::HashMap<String, String>> =
        use_signal(std::collections::HashMap::new);
    let mut deleted_sks: Signal<std::collections::HashSet<String>> =
        use_signal(std::collections::HashSet::new);

    let mut toast = use_toast();
    let like_comment = use_action(move |sk_str: String, next: bool| async move {
        let prev_overlay = like_overlays.read().get(&sk_str).cloned();
        let prev_delta = prev_overlay.as_ref().map(|(_, d)| *d).unwrap_or(0);
        let new_delta = prev_delta + if next { 1 } else { -1 };
        like_overlays
            .write()
            .insert(sk_str.clone(), (next, new_delta));

        let rollback = |sks: String, prev: Option<(bool, i64)>| {
            let mut like_overlays = like_overlays;
            match prev {
                Some(p) => {
                    like_overlays.write().insert(sks, p);
                }
                None => {
                    like_overlays.write().remove(&sks);
                }
            }
        };

        let target_sk = match SpacePostCommentTargetEntityType::from_str(&sk_str) {
            Ok(v) => v,
            Err(e) => {
                rollback(sk_str, prev_overlay);
                tracing::error!("like: invalid sk: {:?}", e);
                toast.error(e);
                return Ok::<(), crate::common::Error>(());
            }
        };
        let req = LikeCommentRequest { like: next };
        if let Err(e) = like_comment(space_id(), discussion_id(), target_sk, req).await {
            rollback(sk_str, prev_overlay);
            tracing::error!("like failed: {:?}", e);
            toast.error(e);
        }
        Ok::<(), crate::common::Error>(())
    });

    let update_comment = use_action(move |sk_str: String, new_content: String| async move {
        let target_sk = match SpacePostCommentTargetEntityType::from_str(&sk_str) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("update: invalid sk: {:?}", e);
                toast.error(e);
                return Ok::<(), crate::common::Error>(());
            }
        };
        let prev_overlay = content_overlays.read().get(&sk_str).cloned();
        content_overlays
            .write()
            .insert(sk_str.clone(), new_content.clone());

        let req = UpdateCommentRequest {
            content: new_content,
            images: None,
        };
        match update_comment(space_id(), discussion_id(), target_sk, req).await {
            Ok(_) => {
                comments_loader.restart();
            }
            Err(e) => {
                match prev_overlay {
                    Some(p) => {
                        content_overlays.write().insert(sk_str, p);
                    }
                    None => {
                        content_overlays.write().remove(&sk_str);
                    }
                }
                tracing::error!("update failed: {:?}", e);
                toast.error(e);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let delete_comment = use_action(move |sk_str: String| async move {
        let target_sk = match SpacePostCommentTargetEntityType::from_str(&sk_str) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("delete: invalid sk: {:?}", e);
                toast.error(e);
                return Ok::<(), crate::common::Error>(());
            }
        };
        let was_present = deleted_sks.write().insert(sk_str.clone());
        match delete_comment(space_id(), discussion_id(), target_sk).await {
            Ok(_) => {
                // Drop any polled snapshot so polling (which only fires
                // on created_at > cursor) doesn't resurrect the entry.
                polled_new.with_mut(|list| list.retain(|c| c.sk.to_string() != sk_str));
                comments_loader.restart();
                // Thread view: parent's reply count lives on parent_loader.
                parent_loader.restart();
                deleted_sks.write().remove(&sk_str);
            }
            Err(e) => {
                if was_present {
                    deleted_sks.write().remove(&sk_str);
                }
                tracing::error!("delete failed: {:?}", e);
                toast.error(e);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let add_comment_action = use_action(move |content: String, images: Vec<String>| async move {
        let req = AddCommentRequest { content, images };
        match add_comment(space_id(), discussion_id(), req).await {
            Ok(_) => {
                comments_loader.restart();
            }
            Err(e) => {
                tracing::error!("add comment: {:?}", e);
                toast.error(e);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let reply_comment_action = use_action(
        move |parent_sk: String, content: String, images: Vec<String>| async move {
            // Strip the `SPACE_POST_COMMENT#` prefix via Target's FromStr,
            // then rewrap as the narrower type the reply endpoint wants.
            let comment_sk_entity =
                match SpacePostCommentTargetEntityType::from_str(&parent_sk) {
                    Ok(v) => SpacePostCommentEntityType(v.0),
                    Err(e) => {
                        tracing::error!("reply: invalid parent sk: {:?}", e);
                        toast.error(e);
                        return Ok::<(), crate::common::Error>(());
                    }
                };
            let req = ReplyCommentRequest { content, images };
            match reply_comment(space_id(), discussion_id(), comment_sk_entity, req).await {
                Ok(_) => {
                    comments_loader.restart();
                    replies_loader.restart();
                    parent_loader.restart();
                }
                Err(e) => {
                    tracing::error!("reply comment: {:?}", e);
                    toast.error(e);
                }
            }
            Ok(())
        },
    );

    Ok(use_context_provider(|| UseDiscussionArena {
        active_reply_thread,
        sheet_expanded,
        disc_loader,
        comments_loader,
        parent_loader,
        replies_loader,
        members_loader,
        polled_new,
        last_seen_at,
        mention_query_raw,
        mention_query,
        members,
        top_priority,
        like_overlays,
        content_overlays,
        deleted_sks,
        like_comment,
        update_comment,
        delete_comment,
        add_comment: add_comment_action,
        reply_comment: reply_comment_action,
    }))
}
