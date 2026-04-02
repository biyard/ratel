use crate::common::Result;
use crate::common::types::{FeedPartition, Partition};
use crate::features::posts::models::Post;
use crate::features::posts::types::PostStatus;
use crate::features::rag::qdrant::payloads::PostPayload;

/// Index a published Post into Qdrant.
pub async fn index_post(post: Post) -> Result<()> {
    if post.status != PostStatus::Published {
        return Ok(());
    }

    let plain_text = crate::features::posts::utils::extract_plain_text(&post.html_contents);
    let plain_text_preview: String = plain_text.chars().take(500).collect();

    if post.title.is_empty() && plain_text_preview.is_empty() {
        return Ok(());
    }

    let tenant_id = super::tenant_id();
    let payload = PostPayload::from_post(&post, tenant_id, plain_text_preview);

    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();
    payload.upsert_points(qdrant).await?;
    Ok(())
}

/// Delete a Post's vector from Qdrant.
pub async fn delete_post_index(post: Post) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();
    let point_id = match &post.pk {
        Partition::Feed(uuid) => FeedPartition(uuid.clone()),
        _ => FeedPartition(format!("{:?}", post.pk)),
    };
    PostPayload::delete_points(qdrant, &point_id.to_string()).await?;
    Ok(())
}
