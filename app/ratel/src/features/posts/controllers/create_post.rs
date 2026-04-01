// Migrated from packages/main-api/src/controllers/v3/posts/create_post.rs
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreatePostResponse {
    pub post_pk: Partition,
}

#[mcp_tool(name = "create_post", description = "Create a new draft post in Ratel.")]
#[post("/api/posts", user: User)]
pub async fn create_post_handler(team_id: Option<TeamPartition>) -> Result<CreatePostResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let author: Author = if let Some(team_id) = team_id {
        tracing::debug!(
            "Creating post under team: {:?} by user {:?}",
            team_id,
            user.pk
        );
        let team_pk: Partition = team_id.into();
        Team::get_permitted_team(cli, team_pk, user.pk, TeamGroupPermission::PostWrite)
            .await?
            .into()
    } else {
        user.into()
    };

    let post = Post::draft(author);
    post.create(cli).await?;

    Ok(CreatePostResponse { post_pk: post.pk })
}
