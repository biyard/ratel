/// Shared DynamoDB Stream record handler used by both the Lambda EventBridge
/// path and the local-dev stream poller.
///
/// Processes a single DynamoDB stream record by matching the event name and
/// sort key prefix against known patterns (mirroring the CDK EventBridge Pipes
/// filter criteria), then dispatching to the appropriate handler.
#[cfg(feature = "server")]
pub async fn handle_stream_record(
    event_name: &str,
    new_image: Option<&std::collections::HashMap<String, serde_dynamo::AttributeValue>>,
    old_image: Option<&std::collections::HashMap<String, serde_dynamo::AttributeValue>>,
) -> crate::common::Result<()> {
    use crate::common::Error;
    use crate::common::utils::InfraError;

    // Helper to get sk string from a DynamoDB image
    fn get_sk(
        image: &std::collections::HashMap<String, serde_dynamo::AttributeValue>,
    ) -> Option<String> {
        image.get("sk").and_then(|v| {
            if let serde_dynamo::AttributeValue::S(s) = v {
                Some(s.clone())
            } else {
                None
            }
        })
    }

    fn get_string_field(
        image: &std::collections::HashMap<String, serde_dynamo::AttributeValue>,
        key: &str,
    ) -> Option<String> {
        image.get(key).and_then(|v| {
            if let serde_dynamo::AttributeValue::S(s) = v {
                Some(s.clone())
            } else {
                None
            }
        })
    }

    fn deserialize<T: serde::de::DeserializeOwned>(
        image: &std::collections::HashMap<String, serde_dynamo::AttributeValue>,
    ) -> crate::common::Result<T> {
        serde_dynamo::from_item(image.clone())
            .map_err(|e| {
                tracing::error!("stream deserialize: {e}");
                Error::from(InfraError::StreamDeserializeFailed)
            })
    }

    // Dispatch essence indexing in addition to the entity-specific handlers
    // below. Runs unconditionally so a Post (existing PostVectorIndex branch)
    // also gets mirrored into the Essence list.
    if matches!(event_name, "INSERT" | "MODIFY") {
        if let Some(image) = new_image {
            let sk = get_sk(image).unwrap_or_default();
            if let Err(e) = essence_index_dispatch(&sk, image).await {
                tracing::error!(error = %e, sk = %sk, "stream: essence index dispatch failed");
            }
        }
    } else if event_name == "REMOVE" {
        if let Some(image) = old_image {
            let sk = get_sk(image).unwrap_or_default();
            if let Err(e) = essence_remove_dispatch(&sk, image).await {
                tracing::error!(error = %e, sk = %sk, "stream: essence remove dispatch failed");
            }
        }
    }

    match event_name {
        "INSERT" => {
            let image = new_image.ok_or(Error::from(InfraError::StreamMissingImage))?;
            let sk = get_sk(image).unwrap_or_default();

            if sk.starts_with("SPACE_POST_COMMENT#") {
                // AiModeratorReplyIndex: index comment into Qdrant
                let comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment = deserialize(image)?;
                if let Err(e) =
                    crate::features::rag::qdrant::indexers::reply_indexer::index_reply(comment.clone()).await
                {
                    tracing::error!(error = %e, "stream: AiModeratorReplyIndex failed");
                }
                // DiscussionXpRecord: record XP for discussion comment
                if let Err(e) =
                    crate::features::activity::services::handle_discussion_xp(comment).await
                {
                    tracing::error!(error = %e, "stream: DiscussionXpRecord failed");
                }
            } else if sk.starts_with("SPACE_POST_COMMENT_REPLY#") {
                // DiscussionXpRecord: record XP for discussion reply
                let comment = deserialize(image)?;
                if let Err(e) =
                    crate::features::activity::services::handle_discussion_xp(comment).await
                {
                    tracing::error!(error = %e, "stream: DiscussionXpRecord (reply) failed");
                }
            } else if sk == "POST" || sk.starts_with("POST") {
                // PostVectorIndex for newly inserted published posts
                if get_string_field(image, "status").as_deref() == Some("PUBLISHED") {
                    let post = deserialize(image)?;
                    if let Err(e) =
                        crate::features::rag::qdrant::indexers::post_indexer::index_post(post).await
                    {
                        tracing::error!(error = %e, "stream: PostVectorIndex (INSERT) failed");
                    }
                }
            } else if sk.starts_with("NOTIFICATION#") {
                let notification: crate::common::models::notification::Notification =
                    deserialize(image)?;
                if let Err(e) = notification.process().await {
                    tracing::error!(error = %e, "stream: NotificationSend failed");
                }
            } else if sk.starts_with("SPACE_ACTIVITY#") {
                {
                    let activity = deserialize(image)?;
                    if let Err(e) =
                        crate::features::activity::services::aggregate_score(activity).await
                    {
                        tracing::error!(error = %e, "stream: ActivityScoreAggregate failed");
                    }
                }
            } else if sk.starts_with("SPACE_STATUS_CHANGE_EVENT#") {
                let event: crate::common::models::space::SpaceStatusChangeEvent =
                    deserialize(image)?;
                if let Err(e) =
                    crate::features::spaces::space_common::services::handle_space_status_change(
                        event,
                    )
                    .await
                {
                    tracing::error!(error = %e, "stream: SpaceStatusChangeEvent failed");
                }
            } else if sk.starts_with("SPACE_POLL_USER_ANSWER#") {
                // PollXpRecord: record XP for poll answer
                let answer = deserialize(image)?;
                if let Err(e) =
                    crate::features::activity::services::handle_poll_xp(answer).await
                {
                    tracing::error!(error = %e, "stream: PollXpRecord failed");
                }
            } else if sk.starts_with("SPACE_QUIZ_ATTEMPT#") {
                // QuizXpRecord: record XP for quiz attempt
                let attempt = deserialize(image)?;
                if let Err(e) =
                    crate::features::activity::services::handle_quiz_xp(attempt).await
                {
                    tracing::error!(error = %e, "stream: QuizXpRecord failed");
                }
            } else if sk.starts_with("FOLLOWER#") {
                // FollowXpRecord: record XP for follow action
                let follow = deserialize(image)?;
                if let Err(e) =
                    crate::features::activity::services::handle_follow_xp(follow).await
                {
                    tracing::error!(error = %e, "stream: FollowXpRecord failed");
                }
            } else if sk.starts_with("SPACE_ANALYZE_REPORT#") {
                // AnalyzeReportInProgress: kick off auto poll/quiz/follow
                // analysis. Filter on INSERT only — Lambda's own status
                // flip would otherwise re-trigger this branch via MODIFY.
                let report = deserialize(image)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                if let Err(e) =
                    crate::features::spaces::pages::apps::apps::analyzes::services::auto_analysis::process_analyze_report(cli, &report)
                        .await
                {
                    tracing::error!(error = %e, "stream: AnalyzeReportInProgress failed");
                }
            } else if sk.starts_with("SPACE_ANALYZE_DISCUSSION_RESULT#") {
                // AnalyzeDiscussionInProgress: discussion text analysis
                // pipeline. INSERT-only filter on the pipe keeps the
                // updater's MODIFY from re-triggering the handler.
                let row = deserialize(image)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                if let Err(e) =
                    crate::features::spaces::pages::apps::apps::analyzes::services::discussion_analysis::process_discussion_analysis(cli, &row)
                        .await
                {
                    tracing::error!(error = %e, "stream: AnalyzeDiscussionInProgress failed");
                }
            }
        }
        "MODIFY" => {
            let image = new_image.ok_or(Error::from(InfraError::StreamMissingImage))?;
            let sk = get_sk(image).unwrap_or_default();

            if sk.starts_with("SPACE_POST#") {
                // AiModeratorReplyCheck: check if moderation should trigger
                let post = deserialize(image)?;
                if let Err(e) =
                    crate::features::ai_moderator::services::event_handler::handle_ai_moderator_event(post).await
                {
                    tracing::error!(error = %e, "stream: AiModeratorReplyCheck failed");
                }
            } else if sk == "POST" || sk.starts_with("POST") {
                let status = get_string_field(image, "status").unwrap_or_default();
                if status == "PUBLISHED" {
                    // PostVectorIndex
                    let post = deserialize(image)?;
                    if let Err(e) =
                        crate::features::rag::qdrant::indexers::post_indexer::index_post(post).await
                    {
                        tracing::error!(error = %e, "stream: PostVectorIndex failed");
                    }

                    // TimelineUpdate
                    let post2 = deserialize(image)?;
                    if let Err(e) =
                        crate::features::timeline::services::fan_out_timeline_entries(post2).await
                    {
                        tracing::error!(error = %e, "stream: TimelineUpdate failed");
                    }
                }
            } else if sk == "SPACE_COMMON" {
                // PopularSpaceUpdate
                let space = deserialize(image)?;
                if let Err(e) =
                    crate::features::timeline::services::fan_out_popular_space(space).await
                {
                    tracing::error!(error = %e, "stream: PopularSpaceUpdate failed");
                }
            } else if sk.starts_with("SUB_TEAM_ANNOUNCEMENT#") {
                // Fan-out fires only on the Draft→Published transition. We
                // don't have oldImage here (local-dev stream poller is
                // NewImage-only in the branches above), so we match on the
                // new status directly — handle_announcement_published is a
                // no-op for non-Published rows.
                let status = get_string_field(image, "status").unwrap_or_default();
                if status == "Published" {
                    let announcement: crate::features::sub_team::models::SubTeamAnnouncement =
                        deserialize(image)?;
                    let cfg = crate::common::CommonConfig::default();
                    let cli = cfg.dynamodb();
                    if let Err(e) =
                        crate::features::sub_team::services::announcement_fanout::handle_announcement_published(
                            cli,
                            announcement,
                        )
                        .await
                    {
                        tracing::error!(error = %e, "stream: SubTeamAnnouncementPublished failed");
                    }
                }
            }
        }
        "REMOVE" => {
            let image = old_image.ok_or(Error::from(InfraError::StreamMissingImage))?;
            let sk = get_sk(image).unwrap_or_default();

            if sk == "POST" || sk.starts_with("POST") {
                if get_string_field(image, "status").as_deref() == Some("PUBLISHED") {
                    let post = deserialize(image)?;
                    if let Err(e) =
                        crate::features::rag::qdrant::indexers::post_indexer::delete_post_index(
                            post,
                        )
                        .await
                    {
                        tracing::error!(error = %e, "stream: PostVectorDelete failed");
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}

/// Mirror an INSERT/MODIFY of any essence-indexable entity into the user's
/// Essence list. Each branch deserializes the image into the matching
/// model and delegates to the shared `essence::services` indexer (the same
/// helper the migrate endpoint uses), so behaviour stays identical between
/// stream-driven indexing and explicit backfills.
#[cfg(feature = "server")]
async fn essence_index_dispatch(
    sk: &str,
    image: &std::collections::HashMap<String, serde_dynamo::AttributeValue>,
) -> crate::common::Result<()> {
    use crate::common::Error;
    use crate::common::utils::InfraError;

    fn deserialize<T: serde::de::DeserializeOwned>(
        image: &std::collections::HashMap<String, serde_dynamo::AttributeValue>,
    ) -> crate::common::Result<T> {
        serde_dynamo::from_item(image.clone()).map_err(|e| {
            tracing::error!("essence dispatch deserialize: {e}");
            Error::from(InfraError::StreamDeserializeFailed)
        })
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    if sk == "POST" {
        let post: crate::features::posts::models::Post = deserialize(image)?;
        crate::features::essence::services::index_post(cli, &post).await?;
    } else if sk == "SPACE_ACTION" {
        // Quiz essence rows pull their `title`/`description` from the matching
        // `SpaceAction` row (quizzes themselves only carry questions). The
        // initial `create_quiz` transact writes SpaceAction with empty copy,
        // then `update_space_action` fills it in later — so we re-index the
        // underlying quiz whenever the action metadata changes to pick up
        // the new title/description. Polls/Discussions/Follows carry their
        // own copy and don't need this follow-up.
        use crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz;
        use crate::features::spaces::pages::actions::models::SpaceAction;
        use crate::features::spaces::pages::actions::types::SpaceActionType;
        let action: SpaceAction = deserialize(image)?;
        if matches!(action.space_action_type, SpaceActionType::Quiz) {
            let space_pk: crate::common::types::Partition = action.pk.0.clone().into();
            let quiz_sk = crate::common::types::EntityType::SpaceQuiz(action.pk.1.clone());
            if let Ok(Some(quiz)) = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk)).await {
                crate::features::essence::services::index_quiz(cli, &quiz).await?;
            }
        }
    } else if sk.starts_with("POST_COMMENT#") || sk.starts_with("POST_COMMENT_REPLY#") {
        // Both top-level comments and replies use the `PostComment` model
        // but differ by sk. `POST_COMMENT_LIKE#` is intentionally excluded —
        // it deserializes to a different shape and isn't essence-indexed.
        let comment: crate::features::posts::models::PostComment = deserialize(image)?;
        crate::features::essence::services::index_post_comment(cli, &comment).await?;
    } else if sk.starts_with("SPACE_POST_COMMENT#") || sk.starts_with("SPACE_POST_COMMENT_REPLY#")
    {
        // Same pattern for discussion comments. `SPACE_POST_COMMENT_LIKE#` is
        // a different entity (SpacePostCommentLike) and not essence-indexed.
        let comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment = deserialize(image)?;
        crate::features::essence::services::index_discussion_comment(cli, &comment).await?;
    } else if sk.starts_with("SPACE_POLL#") {
        // `SPACE_POLL_USER_ANSWER#` (SpacePollUserAnswer) starts with
        // "SPACE_POLL_" not "SPACE_POLL#" so it's excluded by this prefix.
        let poll: crate::features::spaces::pages::actions::actions::poll::SpacePoll =
            deserialize(image)?;
        crate::features::essence::services::index_poll(cli, &poll).await?;
    } else if sk.starts_with("SPACE_QUIZ#") {
        // `SPACE_QUIZ_ANSWER#` and `SPACE_QUIZ_ATTEMPT#` start with
        // "SPACE_QUIZ_" not "SPACE_QUIZ#" so they're excluded.
        let quiz: crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz =
            deserialize(image)?;
        crate::features::essence::services::index_quiz(cli, &quiz).await?;
    }
    Ok(())
}

/// Mirror a REMOVE of any essence-indexable entity by detaching its row
/// from the user's Essence list. Replaces the per-controller cascade that
/// was missing in the previous synchronous design.
#[cfg(feature = "server")]
async fn essence_remove_dispatch(
    sk: &str,
    image: &std::collections::HashMap<String, serde_dynamo::AttributeValue>,
) -> crate::common::Result<()> {
    use crate::common::Error;
    use crate::common::utils::InfraError;

    fn deserialize<T: serde::de::DeserializeOwned>(
        image: &std::collections::HashMap<String, serde_dynamo::AttributeValue>,
    ) -> crate::common::Result<T> {
        serde_dynamo::from_item(image.clone()).map_err(|e| {
            tracing::error!("essence remove deserialize: {e}");
            Error::from(InfraError::StreamDeserializeFailed)
        })
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    if sk == "POST" {
        let post: crate::features::posts::models::Post = deserialize(image)?;
        crate::features::essence::services::detach_post(cli, &post).await?;
    } else if sk.starts_with("POST_COMMENT#") || sk.starts_with("POST_COMMENT_REPLY#") {
        let comment: crate::features::posts::models::PostComment = deserialize(image)?;
        crate::features::essence::services::detach_post_comment(cli, &comment).await?;
    } else if sk.starts_with("SPACE_POST_COMMENT#") || sk.starts_with("SPACE_POST_COMMENT_REPLY#")
    {
        let comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment = deserialize(image)?;
        crate::features::essence::services::detach_discussion_comment(cli, &comment).await?;
    } else if sk.starts_with("SPACE_POLL#") {
        let poll: crate::features::spaces::pages::actions::actions::poll::SpacePoll =
            deserialize(image)?;
        crate::features::essence::services::detach_poll(cli, &poll).await?;
    } else if sk.starts_with("SPACE_QUIZ#") {
        let quiz: crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz =
            deserialize(image)?;
        crate::features::essence::services::detach_quiz(cli, &quiz).await?;
    }
    Ok(())
}
