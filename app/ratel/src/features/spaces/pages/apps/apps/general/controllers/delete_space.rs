use crate::features::spaces::pages::apps::apps::general::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteSpaceResponse {
    pub message: String,
}

#[delete("/api/spaces/{space_id}/settings", user: crate::features::auth::User)]
pub async fn delete_space(space_id: SpacePartition) -> crate::common::Result<DeleteSpaceResponse> {
    use crate::common::models::space::SpaceCommon;
    use crate::common::types::{EntityType, Partition};
    use crate::features::posts::models::{Post, Team};
    use crate::features::posts::types::TeamGroupPermission;
    use crate::features::spaces::pages::actions::actions::poll::SpacePoll;

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    // Check permission: individual creator or team admin
    let is_admin = match &space.user_pk {
        Partition::User(_) => space.user_pk == user.pk,
        Partition::Team(_) => {
            Team::has_permission(dynamo, &space.user_pk, &user.pk, TeamGroupPermission::TeamAdmin)
                .await
                .unwrap_or(false)
        }
        _ => false,
    };

    if !is_admin {
        return Err(Error::NoPermission);
    }

    let space_pk: Partition = space.pk.clone();
    let post_pk = space_pk.clone().to_post_key()?;

    SpaceCommon::delete(dynamo, &space.pk, Some(space.sk)).await?;
    SpacePoll::delete_one(dynamo, &space_pk).await?;

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
