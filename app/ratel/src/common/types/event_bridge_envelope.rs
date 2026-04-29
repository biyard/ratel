#[cfg(feature = "lambda")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventBridgeEnvelope {
    pub source: String,
    #[serde(rename = "detail-type")]
    pub detail_type: DetailType,
    pub detail: serde_json::Value,
    #[serde(default)]
    pub account: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub resources: Vec<String>,
}

#[cfg(feature = "lambda")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DetailType {
    TimelineUpdate,
    PopularPostUpdate,
    PopularSpaceUpdate,
    NotificationSend,
    PostVectorIndex,
    PostVectorDelete,
    AiModeratorReplyCheck,
    AiModeratorReplyIndex,
    ActivityScoreAggregate,
    SpaceStatusChangeEvent,
    /// Fires on SPACE_ACTION MODIFY when status transitions from DESIGNING to
    /// ONGOING. Drives participant fan-out (inbox + email) for newly-live
    /// actions inside an Ongoing space.
    SpaceActionStatusChange,
    PollXpRecord,
    QuizXpRecord,
    DiscussionXpRecord,
    FollowXpRecord,
    // Essence indexing — driven by DynamoDB Stream so writes don't have to
    // wait on a synchronous index call. Each variant maps to a single source
    // entity type. Index variants fire on INSERT/MODIFY; Delete variants on
    // REMOVE. The underlying handlers live in `essence::services` and are
    // shared with the migrate endpoint.
    EssenceIndexPost,
    EssenceIndexPostComment,
    EssenceIndexDiscussionComment,
    EssenceIndexPoll,
    EssenceIndexQuiz,
    /// Fires on SPACE_ACTION INSERT/MODIFY. Quiz essence rows derive their
    /// title/description from the matching SpaceAction row, so we re-index
    /// the underlying quiz whenever action metadata changes.
    EssenceActionMetadataUpdate,
    EssenceDeletePost,
    EssenceDeletePostComment,
    EssenceDeleteDiscussionComment,
    EssenceDeletePoll,
    EssenceDeleteQuiz,
    /// Fires on SubTeamAnnouncement MODIFY with status=Published. Drives the
    /// broadcast fan-out: create a pinned Post in every recognized sub-team
    /// feed and notify each member.
    SubTeamAnnouncementPublished,
    #[serde(other)]
    Unknown,
}

#[cfg(feature = "lambda")]
impl DetailType {
    fn parse_detail<T: serde::de::DeserializeOwned>(
        detail: &serde_json::Value,
    ) -> Result<T, lambda_runtime::Error> {
        let new_image = detail
            .get("newImage")
            .ok_or("missing newImage in detail")?;

        let item: std::collections::HashMap<String, serde_dynamo::AttributeValue> =
            serde_json::from_value(new_image.clone())
                .map_err(|e| format!("failed to parse DynamoDB image: {}", e))?;

        serde_dynamo::from_item(item)
            .map_err(|e| lambda_runtime::Error::from(format!("failed to deserialize: {}", e)))
    }
}

#[cfg(feature = "lambda")]
impl EventBridgeEnvelope {
    pub async fn proc(self) -> Result<(), lambda_runtime::Error> {
        tracing::info!(
            detail_type = ?self.detail_type,
            "Received EventBridge event"
        );

        let result = match self.detail_type {
            DetailType::TimelineUpdate => {
                crate::features::timeline::services::fan_out_timeline_entries(
                    DetailType::parse_detail(&self.detail)?,
                )
                .await
            }
            DetailType::PopularPostUpdate => {
                crate::features::timeline::services::fan_out_popular_post(
                    DetailType::parse_detail(&self.detail)?,
                )
                .await
            }
            DetailType::PopularSpaceUpdate => {
                let space: crate::common::models::space::SpaceCommon =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = space.pk.clone();
                let r = crate::features::timeline::services::fan_out_popular_space(space).await;
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::NotificationSend => {
                let notification: crate::common::models::notification::Notification =
                    DetailType::parse_detail(&self.detail)?;
                notification.process().await
            }
            DetailType::PostVectorIndex => {
                let post: crate::features::posts::models::Post =
                    DetailType::parse_detail(&self.detail)?;
                crate::features::rag::qdrant::indexers::post_indexer::index_post(post).await
            }
            DetailType::PostVectorDelete => {
                let post: crate::features::posts::models::Post =
                    DetailType::parse_detail(&self.detail)?;
                crate::features::rag::qdrant::indexers::post_indexer::delete_post_index(post).await
            }
            DetailType::AiModeratorReplyCheck => {
                let post: crate::features::spaces::pages::actions::actions::discussion::SpacePost =
                    DetailType::parse_detail(&self.detail)?;
                crate::features::ai_moderator::services::event_handler::handle_ai_moderator_event(
                    post,
                )
                .await
            }
            DetailType::AiModeratorReplyIndex => {
                let comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment =
                    DetailType::parse_detail(&self.detail)?;
                crate::features::rag::qdrant::indexers::reply_indexer::index_reply(comment).await
            }
            DetailType::ActivityScoreAggregate => {
                let activity: crate::features::activity::models::SpaceActivity =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = activity.space_pk.clone();
                let r = crate::features::activity::services::aggregate_score(activity).await;
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::SpaceStatusChangeEvent => {
                let event: crate::common::models::space::SpaceStatusChangeEvent =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = event.space_pk.clone();
                let r =
                    crate::features::spaces::space_common::services::handle_space_status_change(
                        event,
                    )
                    .await;
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::SpaceActionStatusChange => {
                let action: crate::features::spaces::pages::actions::models::SpaceAction =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = action.space_pk.clone();
                let r = crate::features::spaces::pages::actions::services::notify_action_ongoing(
                    action,
                )
                .await;
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::PollXpRecord => {
                let answer: crate::features::spaces::pages::actions::actions::poll::SpacePollUserAnswer =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = space_pk_from_id_str(answer.space_id.as_deref());
                let r = crate::features::activity::services::handle_poll_xp(answer).await;
                fanout_if_some(space_pk.as_ref()).await;
                r
            }
            DetailType::QuizXpRecord => {
                let attempt: crate::features::spaces::pages::actions::actions::quiz::SpaceQuizAttempt =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = space_pk_from_id_str(attempt.space_id.as_deref());
                let r = crate::features::activity::services::handle_quiz_xp(attempt).await;
                fanout_if_some(space_pk.as_ref()).await;
                r
            }
            DetailType::DiscussionXpRecord => {
                let comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = comment.space_pk.clone();
                let r = crate::features::activity::services::handle_discussion_xp(comment).await;
                fanout_if_some(space_pk.as_ref()).await;
                r
            }
            DetailType::FollowXpRecord => {
                let follow: crate::common::models::auth::UserFollow =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = space_pk_from_id_str(follow.space_id.as_deref());
                let r = crate::features::activity::services::handle_follow_xp(follow).await;
                fanout_if_some(space_pk.as_ref()).await;
                r
            }
            DetailType::EssenceIndexPost => {
                let post: crate::features::posts::models::Post =
                    DetailType::parse_detail(&self.detail)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                crate::features::essence::services::index_post(cli, &post).await
            }
            DetailType::EssenceIndexPostComment => {
                let comment: crate::features::posts::models::PostComment =
                    DetailType::parse_detail(&self.detail)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                crate::features::essence::services::index_post_comment(cli, &comment).await
            }
            DetailType::EssenceIndexDiscussionComment => {
                let comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = comment.space_pk.clone();
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                let r =
                    crate::features::essence::services::index_discussion_comment(cli, &comment)
                        .await;
                fanout_if_some(space_pk.as_ref()).await;
                r
            }
            DetailType::EssenceIndexPoll => {
                let poll: crate::features::spaces::pages::actions::actions::poll::SpacePoll =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = poll.pk.clone();
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                let r = crate::features::essence::services::index_poll(cli, &poll).await;
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::EssenceIndexQuiz => {
                let quiz: crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = quiz.pk.clone();
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                let r = crate::features::essence::services::index_quiz(cli, &quiz).await;
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::EssenceActionMetadataUpdate => {
                use crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz;
                use crate::features::spaces::pages::actions::models::SpaceAction;
                use crate::features::spaces::pages::actions::types::SpaceActionType;
                let action: SpaceAction = DetailType::parse_detail(&self.detail)?;
                let space_pk: crate::common::types::Partition = action.pk.0.clone().into();
                let r = if matches!(action.space_action_type, SpaceActionType::Quiz) {
                    let cfg = crate::common::CommonConfig::default();
                    let cli = cfg.dynamodb();
                    let quiz_sk =
                        crate::common::types::EntityType::SpaceQuiz(action.pk.1.clone());
                    match SpaceQuiz::get(cli, &space_pk, Some(quiz_sk)).await {
                        Ok(Some(quiz)) => {
                            crate::features::essence::services::index_quiz(cli, &quiz).await
                        }
                        Ok(None) => Ok(()),
                        Err(e) => Err(e),
                    }
                } else {
                    Ok(())
                };
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::EssenceDeletePost => {
                let post: crate::features::posts::models::Post =
                    DetailType::parse_detail(&self.detail)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                crate::features::essence::services::detach_post(cli, &post).await
            }
            DetailType::EssenceDeletePostComment => {
                let comment: crate::features::posts::models::PostComment =
                    DetailType::parse_detail(&self.detail)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                crate::features::essence::services::detach_post_comment(cli, &comment).await
            }
            DetailType::EssenceDeleteDiscussionComment => {
                let comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = comment.space_pk.clone();
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                let r =
                    crate::features::essence::services::detach_discussion_comment(cli, &comment)
                        .await;
                fanout_if_some(space_pk.as_ref()).await;
                r
            }
            DetailType::EssenceDeletePoll => {
                let poll: crate::features::spaces::pages::actions::actions::poll::SpacePoll =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = poll.pk.clone();
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                let r = crate::features::essence::services::detach_poll(cli, &poll).await;
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::EssenceDeleteQuiz => {
                let quiz: crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz =
                    DetailType::parse_detail(&self.detail)?;
                let space_pk = quiz.pk.clone();
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                let r = crate::features::essence::services::detach_quiz(cli, &quiz).await;
                fanout_hot_space(&space_pk).await;
                r
            }
            DetailType::SubTeamAnnouncementPublished => {
                let announcement: crate::features::sub_team::models::SubTeamAnnouncement =
                    DetailType::parse_detail(&self.detail)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                crate::features::sub_team::services::announcement_fanout::handle_announcement_published(cli, announcement).await
            }
            DetailType::Unknown => {
                tracing::warn!(
                    "Unhandled EventBridge event: source={}",
                    self.source,
                );
                Ok(())
            }
        };

        if let Err(ref e) = result {
            tracing::error!(
                detail_type = ?self.detail_type,
                source = %self.source,
                error = %e,
                "EventBridge handler failed"
            );
        }

        result.map_err(lambda_runtime::Error::from)?;

        Ok(())
    }
}

/// Re-snapshot HotSpace (and per-viewer rows) for `space_pk`. Best-effort —
/// `services::space_fanout::upsert_hot_space` swallows its own errors so a
/// fanout miss never blocks the envelope handler that triggered us.
#[cfg(feature = "lambda")]
async fn fanout_hot_space(space_pk: &crate::common::types::Partition) {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    crate::features::spaces::space_common::services::upsert_hot_space(cli, space_pk).await;
}

#[cfg(feature = "lambda")]
async fn fanout_if_some(space_pk: Option<&crate::common::types::Partition>) {
    if let Some(pk) = space_pk {
        fanout_hot_space(pk).await;
    }
}

/// XP records carry the space id as a bare string (not a `Partition`); rebuild
/// the `Partition::Space` form for fanout. Empty/None inputs return None so
/// callers can skip cleanly.
#[cfg(feature = "lambda")]
fn space_pk_from_id_str(space_id: Option<&str>) -> Option<crate::common::types::Partition> {
    space_id
        .filter(|s| !s.is_empty())
        .map(|s| crate::common::types::Partition::Space(s.to_string()))
}
