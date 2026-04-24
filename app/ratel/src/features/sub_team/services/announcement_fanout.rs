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
/// When demoting the previous pinned announcement post in a child's feed, we
/// scan the most recent published posts by the child team rather than adding
/// a dedicated GSI. This cap bounds that scan.
const PRIOR_POST_SCAN_LIMIT: i32 = 20;

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

    let links = list_recognized_sub_teams(cli, &announcement.pk).await?;
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

    for link in &links {
        let child_team_id = link.child_team_id.clone();
        let child_team_pk: Partition = Partition::Team(child_team_id.clone());

        let mut new_post = Post {
            pk: Partition::Feed(uuid::Uuid::now_v7().to_string()),
            sk: EntityType::Post,
            created_at: now,
            updated_at: now,
            title: announcement.title.clone(),
            html_contents: announcement.body.clone(),
            post_type: PostType::Post,
            status: PostStatus::Published,
            visibility: Some(Visibility::Public),
            shares: 0,
            likes: 0,
            comments: 0,
            reports: 0,
            user_pk: child_team_pk.clone(),
            author_display_name: parent_team.display_name.clone(),
            author_profile_url: parent_team.profile_url.clone(),
            author_username: parent_team.username.clone(),
            author_type: crate::common::types::UserType::Team,
            space_pk: None,
            space_type: None,
            space_visibility: None,
            booster: None,
            rewards: None,
            urls: vec![],
            categories: vec![],
            announcement_id: Some(announcement.announcement_id.clone()),
            announcement_parent_team_id: Some(parent_team_id.clone()),
            pinned_as_announcement: true,
        };

        // Find the previous pinned announcement post from this same parent
        // team in the child's feed. Bounded scan of the child's most recent
        // posts (sk = created_at GSI) — Phase 1 cap of 20.
        let prior = find_prior_pinned_post(cli, &child_team_pk, &parent_team_id).await;

        // Write transactionally: new Post + demote prior (if any) + update
        // announcement's fan_out_count at the end once all succeed.
        let create_tx = new_post.create_transact_write_item();
        let mut transacts = vec![create_tx];
        if let Some(prior_post) = prior.as_ref() {
            let demote_tx = Post::updater(&prior_post.pk, &prior_post.sk)
                .with_pinned_as_announcement(false)
                .with_updated_at(now)
                .transact_write_item();
            transacts.push(demote_tx);
        }

        match cli
            .transact_write_items()
            .set_transact_items(Some(transacts))
            .send()
            .await
        {
            Ok(_) => {
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

/// Scan the child team's most recent posts and return the one pinned as an
/// announcement from the given parent team, if any. Bounded to
/// `PRIOR_POST_SCAN_LIMIT` — Phase 1 accepts missing very old pins rather
/// than paying for a dedicated GSI.
async fn find_prior_pinned_post(
    cli: &aws_sdk_dynamodb::Client,
    child_team_pk: &Partition,
    parent_team_id: &str,
) -> Option<Post> {
    let opt = Post::opt().limit(PRIOR_POST_SCAN_LIMIT);
    let (posts, _) = match Post::find_by_user_pk(cli, child_team_pk, opt).await {
        Ok(pair) => pair,
        Err(e) => {
            crate::error!("find_prior_pinned_post: query failed: {e}");
            return None;
        }
    };
    posts.into_iter().find(|p| {
        p.pinned_as_announcement
            && p.announcement_parent_team_id.as_deref() == Some(parent_team_id)
    })
}

fn build_post_detail_url(post_pk: &Partition) -> String {
    match post_pk {
        Partition::Feed(id) => format!("/posts/{id}"),
        other => format!("/posts/{other}"),
    }
}
