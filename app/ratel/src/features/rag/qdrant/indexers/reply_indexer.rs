use crate::common::Result;
use crate::features::rag::qdrant::payloads::ReplyPayload;
use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};
use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

use super::{tenant_id, upsert_point};

/// Index a SpacePostComment into Qdrant.
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
        .unwrap_or(&space_pk_str)
        .to_string();

    let discussion_id = comment.pk.to_string();

    if space_id.is_empty() || discussion_id.is_empty() {
        tracing::warn!(
            comment_sk = %comment.sk,
            "Skipping Qdrant index: missing space_pk or discussion_sk"
        );
        return Ok(());
    }

    let point_id = comment.sk.to_string();
    let vector = bedrock.embed(&comment.content).await?;

    let payload = ReplyPayload {
        r#type: QdrantIndexType::Reply,
        tenant_id: tenant_id(),
        user_id: comment.author_pk.to_string(),
        space_id,
        discussion_id,
        content: comment.content,
        author: comment.author_display_name,
    };

    upsert_point(qdrant, &point_id, vector, payload.into_payload()).await
}
