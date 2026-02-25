use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteSpaceResponse {
    pub message: String,
}

#[delete("/api/spaces/{space_id}/settings", user: ratel_auth::User)]
pub async fn delete_space(space_id: SpacePartition) -> common::Result<DeleteSpaceResponse> {
    use common::models::space::SpaceCommon;
    use common::types::{EntityType, Partition};
    use ratel_post::models::Post;
    use space_action_poll::SpacePoll;

    let dynamo = crate::config::get().common.dynamodb();
    let space = super::get_space_and_ensure_admin(&space_id, &user).await?;

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
