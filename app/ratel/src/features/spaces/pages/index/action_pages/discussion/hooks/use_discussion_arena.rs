use crate::common::components::mention_autocomplete::MentionCandidate;
use crate::common::hooks::use_interval;
use crate::common::*;
use crate::features::spaces::pages::actions::actions::discussion::controllers::{
    get_comment, get_discussion_detail, list_comments, list_replies,
};
use crate::features::spaces::pages::actions::actions::discussion::{
    DiscussionCommentResponse, DiscussionResponse,
};
use crate::features::spaces::space_common::controllers::{list_space_members, SpaceMemberResponse};
use dioxus::fullstack::Loader;

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

    let comments_loader = use_loader(move || async move {
        list_comments(space_id(), discussion_id(), None, None).await
    })?;

    // Default-on-None keeps the initial mount synchronous and turns
    // future flips into background refreshes, so `?` never suspends up
    // to the overlay's SuspenseBoundary mid-session.
    let parent_loader = use_loader(move || async move {
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

    let replies_loader = use_loader(move || async move {
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
    }))
}
