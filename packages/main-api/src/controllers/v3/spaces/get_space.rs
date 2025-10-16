use super::*;
use crate::models::user::User;
use crate::models::{Post, SpaceCommon};
use crate::types::*;
use crate::{AppState, Error2};
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
}

pub async fn get_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<GetSpaceResponse>, Error2> {
    let space = SpaceCommon::get(&dynamo.client, &space_pk, Some(&EntityType::SpaceCommon));

    let post_pk = space_pk.clone().to_post_key()?;
    let post = Post::get(&dynamo.client, &post_pk, Some(&EntityType::Post));

    let (space, post) = tokio::try_join!(space, post)?;

    let space = space.ok_or(Error2::SpaceNotFound)?;
    let post = post.ok_or(Error2::PostNotFound)?;

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
        }
    }
}
