use std::collections::HashSet;

use crate::features::posts::models::Post;
use crate::features::timeline::*;
use crate::features::timeline::models::TimelineReason;

use super::common::{collect_followers_with_reason, write_timeline_entries};

/// Fan out a published post to all relevant timelines.
///
/// Strategies applied:
/// 1. **Follower fan-out** -- delivers to all followers of the author + the author themselves
/// 2. **Team member fan-out** -- if the author is a Team, delivers to all team members
/// 3. Deduplicates across strategies so each user gets at most one entry
pub async fn fan_out_timeline_entries(post: Post) -> Result<()> {
    tracing::info!(
        "Timeline update: post_pk={}, author_pk={}, created_at={}",
        post.pk,
        post.user_pk,
        post.created_at
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let post_pk = &post.pk;
    let author_pk = &post.user_pk;
    let created_at = post.created_at;
    let mut seen = HashSet::new();
    let mut entries: Vec<(Partition, TimelineReason)> = Vec::new();

    // 1. Author themselves
    seen.insert(author_pk.to_string());
    entries.push((author_pk.clone(), TimelineReason::Following));

    // 2. All followers of the author
    collect_followers(cli, author_pk, &mut seen, &mut entries).await?;

    // 3. If the author is a Team, all team members
    if matches!(author_pk, Partition::Team(_)) {
        collect_team_members(cli, author_pk, &mut seen, &mut entries).await?;
    }

    tracing::info!(
        "fan_out_timeline_entries: writing {} timeline entries for post {}",
        entries.len(),
        post_pk
    );

    write_timeline_entries(cli, post_pk, author_pk, created_at, &entries).await
}

/// Collect all follower user_pks of `target_pk`, adding them to `entries` with reason Following.
async fn collect_followers(
    cli: &aws_sdk_dynamodb::Client,
    target_pk: &Partition,
    seen: &mut HashSet<String>,
    entries: &mut Vec<(Partition, TimelineReason)>,
) -> Result<()> {
    collect_followers_with_reason(cli, target_pk, seen, entries, TimelineReason::Following).await
}

/// Collect all team member user_pks via UserTeamGroup GSI2.
async fn collect_team_members(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    seen: &mut HashSet<String>,
    entries: &mut Vec<(Partition, TimelineReason)>,
) -> Result<()> {
    let mut bookmark: Option<String> = None;
    loop {
        let mut opt = crate::features::auth::UserTeamGroup::opt().limit(100);
        if let Some(bk) = bookmark {
            opt = opt.bookmark(bk);
        }

        let (members, next_bookmark) =
            crate::features::auth::UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt)
                .await?;

        for member in members {
            let key = member.pk.to_string();
            if seen.insert(key) {
                entries.push((member.pk, TimelineReason::TeamMember));
            }
        }

        bookmark = next_bookmark;
        if bookmark.is_none() {
            break;
        }
    }
    Ok(())
}
