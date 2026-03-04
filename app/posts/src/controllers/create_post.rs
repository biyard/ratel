// Migrated from packages/main-api/src/controllers/v3/posts/create_post.rs
use crate::models::*;
use crate::types::*;
use crate::*;
use ratel_auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreatePostResponse {
    pub post_pk: Partition,
}

#[post("/api/posts", user: User)]
pub async fn create_post_handler(team_id: Option<TeamPartition>) -> Result<CreatePostResponse> {
    let conf = crate::config::get();
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
