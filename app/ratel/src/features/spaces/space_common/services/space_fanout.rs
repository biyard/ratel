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

use crate::common::models::space::SpaceCommon;
use crate::common::types::*;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::common::*;
use crate::features::posts::models::Post;
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::pages::actions::types::SpaceActionType;
use crate::features::spaces::space_common::models::{HOT_SPACE_RANK_PK, HotSpace, HotSpaceHeat};

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
