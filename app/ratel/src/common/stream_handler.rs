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
            .map_err(|e| Error::InternalServerError(format!("stream deserialize: {e}")))
    }

    match event_name {
        "INSERT" => {
            let image = new_image.ok_or(Error::InternalServerError(
                "INSERT event missing NewImage".into(),
            ))?;
            let sk = get_sk(image).unwrap_or_default();

            if sk.starts_with("SPACE_POST_COMMENT#") {
                // AiModeratorReplyIndex: index comment into Qdrant
                let comment = deserialize(image)?;
                if let Err(e) =
                    crate::features::rag::qdrant::indexers::reply_indexer::index_reply(comment).await
                {
                    tracing::error!(error = %e, "stream: AiModeratorReplyIndex failed");
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
            }
        }
        "MODIFY" => {
            let image = new_image.ok_or(Error::InternalServerError(
                "MODIFY event missing NewImage".into(),
            ))?;
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
            }
        }
        "REMOVE" => {
            let image = old_image.ok_or(Error::InternalServerError(
                "REMOVE event missing OldImage".into(),
            ))?;
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
