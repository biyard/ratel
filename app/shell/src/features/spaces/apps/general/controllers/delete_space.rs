use crate::features::spaces::apps::general::*;
#[cfg(feature = "server")]
use common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteSpaceResponse {
    pub message: String,
}

#[delete("/api/spaces/{space_id}/settings", role: SpaceUserRole)]
pub async fn delete_space(space_id: SpacePartition) -> common::Result<DeleteSpaceResponse> {
    use common::models::space::SpaceCommon;
    use common::types::{EntityType, Partition};
    use crate::features::posts::models::Post;
    use crate::features::spaces::actions::poll::SpacePoll;

    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    let space_pk: Partition = space.pk.clone();
    let post_pk = space_pk.clone().to_post_key()?;

    SpaceCommon::delete(dynamo, &space.pk, Some(space.sk)).await?;
    SpacePoll::delete_one(dynamo, &space_pk).await?;
    // TODO(main-api only): delete SpaceDiscussion entities linked to this space.
    // TODO(main-api only): delete SpaceFile entities linked to this space.
    // TODO(main-api only): delete SpaceRecommendation entities linked to this space.

    if Post::get(dynamo, &post_pk, Some(EntityType::Post))
        .await?
        .is_some()
    {
        Post::updater(post_pk, EntityType::Post)
            .remove_space_pk()
            .remove_space_type()
            .remove_space_visibility()
            .execute(dynamo)
            .await?;
    }

    Ok(DeleteSpaceResponse {
        message: format!("Space '{}' deleted", space_id),
    })
}
