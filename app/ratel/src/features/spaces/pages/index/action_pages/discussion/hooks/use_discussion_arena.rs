use crate::common::components::mention_autocomplete::MentionCandidate;
use crate::common::hooks::{use_infinite_query, use_interval, InfiniteQuery};
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

/// Ranking score for a single comment, computed client-side at every
/// render so the freshness boost stays accurate without server help.
///
/// Components:
/// - `log10(likes + 1) * 3` — popularity, log-scaled so 100→200 votes is
///   a smaller jump than 1→10.
/// - `log10(replies + 1) * 5` — discussion activity, weighted higher
///   than likes because replies are a stronger engagement signal.
/// - `created_at / 86400` — additive freshness baseline (newer always
///   ranks above older, all else equal).
/// - `4 * exp(-age_hours / 1.0)` — short-lived "fresh boost" so a
///   brand-new comment is visible regardless of likes/replies; decays
///   smoothly so there is no UI jump at the 1-hour mark.
///
/// `now_seconds` and `comment.created_at` must share the same unit
/// (seconds since epoch).
pub fn comment_score(comment: &DiscussionCommentResponse, now_seconds: i64) -> f64 {
    let likes = comment.likes as f64;
    let replies = comment.replies as f64;
    let likes_term = (likes + 1.0).log10() * 3.0;
    let replies_term = (replies + 1.0).log10() * 5.0;
    let time_term = comment.created_at as f64 / 86400.0;
    let age_seconds = (now_seconds - comment.created_at).max(0) as f64;
    let age_hours = age_seconds / 3600.0;
    let fresh_term = 4.0 * (-age_hours).exp();
    likes_term + replies_term + time_term + fresh_term
}

/// Merge the paginated base set (from `comments_query.items()`) with the
/// poll-driven `polled_new` buffer and rank the union by `comment_score`.
///
/// Extracted from the `comments` memo in `DiscussionArenaPage` so the full
/// pipeline — base accumulation across pages, polled-new dedup, score-based
/// re-rank — is unit-testable without a Dioxus runtime. Base wins on sk
/// collision so loader refetches after edit/delete clobber stale polled
/// snapshots; the extra `.filter` avoids reshuffling the base order just
/// to drop a duplicate.
pub fn merge_and_rank_comments(
    base: Vec<DiscussionCommentResponse>,
    polled: Vec<DiscussionCommentResponse>,
    now_seconds: i64,
) -> Vec<DiscussionCommentResponse> {
    let base_sks: std::collections::HashSet<String> =
        base.iter().map(|c| c.sk.to_string()).collect();
    let mut merged: Vec<DiscussionCommentResponse> = base
        .into_iter()
        .chain(
            polled
                .into_iter()
                .filter(|p| !base_sks.contains(&p.sk.to_string())),
        )
        .collect();
    merged.sort_by(|a, b| {
        comment_score(b, now_seconds)
            .partial_cmp(&comment_score(a, now_seconds))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    merged
}

#[derive(Clone, Copy, DioxusController)]
pub struct UseDiscussionArena {
    pub active_reply_thread: Signal<Option<String>>,
    pub sheet_expanded: Signal<bool>,
    pub disc_loader: Loader<DiscussionResponse>,
    pub comments_query:
        InfiniteQuery<String, DiscussionCommentResponse, ListResponse<DiscussionCommentResponse>>,
    pub parent_loader: Loader<DiscussionCommentResponse>,
    pub replies_loader: Loader<ListResponse<DiscussionCommentResponse>>,
    pub polled_new: Signal<Vec<DiscussionCommentResponse>>,
    pub last_seen_at: Signal<i64>,
    /// Bumped every poll tick so a `use_memo` over `comment_score` re-runs
    /// even when no new data arrives, letting the time-decay term in the
    /// score reorder comments smoothly as time passes.
    pub sort_tick: Signal<u32>,
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
    /// the next `comments_query.refresh()` drops them for real.
    pub deleted_sks: Signal<std::collections::HashSet<String>>,

    pub like_comment: Action<(String, bool), ()>,
    pub update_comment: Action<(String, String), ()>,
    pub delete_comment: Action<(String,), ()>,
    pub add_comment: Action<(String, Vec<String>), ()>,
    pub reply_comment: Action<(String, String, Vec<String>), ()>,

    /// Bumped whenever a reply successfully posts. CommentItem watches this
    /// to reload its local `replies` signal — the global composer submits
    /// outside the CommentItem's scope, so we need a cross-component signal
    /// to tell the thread-active item that its reply list is stale.
    pub reply_refresh_tick: Signal<u32>,

    /// `(display_name, user_pk)` pushed by ReplyItem when the user clicks
    /// Reply on a reply: the global composer's `use_effect` consumes it
    /// to inject an `@author ` mention pointing at the reply's author,
    /// then clears the slot. Live across the whole arena because the
    /// composer lives outside any individual CommentItem.
    pub pending_mention: Signal<Option<(String, String)>>,
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
    let pending_mention: Signal<Option<(String, String)>> = use_signal(|| None);

    let disc_loader = use_loader(move || async move {
        get_discussion_detail(space_id(), discussion_id()).await
    })?;

    let mut comments_query = use_infinite_query(move |bookmark: Option<String>| async move {
        list_comments(space_id(), discussion_id(), bookmark, None).await
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

    // Reply-aware mention priority needs the active thread's replies live.
    // `replies_loader` above (use_loader) does not reliably re-run on
    // `active_reply_thread` changes after SSR hydration — same race that
    // broke `members_loader`. Drive a dedicated fetch from `use_effect`
    // instead so the priority memo always sees current thread replies.
    let mut thread_replies: Signal<Vec<DiscussionCommentResponse>> = use_signal(Vec::new);
    use_effect(move || {
        let id = active_reply_thread();
        spawn(async move {
            let items = match id {
                None => Vec::new(),
                Some(parent_id) => match list_replies(
                    space_id(),
                    discussion_id(),
                    SpacePostCommentEntityType(parent_id),
                    None,
                )
                .await
                {
                    Ok(r) => r.items,
                    Err(e) => {
                        crate::error!("thread_replies fetch failed: {e}");
                        Vec::new()
                    }
                },
            };
            thread_replies.set(items);
        });
    });

    let mut polled_new: Signal<Vec<DiscussionCommentResponse>> = use_signal(Vec::new);
    let mut last_seen_at: Signal<i64> = use_signal(move || {
        comments_query
            .items()
            .iter()
            .map(|c| c.created_at)
            .max()
            .unwrap_or_else(crate::common::utils::time::get_now_timestamp)
    });
    let mut sort_tick: Signal<u32> = use_signal(|| 0);
    use_interval(5000, move || {
        // Bump unconditionally so the score-sort memo re-runs every tick,
        // even when no new comments arrived (the fresh-boost decays with
        // time so positions need to update independently of fetch results).
        sort_tick.with_mut(|t| *t = t.wrapping_add(1));
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

    // `use_loader` does not reliably re-run on prop signal changes after SSR
    // hydration in Dioxus 0.7 — confirmed empirically: even when
    // `mention_query` flips from None to Some(""), the loader closure never
    // fires post-hydration. Drive the fetch from `use_effect` (which does
    // re-run reliably, as the debounce above demonstrates) and store
    // results in a plain Signal that downstream `use_memo` reads.
    let mut members_signal: Signal<Vec<SpaceMemberResponse>> = use_signal(Vec::new);
    use_effect(move || {
        let q = mention_query();
        spawn(async move {
            let items = match q {
                None => Vec::new(),
                Some(q) => match list_space_members(space_id(), None, Some(q)).await {
                    Ok(r) => r.items,
                    Err(e) => {
                        crate::error!("mention members fetch failed: {e}");
                        Vec::new()
                    }
                },
            };
            members_signal.set(items);
        });
    });

    let members_memo = use_memo(move || {
        members_signal()
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
        let base = comments_query.items();
        let polled = polled_new();
        let base_sks: std::collections::HashSet<String> =
            base.iter().map(|c| c.sk.to_string()).collect();

        // Reply mode: hoist parent comment author as primary, reply
        // authors as the thread. Falls back to the discussion-wide
        // ordering below if the parent isn't in the loaded base/polled
        // set (very old thread scrolled past).
        if let Some(parent_id) = active_reply_thread() {
            let parent_sk_str =
                EntityType::SpacePostComment(parent_id).to_string();
            let parent = base
                .iter()
                .chain(polled.iter())
                .find(|c| c.sk.to_string() == parent_sk_str);

            if let Some(parent) = parent {
                let replies = thread_replies();
                let parent_tuple =
                    (parent.author_pk.to_string(), parent.content.clone());
                let reply_tuples: Vec<(String, String)> = replies
                    .iter()
                    .map(|r| (r.author_pk.to_string(), r.content.clone()))
                    .collect();

                let primary_ref = (parent_tuple.0.as_str(), parent_tuple.1.as_str());
                let thread_refs: Vec<(&str, &str)> = reply_tuples
                    .iter()
                    .map(|(a, c)| (a.as_str(), c.as_str()))
                    .collect();

                return crate::common::utils::mention::build_mention_priority(
                    Some(primary_ref),
                    &thread_refs,
                );
            }
        }

        // Non-reply mode: every loaded participant gets equal-rank
        // priority over non-participants (no single primary).
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
    let mut reply_refresh_tick: Signal<u32> = use_signal(|| 0);

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
                comments_query.refresh();
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
                comments_query.refresh();
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
                comments_query.refresh();
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
                    comments_query.refresh();
                    replies_loader.restart();
                    parent_loader.restart();
                    reply_refresh_tick.with_mut(|t| *t = t.wrapping_add(1));
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
        comments_query,
        parent_loader,
        replies_loader,
        polled_new,
        last_seen_at,
        sort_tick,
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
        reply_refresh_tick,
        pending_mention,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a fixture comment with only the fields `comment_score` reads.
    fn fixture(created_at: i64, likes: u64, replies: u64) -> DiscussionCommentResponse {
        DiscussionCommentResponse {
            created_at,
            likes,
            replies,
            ..Default::default()
        }
    }

    // ── Sanity: monotonicity ──────────────────────────────────────

    #[test]
    fn more_likes_score_higher() {
        let now = 1_000_000;
        let same_age = now - 3600;
        let a = fixture(same_age, 0, 0);
        let b = fixture(same_age, 10, 0);
        assert!(comment_score(&b, now) > comment_score(&a, now));
    }

    #[test]
    fn more_replies_score_higher() {
        let now = 1_000_000;
        let same_age = now - 3600;
        let a = fixture(same_age, 0, 0);
        let b = fixture(same_age, 0, 10);
        assert!(comment_score(&b, now) > comment_score(&a, now));
    }

    #[test]
    fn newer_score_higher_when_engagement_equal() {
        let now = 1_000_000;
        let new = fixture(now, 5, 5);
        let old = fixture(now - 86400, 5, 5);
        assert!(comment_score(&new, now) > comment_score(&old, now));
    }

    // ── Algorithm intent ──────────────────────────────────────────

    #[test]
    fn fresh_comment_outranks_older_silent_comment() {
        // Requirement: "1시간까지는 좋아요/대댓글 없어도 높이 보여야 함"
        let now = 1_000_000;
        let fresh = fixture(now - 60, 0, 0); // 1 min old
        let old_silent = fixture(now - 7200, 0, 0); // 2 h old
        assert!(comment_score(&fresh, now) > comment_score(&old_silent, now));
    }

    #[test]
    fn replies_weighted_higher_than_likes() {
        // Same age, same engagement count but different KIND of engagement —
        // replies should win because they're a stronger discussion signal.
        let now = 1_000_000;
        let same_age = now - 3600;
        let liked = fixture(same_age, 100, 0);
        let replied = fixture(same_age, 0, 100);
        assert!(comment_score(&replied, now) > comment_score(&liked, now));
    }

    #[test]
    fn popular_old_comment_beats_brand_new_silent() {
        // A meaningfully engaged comment should still outrank a silent fresh
        // one — fresh boost shouldn't completely dominate.
        let now = 1_000_000;
        let popular_old = fixture(now - 21600, 50, 5); // 6h old, 50 likes 5 replies
        let new_silent = fixture(now, 0, 0);
        assert!(comment_score(&popular_old, now) > comment_score(&new_silent, now));
    }

    // ── Curve shape ────────────────────────────────────────────────

    #[test]
    fn fresh_boost_decays_within_a_few_hours() {
        // Boost gap between 1h and 10h olds should be at least ~1 score
        // point so the time-driven re-sort is observable across the 5s tick.
        let now = 1_000_000;
        let one_h = comment_score(&fixture(now - 3600, 0, 0), now);
        let ten_h = comment_score(&fixture(now - 36000, 0, 0), now);
        assert!(one_h - ten_h > 1.0, "1h={one_h}, 10h={ten_h}");
    }

    #[test]
    fn likes_score_is_sublinear() {
        // Mega-popular comments must NOT dominate forever. A 100-like comment
        // should score far less than 100× a 1-like comment — that's the whole
        // point of log scaling.
        let now = 1_000_000;
        let same_age = now - 3600;
        let s_1 = comment_score(&fixture(same_age, 1, 0), now);
        let s_100 = comment_score(&fixture(same_age, 100, 0), now);
        // Linear scaling would give roughly 100× the increment. Log scaling
        // should keep it under, say, 10× — comfortably sublinear.
        assert!(
            (s_100 - s_1) < 10.0 * (s_1 - 0.0),
            "log scale broken: s(1)={s_1}, s(100)={s_100}"
        );
    }

    #[test]
    fn likes_score_jumps_stabilize_at_high_counts() {
        // Above ~10, each decade of likes adds a near-constant ~3 points
        // (log10(10) × W_L). Verifies the curve flattens — once a comment
        // is "popular enough", more likes barely change ranking.
        let now = 1_000_000;
        let same_age = now - 3600;
        let s_100 = comment_score(&fixture(same_age, 100, 0), now);
        let s_1000 = comment_score(&fixture(same_age, 1_000, 0), now);
        let s_10000 = comment_score(&fixture(same_age, 10_000, 0), now);
        let jump_a = s_1000 - s_100;
        let jump_b = s_10000 - s_1000;
        let diff = (jump_a - jump_b).abs();
        // At decades this large, +1 offset is negligible, so jumps should be
        // essentially equal.
        assert!(
            diff < 0.05,
            "decade jumps not stabilized: 100→1k={jump_a}, 1k→10k={jump_b}"
        );
    }

    // ── Edge cases / safety ──────────────────────────────────────

    #[test]
    fn negative_age_does_not_panic() {
        // Clock skew: comment "from the future".
        let now = 1_000_000;
        let future = fixture(now + 3600, 5, 0);
        let _ = comment_score(&future, now); // must not panic
    }

    #[test]
    fn zero_engagement_zero_age_safe() {
        // Default-constructed comment (all zeros) must produce a finite score.
        let s = comment_score(&DiscussionCommentResponse::default(), 0);
        assert!(s.is_finite(), "got {s}");
    }

    // ── Realistic mixed scenario ──────────────────────────────────

    #[test]
    fn realistic_ordering_matches_intent() {
        // Mirrors the simulation table from the design discussion.
        let now = 1_000_000;
        let scenarios = vec![
            ("just_now",          fixture(now,           0, 0)),
            ("30min_silent",      fixture(now - 1800,    0, 0)),
            ("1h_silent",         fixture(now - 3600,    0, 0)),
            ("1h_5_likes",        fixture(now - 3600,    5, 0)),
            ("6h_popular",        fixture(now - 21600,   50, 5)),
            ("1d_modest",         fixture(now - 86400,   10, 1)),
            ("7d_classic",        fixture(now - 604800,  100, 20)),
        ];
        let mut scored: Vec<(&str, f64)> = scenarios
            .iter()
            .map(|(k, c)| (*k, comment_score(c, now)))
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let order: Vec<&str> = scored.iter().map(|(k, _)| *k).collect();

        // Top should be the engaged comment (popular 6h-old).
        assert_eq!(order[0], "6h_popular", "ranking={scored:?}");
        // Brand-new silent comment must beat the older silent ones.
        let pos = |k: &str| order.iter().position(|x| *x == k).unwrap();
        assert!(pos("just_now") < pos("30min_silent"));
        assert!(pos("just_now") < pos("1h_silent"));
        // 1h-old with 5 likes outranks 1h-old silent.
        assert!(pos("1h_5_likes") < pos("1h_silent"));
    }

    // ── merge_and_rank_comments: pagination accumulation + polling ────

    /// Fixture that lets each test produce distinct sks so the dedup
    /// filter in `merge_and_rank_comments` isn't short-circuited.
    fn fixture_with_sk(
        sk_id: &str,
        created_at: i64,
        likes: u64,
        replies: u64,
    ) -> DiscussionCommentResponse {
        DiscussionCommentResponse {
            sk: EntityType::SpacePostComment(sk_id.to_string()),
            created_at,
            likes,
            replies,
            ..Default::default()
        }
    }

    #[test]
    fn merge_preserves_all_items_from_base_and_polled() {
        let now = 1_000_000;
        let base = vec![
            fixture_with_sk("a", now - 3600, 10, 0),
            fixture_with_sk("b", now - 3600, 5, 0),
        ];
        let polled = vec![fixture_with_sk("c", now - 60, 0, 0)];

        let merged = merge_and_rank_comments(base, polled, now);
        assert_eq!(merged.len(), 3);
    }

    #[test]
    fn merge_dedupes_polled_against_base() {
        // Polling fires after a refresh — the same comment may appear in
        // both the base page and the polled buffer. Base must win (latest
        // server truth) and the polled copy is dropped.
        let now = 1_000_000;
        let base = vec![fixture_with_sk("shared", now - 3600, 10, 0)];
        let polled = vec![
            fixture_with_sk("shared", now - 3600, 999, 999), // stale counters
            fixture_with_sk("new_only", now - 60, 0, 0),
        ];

        let merged = merge_and_rank_comments(base, polled, now);
        assert_eq!(merged.len(), 2, "shared comment must not appear twice");
        // The surviving `shared` entry must be base's copy (likes=10), not
        // polled's stale likes=999.
        let shared = merged
            .iter()
            .find(|c| c.sk.to_string().contains("shared"))
            .expect("shared comment missing");
        assert_eq!(shared.likes, 10);
    }

    #[test]
    fn paginated_pages_re_sort_across_page_boundary() {
        // Simulate what use_infinite_query produces after the user scrolls
        // past page 1: `comments_query.items()` returns the union of page 1
        // and page 2, concatenated in server fetch order (likes DESC). The
        // client must re-rank the whole accumulated set — a page-2 item
        // with a higher `comment_score` must end up ABOVE page-1 items even
        // though the server returned it later.
        let now = 1_000_000;
        // Page 1 — server order: older items with modest likes.
        let page1 = vec![
            fixture_with_sk("p1-stale-likes", now - 172800, 50, 0), // 2d old, 50 likes
            fixture_with_sk("p1-tail", now - 172800, 20, 0),
        ];
        // Page 2 — appears later in server order but carries a 1h-old
        // comment with heavy reply engagement. `replies_term` + recency
        // push its score above page 1.
        let page2 = vec![
            fixture_with_sk("p2-discussion", now - 3600, 0, 20), // 1h old, 20 replies
        ];
        let accumulated: Vec<_> = page1.into_iter().chain(page2).collect();

        let merged = merge_and_rank_comments(accumulated, vec![], now);
        let order: Vec<String> = merged.iter().map(|c| c.sk.to_string()).collect();

        // Page 2's discussion-heavy comment must float above the concat
        // order and reach index 0 — confirms the re-sort crosses the page
        // boundary.
        assert_eq!(
            merged[0].sk.to_string(),
            EntityType::SpacePostComment("p2-discussion".into()).to_string(),
            "re-sort should lift page-2 engagement above older page-1 likes: {order:?}"
        );
    }

    #[test]
    fn polled_engaged_comment_surfaces_above_paginated_base() {
        // The 5s poll buffer is meant to bring in freshly-created comments
        // without waiting for the user to scroll to the relevant page. When
        // the polled comment has enough engagement that its `comment_score`
        // beats every accumulated item, merge_and_rank must place it first.
        let now = 1_000_000;
        let accumulated_base = vec![
            fixture_with_sk("old_medium", now - 86400, 20, 2),
            fixture_with_sk("old_silent", now - 172800, 0, 0),
        ];
        // Polled item: 1 min old with non-trivial engagement — strong
        // fresh_term + some likes + replies → clearly highest score.
        let polled_fresh = vec![fixture_with_sk("viral_new", now - 60, 10, 3)];

        let merged = merge_and_rank_comments(accumulated_base, polled_fresh, now);
        assert_eq!(
            merged[0].sk.to_string(),
            EntityType::SpacePostComment("viral_new".into()).to_string(),
            "engaged polled comment must rank first; got={:?}",
            merged.iter().map(|c| c.sk.to_string()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn merge_is_stable_across_repeated_calls_with_same_inputs() {
        // Guard against ordering drift caused by an unstable comparator.
        // Two invocations with identical inputs must yield the same order.
        let now = 1_000_000;
        let base = vec![
            fixture_with_sk("a", now - 3600, 10, 2),
            fixture_with_sk("b", now - 7200, 20, 1),
            fixture_with_sk("c", now - 86400, 100, 5),
        ];
        let polled = vec![fixture_with_sk("d", now - 120, 0, 0)];

        let first = merge_and_rank_comments(base.clone(), polled.clone(), now);
        let second = merge_and_rank_comments(base, polled, now);
        let sks = |v: &[DiscussionCommentResponse]| {
            v.iter().map(|c| c.sk.to_string()).collect::<Vec<_>>()
        };
        assert_eq!(sks(&first), sks(&second));
    }
}
