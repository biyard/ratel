use crate::features::activity::models::SpaceScore;
use crate::features::activity::*;

#[cfg(feature = "server")]
pub async fn aggregate_score(
    activity: crate::features::activity::models::SpaceActivity,
) -> crate::common::Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let space_id = activity.pk.0.clone();
    let author = activity.pk.1.clone();
    let (score_pk, score_sk) = SpaceScore::keys(&space_id, &author);

    // Get existing score or create default
    let existing = SpaceScore::get(cli, &score_pk, Some(score_sk.clone()))
        .await?
        .unwrap_or_default();

    let new_total = existing.total_score + activity.total_score;

    use crate::features::spaces::pages::actions::types::SpaceActionType;
    let (poll, quiz, follow, discussion) = match activity.action_type {
        SpaceActionType::Poll => (
            existing.poll_score + activity.total_score,
            existing.quiz_score,
            existing.follow_score,
            existing.discussion_score,
        ),
        SpaceActionType::Quiz => (
            existing.poll_score,
            existing.quiz_score + activity.total_score,
            existing.follow_score,
            existing.discussion_score,
        ),
        SpaceActionType::Follow => (
            existing.poll_score,
            existing.quiz_score,
            existing.follow_score + activity.total_score,
            existing.discussion_score,
        ),
        SpaceActionType::TopicDiscussion => (
            existing.poll_score,
            existing.quiz_score,
            existing.follow_score,
            existing.discussion_score + activity.total_score,
        ),
        // Meet actions are not yet tracked in per-category score breakdowns.
        SpaceActionType::Meet => (
            existing.poll_score,
            existing.quiz_score,
            existing.follow_score,
            existing.discussion_score,
        ),
    };

    SpaceScore::updater(&score_pk, &score_sk)
        .with_total_score(new_total)
        .with_rank_total_score(new_total)
        .with_poll_score(poll)
        .with_quiz_score(quiz)
        .with_follow_score(follow)
        .with_discussion_score(discussion)
        .with_user_pk(activity.user_pk)
        .with_user_name(activity.user_name)
        .with_user_avatar(activity.user_avatar)
        .with_space_pk(activity.space_pk)
        .with_updated_at(now)
        .execute(cli)
        .await?;
    Ok(())
}
