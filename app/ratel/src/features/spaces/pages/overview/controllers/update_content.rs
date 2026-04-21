use crate::features::spaces::pages::overview::*;
use crate::features::spaces::types::SpaceError;
#[cfg(feature = "server")]
use crate::common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;
#[cfg(feature = "server")]
use crate::features::posts::models::Post;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContentRequest {
    pub content: String,
}

#[patch("/api/spaces/{space_pk}/overview/content", role: SpaceUserRole)]
pub async fn update_space_content(
    space_pk: SpacePartition,
    req: UpdateContentRequest,
) -> Result<()> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let space_partition = Partition::Space(space_pk.to_string());
    let post_partition = space_partition.clone().to_post_key()?;
    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let update_space = SpaceCommon::updater(&space_partition, EntityType::SpaceCommon)
        .with_content(req.content.clone())
        .execute(dynamo);
    let update_post = Post::updater(&post_partition, EntityType::Post)
        .with_html_contents(req.content)
        .execute(dynamo);

    tokio::try_join!(update_space, update_post).map_err(|e| {
        crate::error!("failed to update space/post content: {e:?}");
        SpaceError::UpdateFailed
    })?;

    Ok(())
}
