use crate::features::posts::models::Post;
use crate::features::posts::types::TeamGroupPermission;
use crate::features::posts::*;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::SpacePartition;
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateSpaceRequest {
    pub post_id: FeedPartition,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateSpaceResponse {
    pub space_id: SpacePartition,
}

#[mcp_tool(name = "create_space", description = "Create a space on an existing post.")]
#[post("/api/spaces/create", user: User)]
pub async fn create_space_handler(req: CreateSpaceRequest) -> Result<CreateSpaceResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = req.post_id.clone().into();
    let (post, has_perm) =
        Post::has_permission(cli, &post_pk, Some(&user.pk), TeamGroupPermission::PostEdit).await?;
    if !has_perm {
        return Err(Error::Unauthorized("No permission".into()));
    }

    let space = SpaceCommon::new(
        req.post_id.clone(),
        post.user_pk.clone(),
        post.author_display_name.clone(),
        post.author_profile_url.clone(),
        post.author_username.clone(),
    );
    let participant = SpaceParticipant::new_non_anonymous(space.pk.clone(), user.clone());

    let post_updater = Post::updater(&post.pk, &post.sk).with_space_pk(space.pk.clone());

    crate::transact_write_items!(
        cli,
        vec![
            space.create_transact_write_item(),
            participant.create_transact_write_item(),
            post_updater.transact_write_item(),
        ]
    )?;

    let space_id = match space.pk.clone() {
        Partition::Space(id) => SpacePartition(id),
        _ => SpacePartition::default(),
    };

    Ok(CreateSpaceResponse { space_id })
}
