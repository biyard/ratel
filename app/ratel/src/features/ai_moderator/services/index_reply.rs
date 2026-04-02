use crate::common::Result;
use crate::features::ai_moderator::services::qdrant_service::{
    get_qdrant_client, QdrantIndexType, ReplyPayload, QdrantPayload,
};
use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

/// Index a SpacePostComment into the discussion-scoped Qdrant collection.
///
/// Called from DynamoStream when a new SpacePostComment is inserted.
/// Acquires QdrantClient and BedrockEmbeddingsClient from common config.
pub async fn index_reply(comment: SpacePostComment) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let bedrock = config.bedrock_embeddings();

    // Determine space_id and discussion_sk from the comment
    let space_pk_str = comment
        .space_pk
        .as_ref()
        .map(|p| p.to_string())
        .unwrap_or_default();
    let space_id = space_pk_str
        .strip_prefix("SPACE#")
        .unwrap_or(&space_pk_str);

    // pk of comment is the SpacePost partition (SPACE_POST#uuid)
    let discussion_sk = comment.pk.to_string();

    if space_id.is_empty() || discussion_sk.is_empty() {
        tracing::warn!(
            comment_sk = %comment.sk,
            "Skipping Qdrant index: missing space_pk or discussion_sk"
        );
        return Ok(());
    }

    let qdrant = get_qdrant_client(space_id, &discussion_sk);

    // Use the comment sk as the point id
    let point_id = comment.sk.to_string();

    let vector = bedrock.embed(&comment.content).await?;

    let payload = ReplyPayload {
        r#type: QdrantIndexType::Reply,
        content: comment.content,
        author: comment.author_display_name,
        author_pk: comment.author_pk.to_string(),
    };

    qdrant
        .upsert_point(point_id, vector, payload.into_payload())
        .await
}
