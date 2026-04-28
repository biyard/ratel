//! HotSpace fanout. The only writer of `HotSpace` rows.
//!
//! Triggered after a Space mutation (publish, content edit, status change,
//! quota change, etc.). Re-reads the current SpaceCommon + Post + action
//! counts, recomputes a `WindowedRankKey` anchored at "now", and upserts the
//! row. Public+Published spaces produce a row; everything else has the row
//! deleted so it disappears from `list_hot_spaces` results.
//!
//! Best-effort: errors must not fail the user-facing mutation that triggered
//! us. Callers should ignore errors and rely on logging.

use std::collections::HashSet;

use crate::common::models::auth::UserFollow;
use crate::common::models::space::SpaceCommon;
use crate::common::types::*;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::common::*;
use crate::features::posts::models::Post;
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::pages::actions::types::SpaceActionType;
use crate::features::spaces::space_common::models::{
    HOT_SPACE_RANK_PK, HotSpace, HotSpaceHeat, UserHotSpace, user_hot_space_rank_pk,
};

/// 30-day windows. Newer windows always lex-sort above older windows, so
/// stale rows that never get re-fanout naturally fall out of the top.
const WINDOW_DAYS: u64 = 30;
const WINDOW_SIZE_SECS: u64 = WINDOW_DAYS * 24 * 60 * 60;

/// Hour resolution gives 720 freshness units per window. With multipliers
/// below, a single new participant offsets ~6 hours of decay.
const FRESHNESS_RESOLUTION: TimeBucket = TimeBucket::Hour;
const PARTICIPANT_MULTIPLIER: u64 = 6;
const ACTION_MULTIPLIER: u64 = 2;

/// Re-snapshot a space's hot ranking. Idempotent; safe to call repeatedly.
///
/// - Public+Published spaces: upsert the HotSpace row.
/// - Everything else: delete the HotSpace row (best-effort; missing-row is OK).
pub async fn upsert_hot_space(cli: &aws_sdk_dynamodb::Client, space_pk: &Partition) {
    let space = match SpaceCommon::get(cli, space_pk.clone(), Some(EntityType::SpaceCommon)).await
    {
        Ok(Some(s)) => s,
        Ok(None) => {
            // Space deleted — drop any stale row.
            let _ = HotSpace::delete(cli, space_pk.clone(), Some(EntityType::HotSpace)).await;
            return;
        }
        Err(e) => {
            crate::error!(
                space_pk = %space_pk,
                error = %e,
                "fanout: failed to load SpaceCommon"
            );
            return;
        }
    };

    if !space.is_public() {
        if let Err(e) = HotSpace::delete(cli, space_pk.clone(), Some(EntityType::HotSpace)).await {
            crate::warn!(
                space_pk = %space_pk,
                error = %e,
                "fanout: HotSpace delete failed (likely missing — ignored)"
            );
        }
        // Per-viewer rows are best-effort cleaned up only via the read path's
        // backstop check (see list_hot_spaces). They self-heal on the next
        // upsert; deleting all of them here would require a full per-viewer
        // scan which is too expensive for a private/draft transition.
        return;
    }

    let title = match space.pk.clone().to_post_key() {
        Ok(post_pk) => Post::get(cli, post_pk, Some(EntityType::Post))
            .await
            .ok()
            .flatten()
            .map(|p| p.title)
            .unwrap_or_default(),
        Err(_) => String::new(),
    };

    let post_pk = space.pk.clone().to_post_key().unwrap_or_default();
    let description = if !space.content.is_empty() {
        extract_description(&space.content)
    } else {
        String::new()
    };

    let (poll_count, discussion_count, quiz_count, follow_count) =
        count_actions(cli, &space.pk).await;
    let total_actions = poll_count + discussion_count + quiz_count + follow_count;

    let now_ms = get_now_timestamp_millis();
    let now_secs = (now_ms / 1000).max(0) as u64;

    let participants_u64 = space.participants.max(0) as u64;
    let actions_u64 = total_actions.max(0) as u64;

    let rank_key = WindowedRankKey::build(
        FRESHNESS_RESOLUTION,
        WINDOW_SIZE_SECS,
        now_secs,
        &[
            Factor::new(participants_u64, PARTICIPANT_MULTIPLIER),
            Factor::new(actions_u64, ACTION_MULTIPLIER),
        ],
    );

    let row = HotSpace {
        pk: space.pk.clone(),
        sk: EntityType::HotSpace,
        created_at: now_ms,
        updated_at: now_ms,
        rank_pk: HOT_SPACE_RANK_PK.to_string(),
        rank_key,
        post_pk,
        title,
        description,
        logo: space.logo.clone(),
        author_display_name: space.author_display_name.clone(),
        participants: space.participants,
        rewards: space.rewards.unwrap_or(0),
        poll_count,
        discussion_count,
        quiz_count,
        follow_count,
        total_actions,
        heat: HotSpaceHeat::from_participants(space.participants),
        space_created_at: space.created_at,
    };

    if let Err(e) = row.upsert(cli).await {
        crate::error!(
            space_pk = %space_pk,
            error = %e,
            "fanout: HotSpace upsert failed"
        );
    }

    // Per-viewer fanout. Each viewer reachable from the author (the author
    // themself + their followers + team members when the author is a Team)
    // gets a `UserHotSpace` row mirroring the global snapshot. Logged-in
    // reads scan by viewer id so a user only sees the spaces they've been
    // fanned out to.
    let viewers = collect_viewers(cli, &space.user_pk).await;
    fan_out_user_hot_spaces(cli, &space, &row, &viewers).await;
}

/// Collect every viewer that should see the space in their personal Hot
/// stream: author + followers (if Public) + team members (Team-authored).
async fn collect_viewers(cli: &aws_sdk_dynamodb::Client, author_pk: &Partition) -> Vec<Partition> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut viewers: Vec<Partition> = Vec::new();

    // Author themself always sees their own published public spaces.
    if seen.insert(author_pk.to_string()) {
        viewers.push(author_pk.clone());
    }

    // Followers of the author. Best-effort — partial collection is fine,
    // missing followers self-heal on the next upsert.
    if let Err(e) = collect_followers(cli, author_pk, &mut seen, &mut viewers).await {
        crate::warn!(
            author_pk = %author_pk,
            error = %e,
            "fanout: follower collection failed (partial fanout)"
        );
    }

    // Team members fan-out only when the author is a Team. Same best-effort
    // policy — partial fanout self-heals on subsequent upserts.
    if matches!(author_pk, Partition::Team(_)) {
        if let Err(e) = collect_team_members(cli, author_pk, &mut seen, &mut viewers).await {
            crate::warn!(
                team_pk = %author_pk,
                error = %e,
                "fanout: team member collection failed (partial fanout)"
            );
        }
    }

    viewers
}

async fn collect_followers(
    cli: &aws_sdk_dynamodb::Client,
    target_pk: &Partition,
    seen: &mut HashSet<String>,
    viewers: &mut Vec<Partition>,
) -> Result<()> {
    let mut bookmark: Option<String> = None;
    loop {
        let mut opt = UserFollow::opt()
            .sk(EntityType::Follower(String::default()).to_string())
            .limit(100);
        if let Some(bk) = bookmark {
            opt = opt.bookmark(bk);
        }
        let (follows, next_bookmark) = UserFollow::query(cli, target_pk.clone(), opt).await?;
        for follow in follows {
            if seen.insert(follow.user_pk.to_string()) {
                viewers.push(follow.user_pk);
            }
        }
        bookmark = next_bookmark;
        if bookmark.is_none() {
            break;
        }
    }
    Ok(())
}

async fn collect_team_members(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    seen: &mut HashSet<String>,
    viewers: &mut Vec<Partition>,
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
            if seen.insert(member.pk.to_string()) {
                viewers.push(member.pk);
            }
        }
        bookmark = next_bookmark;
        if bookmark.is_none() {
            break;
        }
    }
    Ok(())
}

/// Mirror the global snapshot into each viewer's per-viewer hot stream.
async fn fan_out_user_hot_spaces(
    cli: &aws_sdk_dynamodb::Client,
    space: &SpaceCommon,
    snapshot: &HotSpace,
    viewers: &[Partition],
) {
    for viewer_pk in viewers {
        let viewer_id = match viewer_pk {
            Partition::User(id) => id.clone(),
            Partition::Team(id) => id.clone(),
            other => other.to_string(),
        };
        let space_id = match &space.pk {
            Partition::Space(id) => id.clone(),
            other => other.to_string(),
        };

        let row = UserHotSpace {
            pk: viewer_pk.clone(),
            sk: EntityType::UserHotSpace(space_id),
            created_at: snapshot.created_at,
            updated_at: snapshot.updated_at,
            rank_pk: user_hot_space_rank_pk(&viewer_id),
            rank_key: snapshot.rank_key.clone(),
            space_pk: space.pk.clone(),
            post_pk: snapshot.post_pk.clone(),
            title: snapshot.title.clone(),
            description: snapshot.description.clone(),
            logo: snapshot.logo.clone(),
            author_display_name: snapshot.author_display_name.clone(),
            participants: snapshot.participants,
            rewards: snapshot.rewards,
            poll_count: snapshot.poll_count,
            discussion_count: snapshot.discussion_count,
            quiz_count: snapshot.quiz_count,
            follow_count: snapshot.follow_count,
            total_actions: snapshot.total_actions,
            heat: snapshot.heat,
            space_created_at: snapshot.space_created_at,
        };

        if let Err(e) = row.upsert(cli).await {
            crate::warn!(
                viewer_pk = %viewer_pk,
                space_pk = %space.pk,
                error = %e,
                "fanout: UserHotSpace upsert failed (continuing other viewers)"
            );
        }
    }
}

async fn count_actions(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> (i64, i64, i64, i64) {
    let opts = SpaceAction::opt_all();
    let actions = match SpaceAction::find_by_space(cli, space_pk.clone(), opts).await {
        Ok((actions, _)) => actions,
        Err(_) => return (0, 0, 0, 0),
    };

    let mut polls = 0i64;
    let mut discussions = 0i64;
    let mut quizzes = 0i64;
    let mut follows = 0i64;
    for a in actions {
        match a.space_action_type {
            SpaceActionType::Poll => polls += 1,
            SpaceActionType::TopicDiscussion => discussions += 1,
            SpaceActionType::Quiz => quizzes += 1,
            SpaceActionType::Follow => follows += 1,
            SpaceActionType::Meet => {}
        }
    }
    (polls, discussions, quizzes, follows)
}

fn extract_description(html: &str) -> String {
    let re_img = regex::Regex::new(r"<img[^>]*>").unwrap();
    let without_images = re_img.replace_all(html, "");

    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    let without_tags = re_tags.replace_all(&without_images, "");

    let re_urls = regex::Regex::new(r"https?://[^\s]+").unwrap();
    let without_urls = re_urls.replace_all(&without_tags, "");

    let re_whitespace = regex::Regex::new(r"\s+").unwrap();
    re_whitespace
        .replace_all(&without_urls, " ")
        .trim()
        .to_string()
}
