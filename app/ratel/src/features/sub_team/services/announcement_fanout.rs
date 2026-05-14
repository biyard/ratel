use std::collections::HashSet;

use crate::common::*;
use crate::features::auth::UserTeam;
use crate::features::posts::models::{Post, Team, TeamOwner};
use crate::features::posts::types::{PostStatus, PostType, Visibility};
use crate::features::sub_team::models::{
    SubTeamAnnouncement, SubTeamAnnouncementStatus, SubTeamLink,
};
use crate::features::sub_team::types::SubTeamError;

const PAGE_SIZE: i32 = 100;
const MAX_PAGES: usize = 10;
/// Phase 1 cap — anything beyond this returns BroadcastTooManySubTeams at
/// publish time, so we should never fan out to more than this here either.
pub const MAX_RECOGNIZED_SUB_TEAMS: usize = 50;

/// Resolve every member user pk for a team (UserTeam rows + owner). Used for
/// fan-out inbox notifications.
pub async fn resolve_team_members(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<Vec<Partition>> {
    let mut user_pks: HashSet<String> = HashSet::new();

    if let Ok(Some(owner)) = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await {
        user_pks.insert(owner.user_pk.to_string());
    }

    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let mut opt = crate::features::auth::UserTeamQueryOption::builder().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) = UserTeam::find_by_team(cli, &user_team_sk, opt).await?;
        for row in rows {
            user_pks.insert(row.pk.to_string());
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(user_pks
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok())
        .collect())
}

/// Count recognized sub-teams for a parent team (used for publish-time
/// validation against the 50-team cap).
pub async fn count_recognized_sub_teams(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
) -> Result<usize> {
    let opt = SubTeamLink::opt().sk("SUB_TEAM_LINK".to_string()).limit(
        (MAX_RECOGNIZED_SUB_TEAMS as i32) + 1,
    );
    let (items, _) = SubTeamLink::query(cli, parent_pk.clone(), opt)
        .await
        .map_err(|e| {
            crate::error!("count_recognized_sub_teams query failed: {e}");
            SubTeamError::BroadcastTooManySubTeams
        })?;
    Ok(items.len())
}

/// List recognized sub-teams (bounded — cap enforced by caller).
pub async fn list_recognized_sub_teams(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
) -> Result<Vec<SubTeamLink>> {
    let opt = SubTeamLink::opt()
        .sk("SUB_TEAM_LINK".to_string())
        .limit((MAX_RECOGNIZED_SUB_TEAMS as i32) + 1);
    let (items, _) = SubTeamLink::query(cli, parent_pk.clone(), opt)
        .await
        .map_err(|e| {
            crate::error!("list_recognized_sub_teams query failed: {e}");
            SubTeamError::AnnouncementPublishFailed
        })?;
    Ok(items)
}

/// Fan-out handler for an announcement that has just been flipped to
/// Published. Called from `EventBridgeEnvelope::proc` in deployed envs and
/// directly from `stream_handler` in local-dev.
///
/// For each recognized sub-team, in one transact-write batch:
///   - create a Post (pinned_as_announcement = true) in the child team's feed,
///     authored by the parent team
///   - demote the most-recent prior announcement post (if any)
///
/// After all fan-outs, update the source announcement's fan_out_count.
/// Members of each sub-team receive an inbox notification.
pub async fn handle_announcement_published(
    cli: &aws_sdk_dynamodb::Client,
    announcement: SubTeamAnnouncement,
) -> Result<()> {
    if !matches!(announcement.status, SubTeamAnnouncementStatus::Published) {
        tracing::debug!(
            "handle_announcement_published: ignoring status={:?}",
            announcement.status
        );
        return Ok(());
    }
    // Idempotency. The CDK Pipe filter only checks `status: "Published"`,
    // and this function writes a `fan_out_count` update at the end —
    // that write is itself a MODIFY event with status=Published, so the
    // Lambda would re-enter without this guard and fan out repeatedly.
    if announcement.fan_out_count > 0 {
        tracing::debug!(
            "handle_announcement_published: already fanned out ({}), skipping",
            announcement.fan_out_count
        );
        return Ok(());
    }

    let parent_team_id = match &announcement.pk {
        Partition::Team(id) => id.clone(),
        other => {
            crate::error!(
                "handle_announcement_published: unexpected pk variant: {:?}",
                other
            );
            return Err(SubTeamError::AnnouncementPublishFailed.into());
        }
    };

    // Load parent team once so every fan-out Post can reuse the author copy.
    let parent_team = match Team::get(cli, &announcement.pk, Some(EntityType::Team)).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            crate::error!(
                "handle_announcement_published: parent team not found: {parent_team_id}"
            );
            return Err(SubTeamError::AnnouncementPublishFailed.into());
        }
        Err(e) => {
            crate::error!("handle_announcement_published: parent team fetch: {e}");
            return Err(SubTeamError::AnnouncementPublishFailed.into());
        }
    };

    // Direct messages target a single recognized sub-team — verify the
    // link still exists (the relationship may have been deregistered
    // between send time and stream delivery) and bypass the cap check.
    // Broadcast announcements use the full recognized-sub-teams list.
    let links: Vec<SubTeamLink> = if let Some(target_id) = &announcement.target_child_team_id {
        let link_sk = EntityType::SubTeamLink(target_id.clone());
        match SubTeamLink::get(cli, &announcement.pk, Some(link_sk)).await {
            Ok(Some(link)) => vec![link],
            Ok(None) => {
                tracing::warn!(
                    "[fanout] direct-message target {} is not (or no longer) a recognized \
                     sub-team of {}; skipping",
                    target_id,
                    parent_team_id,
                );
                return Ok(());
            }
            Err(e) => {
                crate::error!("[fanout] direct-message link lookup failed: {e}");
                return Err(SubTeamError::AnnouncementPublishFailed.into());
            }
        }
    } else {
        list_recognized_sub_teams(cli, &announcement.pk).await?
    };
    if links.len() > MAX_RECOGNIZED_SUB_TEAMS {
        crate::error!(
            "handle_announcement_published: {} > MAX_RECOGNIZED_SUB_TEAMS for parent {}",
            links.len(),
            parent_team_id
        );
        return Err(SubTeamError::BroadcastTooManySubTeams.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut fan_out_count: i32 = 0;

    // Prefer rich-text body when present; legacy plain `body` is the
    // fallback for rows created before the composer upgrade.
    let post_body = if !announcement.html_contents.is_empty() {
        announcement.html_contents.clone()
    } else {
        announcement.body.clone()
    };
    // If the announcement created a Space, every Post (parent + children)
    // points to the same Space pk (read at the parent pk's announcement
    // row).
    let space_pk: Option<Partition> = announcement
        .space_pk
        .as_ref()
        .and_then(|s| s.parse::<Partition>().ok());

    // NB: the parent's own anchor Post is created synchronously in
    // `publish_announcement_handler` (so Space creation can hang off
    // it and `get_space_handler` can resolve the Post back). The fanout
    // is therefore responsible for the child Posts only.

    for link in &links {
        let child_team_id = link.child_team_id.clone();
        let child_team_pk: Partition = Partition::Team(child_team_id.clone());
        // Direct messages have a 1:1 announcement↔post relationship, so
        // we pin the fan-out Post pk to the announcement_id. That gives
        // the parent's history rows a deterministic `/posts/{id}` link
        // without an extra "find post by announcement_id" lookup.
        // Broadcasts still use random uuids per child (N children →
        // N posts → can't all share one pk).
        let post_pk = if announcement.target_child_team_id.is_some() {
            Partition::Feed(announcement.announcement_id.clone())
        } else {
            Partition::Feed(uuid::Uuid::now_v7().to_string())
        };
        let mut new_post = Post {
            pk: post_pk,
            sk: EntityType::Post,
            created_at: now,
            updated_at: now,
            title: announcement.title.clone(),
            body: ContentBody::html(post_body.clone()),
            post_type: PostType::Post,
            status: PostStatus::Published,
            visibility: Some(announcement.visibility.clone()),
            shares: 0,
            likes: 0,
            comments: 0,
            reports: 0,
            user_pk: child_team_pk.clone(),
            author_display_name: parent_team.display_name.clone(),
            author_profile_url: parent_team.profile_url.clone(),
            author_username: parent_team.username.clone(),
            author_type: crate::common::types::UserType::Team,
            space_pk: space_pk.clone(),
            space_type: announcement.space_type,
            space_visibility: None,
            booster: None,
            rewards: None,
            urls: vec![],
            categories: announcement.tags.clone(),
            announcement_id: Some(announcement.announcement_id.clone()),
            announcement_parent_team_id: Some(parent_team_id.clone()),
            pinned_as_announcement: true,
        };

        // Write the new fanned-out Post. Earlier versions also "demoted"
        // the previously pinned announcement post here via
        // `Post::updater(..).with_pinned_as_announcement(false).with_updated_at(now)`,
        // but that path is broken: `Post::updater(...)` starts with a
        // default-initialised `inner` (status=Draft, visibility=None), and
        // every `.with_*` call recomputes the gsi5 sort key attribute
        // from that empty inner — overwriting the live row's gsi5 sk
        // with `"Draft##<now>"`. The row data stays correct but it
        // disappears from `find_by_user_and_status` ("Published"
        // begins-with query), so every subsequent fan-out caused the
        // PRIOR broadcast Post to vanish from the sub-team's feed.
        // We don't need explicit demotion — newer broadcasts naturally
        // sort above older ones in the feed by `created_at`.
        let create_tx = new_post.create_transact_write_item();
        let transacts = vec![create_tx];

        match cli
            .transact_write_items()
            .set_transact_items(Some(transacts))
            .send()
            .await
        {
            Ok(_) => {
                // For direct messages, persist the fan-out Post pk back
                // onto the announcement row so the parent's history can
                // build accurate `/posts/{pk}` links without rebuilding
                // them from announcement_id. Done BEFORE demote so a
                // mid-write crash leaves a consistent (newer) link.
                if announcement.target_child_team_id.is_some() {
                    if let Err(e) = persist_target_post_pk(
                        cli,
                        &announcement.pk,
                        &announcement.sk,
                        &new_post.pk,
                        now,
                    )
                    .await
                    {
                        crate::error!(
                            "[fanout] persist_target_post_pk failed: {e}"
                        );
                    }

                    // Demote every prior direct-message Post from this
                    // parent in this child's feed so only the latest
                    // stays pinned. Broadcasts intentionally keep all
                    // pinned (see comment above).
                    if let Err(e) = demote_prior_direct_posts(
                        cli,
                        &child_team_pk,
                        &parent_team_id,
                        &new_post.pk,
                        now,
                    )
                    .await
                    {
                        crate::error!(
                            "[fanout] demote_prior_direct_posts failed for child={}: {e}",
                            child_team_id,
                        );
                    }
                }
                fan_out_count += 1;
                new_post.created_at = now;
                new_post.updated_at = now;
                // Notify every member of the child team.
                let members = resolve_team_members(cli, &child_team_pk)
                    .await
                    .unwrap_or_default();
                let cta_url = build_post_detail_url(&new_post.pk);
                let post_id_display = match &new_post.pk {
                    Partition::Feed(id) => id.clone(),
                    other => other.to_string(),
                };
                for u in members {
                    let payload = InboxPayload::SubTeamAnnouncementReceived {
                        parent_team_id: parent_team_id.clone(),
                        parent_team_name: parent_team.display_name.clone(),
                        announcement_id: announcement.announcement_id.clone(),
                        title: announcement.title.clone(),
                        post_id: post_id_display.clone(),
                        post_pk: new_post.pk.to_string(),
                        cta_url: cta_url.clone(),
                    };
                    if let Err(e) =
                        crate::common::utils::inbox::create_inbox_row(u, payload).await
                    {
                        crate::error!(
                            "handle_announcement_published: inbox create failed: {e}"
                        );
                    }
                }
            }
            Err(e) => {
                crate::error!(
                    "handle_announcement_published: transact_write for child={} failed: {:?}",
                    child_team_id, e
                );
                // Continue — partial fan-out is acceptable per Phase 1 spec;
                // we still write the count that actually succeeded at the end.
            }
        }
    }

    // Update source announcement with fan_out_count. Non-fatal if this fails.
    if let Err(e) = SubTeamAnnouncement::updater(&announcement.pk, &announcement.sk)
        .with_fan_out_count(fan_out_count)
        .with_updated_at(now)
        .execute(cli)
        .await
    {
        crate::error!(
            "handle_announcement_published: fan_out_count update failed: {e}"
        );
    }

    Ok(())
}

fn build_post_detail_url(post_pk: &Partition) -> String {
    match post_pk {
        Partition::Feed(id) => format!("/posts/{id}"),
        other => format!("/posts/{other}"),
    }
}

/// Write the fan-out Post pk back onto the source `SubTeamAnnouncement`
/// row so the parent's history view can deep-link to the actual Post.
///
/// Uses the macro updater. `SubTeamAnnouncement` declares no GSI
/// composite keys, so neither `with_target_post_pk` nor `with_updated_at`
/// triggers GSI sort-key regeneration — the macro path is safe here.
async fn persist_target_post_pk(
    cli: &aws_sdk_dynamodb::Client,
    announcement_pk: &Partition,
    announcement_sk: &EntityType,
    post_pk: &Partition,
    now_ms: i64,
) -> Result<()> {
    SubTeamAnnouncement::updater(announcement_pk, announcement_sk.clone())
        .with_target_post_pk(post_pk.to_string())
        .with_updated_at(now_ms)
        .execute(cli)
        .await
        .map(|_| ())
        .map_err(|e| {
            crate::error!(
                "persist_target_post_pk: updater.execute failed pk={:?} sk={:?}: {:?}",
                announcement_pk,
                announcement_sk,
                e,
            );
            Error::from(SubTeamError::AnnouncementPublishFailed)
        })
}

/// Unpin every prior direct-message Post from `parent_team_id` in
/// `child_team_pk`'s feed except the new one.
///
/// Uses the macro updater with ONLY `with_pinned_as_announcement(false)`.
/// `pinned_as_announcement` is not part of any Post GSI composite key,
/// so its setter touches only that attribute. Crucially we do NOT call
/// `with_updated_at(..)` here — `updated_at` participates in `gsi5_sk`
/// (the "Published feed" index), and the macro re-derives that
/// composite key from a default-initialised inner struct
/// (`status: Draft`, `visibility: None`), corrupting the live row's
/// `gsi5_sk` to `"Draft##<now>"` and making it vanish from
/// `find_by_user_and_status` ("Published" begins-with). Leaving
/// `updated_at` stale on a demotion write is intentional — the post's
/// content didn't change, only its pinning flag.
async fn demote_prior_direct_posts(
    cli: &aws_sdk_dynamodb::Client,
    child_team_pk: &Partition,
    parent_team_id: &str,
    new_post_pk: &Partition,
    _now_ms: i64,
) -> Result<()> {
    // Walk the child's feed, bounded — direct-message posts are rare so
    // a few pages is plenty.
    let mut bookmark: Option<String> = None;
    let mut to_demote: Vec<Post> = Vec::new();
    for _ in 0..MAX_PAGES {
        let mut opt = Post::opt().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (posts, next) = Post::find_by_user_pk(cli, child_team_pk, opt).await?;
        for p in posts {
            if !p.pinned_as_announcement {
                continue;
            }
            if p.pk == *new_post_pk {
                continue;
            }
            // Only demote posts that came from THIS parent (covers both
            // broadcast and direct fan-outs — but in practice this only
            // fires for direct fan-outs since broadcasts call this with
            // `target_child_team_id.is_none()` and skip the demotion).
            if p.announcement_parent_team_id.as_deref() == Some(parent_team_id) {
                to_demote.push(p);
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    for p in to_demote {
        if let Err(e) = Post::updater(&p.pk, p.sk.clone())
            .with_pinned_as_announcement(false)
            .execute(cli)
            .await
        {
            crate::error!(
                "demote_prior_direct_posts: updater.execute failed pk={:?} sk={:?}: {:?}",
                p.pk,
                p.sk,
                e,
            );
        }
    }
    Ok(())
}
