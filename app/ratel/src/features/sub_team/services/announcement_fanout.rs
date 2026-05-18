use std::collections::HashSet;

use crate::common::*;
use crate::features::auth::UserTeam;
use crate::features::posts::models::{Team, TeamOwner};
use crate::features::sub_team::models::{
    SubTeamAnnouncement, SubTeamAnnouncementFanout, SubTeamAnnouncementStatus, SubTeamLink,
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
    let opt = SubTeamLink::opt()
        .sk("SUB_TEAM_LINK".to_string())
        .limit((MAX_RECOGNIZED_SUB_TEAMS as i32) + 1);
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
/// **New model (no Post cloning):** the parent's anchor Post — created
/// synchronously in `publish_announcement_handler` — is the **only**
/// Post row for this announcement. Likes, comments, shares, and the
/// canonical `/posts/{announcement_id}` URL all live on that single row.
///
/// This fan-out writes one lightweight `SubTeamAnnouncementFanout`
/// marker per recognized sub-team (or just the single targeted child,
/// for direct messages). The marker points back at the anchor Post by
/// pk. The child's wall query (`list_team_posts_handler`) reads these
/// markers, batch-gets the anchor Posts they point at, and unions the
/// result with the child's own Posts.
///
/// Members of each sub-team still receive an inbox notification.
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

    // Load parent team once so the inbox notifications can quote the
    // parent's display name without re-fetching per child.
    let parent_team = match Team::get(cli, &announcement.pk, Some(EntityType::Team)).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            crate::error!("handle_announcement_published: parent team not found: {parent_team_id}");
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
    let is_direct = announcement.target_child_team_id.is_some();
    let mut fan_out_count: i32 = 0;
    let anchor_post_pk = Partition::Feed(announcement.announcement_id.clone());
    let cta_url = build_post_detail_url(&anchor_post_pk);
    let post_id_display = announcement.announcement_id.clone();
    let defer_inbox = !is_direct && announcement.space_enabled && announcement.space_pk.is_some();

    for link in &links {
        let child_team_id = link.child_team_id.clone();
        let child_team_pk: Partition = Partition::Team(child_team_id.clone());

        // Broadcasts: write a marker so the wall query can union the
        // anchor Post into the child's feed. Direct messages bypass the
        // marker entirely — their anchor Post is written with
        // `user_pk = target_child_team_pk` at send-time, so it shows up
        // in the child's `Post::find_by_user_and_status` query directly
        // (and stays off the parent's wall).
        if !is_direct {
            let marker = SubTeamAnnouncementFanout::new(
                child_team_pk.clone(),
                announcement.announcement_id.clone(),
                parent_team_id.clone(),
                false,
                now,
            );
            if let Err(e) = marker.create(cli).await {
                crate::error!(
                    "handle_announcement_published: marker create for child={} failed: {e}",
                    child_team_id
                );
                // Partial fan-out is acceptable per Phase 1 spec — keep
                // going so other children still get their inbox.
                continue;
            }
            fan_out_count += 1;
        } else {
            // Direct messages still count toward fan_out_count so the
            // idempotency guard at the top of this function holds.
            fan_out_count += 1;
        }

        if defer_inbox {
            continue;
        }

        // Notify every member of the child team.
        let members = resolve_team_members(cli, &child_team_pk)
            .await
            .unwrap_or_default();
        for u in members {
            let payload = InboxPayload::SubTeamAnnouncementReceived {
                parent_team_id: parent_team_id.clone(),
                parent_team_name: parent_team.display_name.clone(),
                announcement_id: announcement.announcement_id.clone(),
                title: announcement.title.clone(),
                post_id: post_id_display.clone(),
                post_pk: anchor_post_pk.to_string(),
                cta_url: cta_url.clone(),
            };
            if let Err(e) = crate::common::utils::inbox::create_inbox_row(u, payload).await {
                crate::error!("handle_announcement_published: inbox create failed: {e}");
            }
        }
    }

    let mut updater = SubTeamAnnouncement::updater(&announcement.pk, &announcement.sk)
        .with_fan_out_count(fan_out_count)
        .with_updated_at(now);
    if is_direct {
        updater = updater.with_target_post_pk(anchor_post_pk.to_string());
    }
    if let Err(e) = updater.execute(cli).await {
        crate::error!("handle_announcement_published: fan_out_count update failed: {e}");
    }

    Ok(())
}

fn build_post_detail_url(post_pk: &Partition) -> String {
    match post_pk {
        Partition::Feed(id) => format!("/posts/{id}"),
        other => format!("/posts/{other}"),
    }
}

#[cfg(feature = "server")]
pub async fn handle_space_published(
    cli: &aws_sdk_dynamodb::Client,
    space: crate::common::models::space::SpaceCommon,
) -> Result<()> {
    use crate::common::types::InboxPayload;
    use crate::features::posts::models::Post;
    use crate::features::posts::types::PostType;

    // Broadcast-attached Space has `space.pk = SPACE#<id>` and
    // `space.post_pk = FEED#<id>` — the anchor Post lives on the Feed
    // partition, NOT the Space partition. Using `space.pk` here would
    // always lookup-miss and silently drop every notification.
    let post_pk = space.post_pk.clone();
    let post = match Post::get(cli, &post_pk, Some(EntityType::Post)).await {
        Ok(Some(p)) => p,
        Ok(None) => return Ok(()),
        Err(e) => {
            crate::error!("handle_space_published: anchor post lookup failed: {e}");
            return Ok(());
        }
    };
    if !matches!(post.post_type, PostType::Post) {
        return Ok(());
    }
    let (Some(announcement_id), Some(parent_team_id)) = (
        post.announcement_id.clone(),
        post.announcement_parent_team_id.clone(),
    ) else {
        return Ok(());
    };

    let ann_pk = Partition::Team(parent_team_id.clone());
    let ann_sk = EntityType::SubTeamAnnouncement(announcement_id.clone());
    let announcement = match SubTeamAnnouncement::get(cli, &ann_pk, Some(ann_sk.clone())).await {
        Ok(Some(a)) => a,
        Ok(None) => return Ok(()),
        Err(e) => {
            crate::error!("handle_space_published: announcement lookup failed: {e}");
            return Ok(());
        }
    };
    if announcement.target_child_team_id.is_some() {
        return Ok(());
    }
    if announcement.broadcast_notified_at.is_some() {
        return Ok(());
    }

    let parent_team = match Team::get(cli, &announcement.pk, Some(EntityType::Team)).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            crate::error!("handle_space_published: parent team not found: {parent_team_id}");
            return Ok(());
        }
        Err(e) => {
            crate::error!("handle_space_published: parent team fetch: {e}");
            return Ok(());
        }
    };

    let links = list_recognized_sub_teams(cli, &announcement.pk).await?;
    let anchor_post_pk = Partition::Feed(announcement.announcement_id.clone());
    // Space URL takes the raw id (no `SPACE#`/`FEED#` prefix), which
    // matches the canonical `/spaces/:space_id` route in `route.rs`.
    // Derive from `announcement_id` since both `space.pk` and
    // `space.post_pk` carry the same id.
    let cta_url = format!("/spaces/{}", announcement.announcement_id);

    for link in &links {
        let child_team_pk = Partition::Team(link.child_team_id.clone());
        let members = resolve_team_members(cli, &child_team_pk)
            .await
            .unwrap_or_default();
        for u in members {
            let payload = InboxPayload::SubTeamAnnouncementReceived {
                parent_team_id: parent_team_id.clone(),
                parent_team_name: parent_team.display_name.clone(),
                announcement_id: announcement.announcement_id.clone(),
                title: announcement.title.clone(),
                post_id: announcement.announcement_id.clone(),
                post_pk: anchor_post_pk.to_string(),
                cta_url: cta_url.clone(),
            };
            if let Err(e) = crate::common::utils::inbox::create_inbox_row(u, payload).await {
                crate::error!("handle_space_published: inbox create failed: {e}");
            }
        }
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    if let Err(e) = SubTeamAnnouncement::updater(&announcement.pk, &ann_sk)
        .with_broadcast_notified_at(now)
        .with_updated_at(now)
        .execute(cli)
        .await
    {
        crate::error!("handle_space_published: broadcast_notified_at write failed: {e}");
    }

    Ok(())
}

