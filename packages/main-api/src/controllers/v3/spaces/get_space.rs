use super::*;
use crate::models::user::User;
use crate::models::{Post, SpaceCommon};
use crate::types::*;
use crate::{AppState, Error};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct GetSpaceResponse {
    pub pk: Partition,
    pub sk: EntityType,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub urls: Vec<String>,
    pub space_type: SpaceType,
    // TODO: implemente real features
    pub features: Vec<String>,
    pub status: Option<SpaceStatus>,
    pub permissions: i64,
    pub author_type: UserType,
    pub author_display_name: String,
    pub author_username: String,
    pub author_profile_url: String,
    pub certified: bool,

    pub likes: i64,
    pub comments: i64,
    pub shares: i64,
    pub rewards: Option<i64>,
    pub visibility: SpaceVisibility,
    pub publish_state: SpacePublishState,
    pub booster: BoosterType,
    pub files: Option<Vec<File>>,
}

pub async fn get_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<GetSpaceResponse>, Error> {
    let space = SpaceCommon::get(&dynamo.client, &space_pk, Some(&EntityType::SpaceCommon));

    let post_pk = space_pk.clone().to_post_key()?;
    let post = Post::get(&dynamo.client, &post_pk, Some(&EntityType::Post));

    let (space, post) = tokio::try_join!(space, post)?;

    let space = space.ok_or(Error::SpaceNotFound)?;
    let post = post.ok_or(Error::PostNotFound)?;

    let permissions = post.get_permissions(&dynamo.client, user).await?;

    Ok(Json(GetSpaceResponse::from((space, post, permissions))))
}

impl From<(SpaceCommon, Post, TeamGroupPermissions)> for GetSpaceResponse {
    fn from((space, post, permissions): (SpaceCommon, Post, TeamGroupPermissions)) -> Self {
        Self {
            pk: space.pk,
            sk: space.sk,
            title: post.title,
            content: match space.content.is_empty() {
                true => post.html_contents,
                false => space.content,
            },
            created_at: space.created_at,
            updated_at: space.updated_at,
            urls: post.urls,
            space_type: space.space_type,
            features: vec![],
            status: space.status,
            permissions: permissions.into(),
            author_type: post.author_type,
            author_display_name: post.author_display_name,
            author_username: post.author_username,
            author_profile_url: post.author_profile_url,

            // TODO: implement real certification check
            certified: false,
            likes: post.likes,
            comments: post.comments,
            shares: post.shares,
            rewards: space.rewards,
            visibility: space.visibility,
            publish_state: space.publish_state,
            booster: space.booster,

            files: space.files,
        }
    }
}
