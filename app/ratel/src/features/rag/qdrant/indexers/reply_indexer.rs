use crate::common::Result;
use crate::features::rag::qdrant::payloads::ReplyPayload;
use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

/// Index a SpacePostComment into Qdrant.
pub async fn index_reply(comment: SpacePostComment) -> Result<()> {
    if comment.content.is_empty() {
        return Ok(());
    }

    let tenant_id = super::tenant_id();
    let payload = ReplyPayload::from_comment(&comment, tenant_id);

    if payload.space_id.0.is_empty() || payload.discussion_id.is_empty() {
        tracing::warn!(
            comment_id = %payload.comment_id,
            "Skipping Qdrant index: missing space_id or discussion_id"
        );
        return Ok(());
    }

    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();
    payload.upsert_points(qdrant).await?;
    Ok(())
}
