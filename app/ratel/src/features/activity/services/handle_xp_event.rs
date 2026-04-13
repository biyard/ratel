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
    let author = crate::features::activity::types::AuthorPartition::from(user_pk);

    // Extract poll_id from sk: SPACE_POLL_USER_ANSWER#space_pk#poll_sk
    let poll_id = answer.sk.to_string();

    let user_name = answer.display_name.unwrap_or_default();
    let user_avatar = answer.profile_url.unwrap_or_default();

    crate::features::activity::controllers::record_activity(
        cli,
        space_partition,
        author,
        poll_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Poll,
        crate::features::activity::types::SpaceActivityData::Poll {
            poll_id,
            answered_optional_count: 0,
        },
        user_name,
        user_avatar,
    )
    .await
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
    let author = crate::features::activity::types::AuthorPartition::from(user_pk);

    let quiz_id = attempt.sk.to_string();
    let pass_threshold = attempt.pass_threshold.unwrap_or(0);
    let passed = attempt.score >= pass_threshold;

    let user_name = attempt.display_name.unwrap_or_default();
    let user_avatar = attempt.profile_url.unwrap_or_default();

    crate::features::activity::controllers::record_activity(
        cli,
        space_partition,
        author,
        quiz_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Quiz,
        crate::features::activity::types::SpaceActivityData::Quiz {
            quiz_id,
            passed,
            correct_count: attempt.score as u32,
            pass_threshold: pass_threshold as u32,
        },
        user_name,
        user_avatar,
    )
    .await
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

    let author =
        crate::features::activity::types::AuthorPartition::from(comment.author_pk.clone());

    // pk is the SpacePost partition (discussion reference), use as action_id
    let discussion_id = comment.pk.to_string();

    let is_first_contribution = comment.parent_comment_sk.is_none();

    crate::features::activity::controllers::record_activity(
        cli,
        space_partition,
        author,
        discussion_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
        crate::features::activity::types::SpaceActivityData::Discussion {
            discussion_id,
            is_first_contribution,
        },
        comment.author_display_name,
        comment.author_profile_url,
    )
    .await
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
    let author = crate::features::activity::types::AuthorPartition::from(follow.user_pk);

    let user_name = follow.display_name.unwrap_or_default();
    let user_avatar = follow.profile_url.unwrap_or_default();

    crate::features::activity::controllers::record_activity(
        cli,
        space_partition,
        author,
        action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Follow,
        crate::features::activity::types::SpaceActivityData::Follow {
            follow_id: action_id,
        },
        user_name,
        user_avatar,
    )
    .await
}
