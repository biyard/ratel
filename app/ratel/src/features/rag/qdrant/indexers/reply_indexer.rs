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
    tracing::debug!("Indexing data: {:?}", payload);

    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();
    payload.upsert_points(qdrant).await?;
    Ok(())
}
