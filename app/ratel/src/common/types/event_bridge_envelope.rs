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
    PollXpRecord,
    QuizXpRecord,
    DiscussionXpRecord,
    FollowXpRecord,
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
                crate::features::timeline::services::fan_out_popular_space(
                    DetailType::parse_detail(&self.detail)?,
                )
                .await
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
                let activity = DetailType::parse_detail(&self.detail)?;
                crate::features::activity::services::aggregate_score(activity).await
            }
            DetailType::SpaceStatusChangeEvent => {
                let event: crate::common::models::space::SpaceStatusChangeEvent =
                    DetailType::parse_detail(&self.detail)?;
                crate::features::spaces::space_common::services::handle_space_status_change(event)
                    .await
            }
            DetailType::PollXpRecord => {
                let answer = DetailType::parse_detail(&self.detail)?;
                crate::features::activity::services::handle_poll_xp(answer).await
            }
            DetailType::QuizXpRecord => {
                let attempt = DetailType::parse_detail(&self.detail)?;
                crate::features::activity::services::handle_quiz_xp(attempt).await
            }
            DetailType::DiscussionXpRecord => {
                let comment = DetailType::parse_detail(&self.detail)?;
                crate::features::activity::services::handle_discussion_xp(comment).await
            }
            DetailType::FollowXpRecord => {
                let follow = DetailType::parse_detail(&self.detail)?;
                crate::features::activity::services::handle_follow_xp(follow).await
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
