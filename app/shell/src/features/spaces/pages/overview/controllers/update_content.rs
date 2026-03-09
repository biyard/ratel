use crate::features::spaces::pages::overview::*;
#[cfg(feature = "server")]
use common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use common::SpaceUserRole;

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

    let partition = Partition::Space(space_pk.to_string());
    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    SpaceCommon::updater(&partition, EntityType::SpaceCommon)
        .with_content(req.content)
        .execute(dynamo)
        .await
        .map_err(|e| Error::InternalServerError(format!("failed to update content: {e:?}")))?;

    Ok(())
}
