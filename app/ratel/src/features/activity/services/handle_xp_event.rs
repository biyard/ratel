// Reward distribution on activity events.
//
// Each XP event handler below records a `SpaceActivity` for XP aggregation and
// then attempts to pay out the creator-configured reward via
// `SpaceReward::award_if_configured`. The helper is idempotent:
//
// - no `SpaceReward` row for the action  → silently skips (no reward set)
// - `credits == 0`                        → silently skips (rule: no credit, no payout)
// - first contribution per user / period → awards `credits × point` (= credits × 10_000)
// - subsequent contributions             → `UserRewardHistory` conditional-put fails,
//                                          mapped to `AlreadyClaimedInPeriod`, swallowed
//
// For discussions this means the user is rewarded on their first comment (or
// reply) in the discussion and never again, which matches the "reward only for
// the first reply" rule.

#[cfg(feature = "server")]
use crate::common::types::{Partition, RewardUserBehavior, SpacePartition};
#[cfg(feature = "server")]
use crate::features::spaces::space_common::models::space_reward::SpaceReward;

/// Fetch the owner (creator) of a space from `SpaceCommon` so we can pay the
/// 10% bonus. Returns `None` if the space cannot be found — the reward still
/// goes to the participant in that case.
#[cfg(feature = "server")]
async fn fetch_owner_pk(
    cli: &aws_sdk_dynamodb::Client,
    space_partition: &SpacePartition,
) -> Option<Partition> {
    use crate::common::models::space::SpaceCommon;
    use crate::common::types::EntityType;

    let space_pk: Partition = space_partition.clone().into();
    match SpaceCommon::get(cli, &space_pk, Some(&EntityType::SpaceCommon)).await {
        Ok(Some(space)) => Some(space.user_pk),
        Ok(None) => {
            tracing::warn!(space_pk = %space_pk, "SpaceCommon not found — owner bonus skipped");
            None
        }
        Err(e) => {
            tracing::warn!(space_pk = %space_pk, error = %e, "failed to load SpaceCommon — owner bonus skipped");
            None
        }
    }
}

/// Run reward distribution and log — never propagate so XP aggregation still
/// completes even if the reward leg fails.
#[cfg(feature = "server")]
async fn try_award(
    cli: &aws_sdk_dynamodb::Client,
    space_partition: SpacePartition,
    action_id: String,
    behavior: RewardUserBehavior,
    target_pk: Partition,
    owner_pk: Option<Partition>,
) {
    if let Err(e) = SpaceReward::award_if_configured(
        cli,
        space_partition.clone(),
        action_id.clone(),
        behavior.clone(),
        target_pk.clone(),
        owner_pk,
    )
    .await
    {
        tracing::error!(
            space_pk = %space_partition,
            action_id = %action_id,
            behavior = ?behavior,
            user_pk = %target_pk,
            error = %e,
            "reward payout failed on event bridge"
        );
    } else {
        tracing::info!(
            space_pk = %space_partition,
            action_id = %action_id,
            behavior = ?behavior,
            user_pk = %target_pk,
            "reward payout succeeded on event bridge"
        );
    }
}

#[cfg(feature = "server")]
pub async fn handle_poll_xp(
    answer: crate::features::spaces::pages::actions::actions::poll::SpacePollUserAnswer,
) -> crate::common::Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_id_str = answer.space_id.clone().unwrap_or_default();
    if space_id_str.is_empty() {
        tracing::warn!("PollXpRecord: missing space_id, skipping");
        return Ok(());
    }

    let user_pk = match &answer.user_pk {
        Some(pk) => pk.clone(),
        None => {
            tracing::warn!("PollXpRecord: missing user_pk, skipping");
            return Ok(());
        }
    };

    let space_partition = crate::common::types::SpacePartition(space_id_str);
    let author = crate::features::activity::types::AuthorPartition::from(user_pk.clone());

    // answer.sk = SpacePollUserAnswer(space_pk, poll_sk_str). Due to how the
    // two-field variant is deserialized from a `#`-joined string, `poll_sk`
    // may arrive as either:
    //   - "SPACE_POLL#<uuid>"                              (as originally intended)
    //   - "<space_id>#SPACE_POLL#<uuid>"                   (observed in prod)
    // `SpaceReward.action_id` uses just `<uuid>`, so take the segment after
    // the last `SPACE_POLL#` marker.
    let poll_sk_raw = match &answer.sk {
        crate::common::types::EntityType::SpacePollUserAnswer(_, poll_sk) => poll_sk.clone(),
        other => {
            tracing::warn!("PollXpRecord: unexpected sk shape: {other}");
            return Ok(());
        }
    };
    let poll_action_id = poll_sk_raw
        .rsplit_once("SPACE_POLL#")
        .map(|(_, id)| id.to_string())
        .unwrap_or_else(|| poll_sk_raw.clone());

    let user_name = answer.display_name.unwrap_or_default();
    let user_avatar = answer.profile_url.unwrap_or_default();

    crate::features::activity::controllers::record_activity(
        cli,
        space_partition.clone(),
        author,
        poll_action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Poll,
        crate::features::activity::types::SpaceActivityData::Poll {
            poll_id: poll_action_id.clone(),
            answered_optional_count: 0,
        },
        user_name,
        user_avatar,
    )
    .await?;

    let owner_pk = fetch_owner_pk(cli, &space_partition).await;
    try_award(
        cli,
        space_partition,
        poll_action_id,
        RewardUserBehavior::RespondPoll,
        user_pk,
        owner_pk,
    )
    .await;
    Ok(())
}

#[cfg(feature = "server")]
pub async fn handle_quiz_xp(
    attempt: crate::features::spaces::pages::actions::actions::quiz::SpaceQuizAttempt,
) -> crate::common::Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_id_str = attempt.space_id.clone().unwrap_or_default();
    if space_id_str.is_empty() {
        tracing::warn!("QuizXpRecord: missing space_id, skipping");
        return Ok(());
    }

    let user_pk = match &attempt.user_pk {
        Some(pk) => pk.clone(),
        None => {
            tracing::warn!("QuizXpRecord: missing user_pk, skipping");
            return Ok(());
        }
    };

    let space_partition = crate::common::types::SpacePartition(space_id_str);
    let author = crate::features::activity::types::AuthorPartition::from(user_pk.clone());

    // attempt.sk = SpaceQuizAttempt("SPACE_QUIZ#<quiz_uuid>#<attempt_uuid>");
    // SpaceReward.action_id for quiz is just <quiz_uuid>.
    let quiz_action_id = match &attempt.sk {
        crate::common::types::EntityType::SpaceQuizAttempt(raw) => {
            // Expected shape: "SPACE_QUIZ#<quiz>#<attempt>".
            let without_prefix = raw.strip_prefix("SPACE_QUIZ#").unwrap_or(raw);
            without_prefix
                .split_once('#')
                .map(|(quiz, _)| quiz.to_string())
                .unwrap_or_else(|| without_prefix.to_string())
        }
        other => {
            tracing::warn!("QuizXpRecord: unexpected sk shape: {other}");
            return Ok(());
        }
    };

    let pass_threshold = attempt.pass_threshold.unwrap_or(0);
    let passed = attempt.score >= pass_threshold;

    let user_name = attempt.display_name.unwrap_or_default();
    let user_avatar = attempt.profile_url.unwrap_or_default();

    crate::features::activity::controllers::record_activity(
        cli,
        space_partition.clone(),
        author,
        quiz_action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Quiz,
        crate::features::activity::types::SpaceActivityData::Quiz {
            quiz_id: attempt.sk.clone().into(),
            passed,
            correct_count: attempt.score as u32,
            pass_threshold: pass_threshold as u32,
        },
        user_name,
        user_avatar,
    )
    .await?;

    // Only pay the quiz reward on a passing attempt — mirrors the old inline
    // semantics in respond_quiz.
    if passed {
        let owner_pk = fetch_owner_pk(cli, &space_partition).await;
        try_award(
            cli,
            space_partition,
            quiz_action_id,
            RewardUserBehavior::QuizAnswer,
            user_pk,
            owner_pk,
        )
        .await;
    }
    Ok(())
}

#[cfg(feature = "server")]
pub async fn handle_discussion_xp(
    comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment,
) -> crate::common::Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_pk = match &comment.space_pk {
        Some(pk) => pk.clone(),
        None => {
            tracing::warn!("DiscussionXpRecord: missing space_pk, skipping");
            return Ok(());
        }
    };

    let space_id_str = match &space_pk {
        crate::common::types::Partition::Space(id) => id.clone(),
        _ => space_pk.to_string(),
    };
    let space_partition = crate::common::types::SpacePartition(space_id_str);

    let author = crate::features::activity::types::AuthorPartition::from(comment.author_pk.clone());

    // comment.pk = Partition::SpacePost(uuid) → "SPACE_POST#<uuid>".
    // SpaceReward.action_id (set by add_comment controller) uses
    // discussion_sk.to_string() which is a SubPartition — just the UUID.
    let discussion_action_id = match &comment.pk {
        crate::common::types::Partition::SpacePost(id) => id.clone(),
        other => other.to_string(),
    };

    let is_first_contribution = comment.parent_comment_sk.is_none();

    crate::features::activity::controllers::record_activity(
        cli,
        space_partition.clone(),
        author,
        discussion_action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
        crate::features::activity::types::SpaceActivityData::Discussion {
            discussion_id: comment.pk.clone().into(),
            comment_id: comment.sk.clone().into(),
            is_first_contribution,
        },
        comment.author_display_name,
        comment.author_profile_url,
    )
    .await?;

    // The `RewardPeriod::Once` gate on `UserRewardHistory` already enforces
    // "first reply per user per discussion" — let every comment try; only the
    // first one succeeds, subsequent ones no-op.
    let owner_pk = fetch_owner_pk(cli, &space_partition).await;
    try_award(
        cli,
        space_partition,
        discussion_action_id,
        RewardUserBehavior::DiscussionComment,
        comment.author_pk,
        owner_pk,
    )
    .await;
    Ok(())
}

#[cfg(feature = "server")]
pub async fn handle_follow_xp(
    follow: crate::common::models::auth::UserFollow,
) -> crate::common::Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_id_str = match &follow.space_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => {
            tracing::warn!("FollowXpRecord: missing space_id, skipping");
            return Ok(());
        }
    };

    let action_id = match &follow.action_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => {
            tracing::warn!("FollowXpRecord: missing action_id, skipping");
            return Ok(());
        }
    };

    let space_partition = crate::common::types::SpacePartition(space_id_str);
    let author = crate::features::activity::types::AuthorPartition::from(follow.user_pk.clone());

    let user_name = follow.display_name.unwrap_or_default();
    let user_avatar = follow.profile_url.unwrap_or_default();

    crate::features::activity::controllers::record_activity(
        cli,
        space_partition.clone(),
        author,
        action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Follow,
        crate::features::activity::types::SpaceActivityData::Follow {
            follow_id: action_id.clone(),
        },
        user_name,
        user_avatar,
    )
    .await?;

    let owner_pk = fetch_owner_pk(cli, &space_partition).await;
    try_award(
        cli,
        space_partition,
        action_id,
        RewardUserBehavior::Follow,
        follow.user_pk,
        owner_pk,
    )
    .await;
    Ok(())
}
