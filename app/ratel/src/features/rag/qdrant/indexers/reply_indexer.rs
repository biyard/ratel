use crate::common::Result;
use crate::features::rag::qdrant::payloads::ReplyPayload;
use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};
use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

use super::{discussion_collection, upsert_point};

/// Index a SpacePostComment into the discussion-scoped Qdrant collection.
pub async fn index_reply(comment: SpacePostComment) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let bedrock = config.bedrock_embeddings();
    let qdrant = config.qdrant();

    let space_pk_str = comment
        .space_pk
        .as_ref()
        .map(|p| p.to_string())
        .unwrap_or_default();
    let space_id = space_pk_str
        .strip_prefix("SPACE#")
        .unwrap_or(&space_pk_str);

    let discussion_sk = comment.pk.to_string();

    if space_id.is_empty() || discussion_sk.is_empty() {
        tracing::warn!(
            comment_sk = %comment.sk,
            "Skipping Qdrant index: missing space_pk or discussion_sk"
        );
        return Ok(());
    }

    let collection = discussion_collection(space_id, &discussion_sk);
    let point_id = comment.sk.to_string();
    let vector = bedrock.embed(&comment.content).await?;

    let payload = ReplyPayload {
        r#type: QdrantIndexType::Reply,
        content: comment.content,
        author: comment.author_display_name,
        author_pk: comment.author_pk.to_string(),
    };

    upsert_point(qdrant, &collection, &point_id, vector, payload.into_payload()).await
}
