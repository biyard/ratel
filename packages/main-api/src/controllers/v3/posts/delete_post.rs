use crate::{
    AppState, Error2,
    models::feed::{Post, PostArtwork, PostAuthor, PostComment, PostMetadata, PostRepost},
    types::{EntityType, Partition, TeamGroupPermission},
    utils::{
        dynamo_extractor::extract_user,
        security::{RatelResource, check_permission},
    },
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct DeletePostPathParams {
    pub post_pk: String,
}

pub async fn delete_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<DeletePostPathParams>,
) -> Result<(), Error2> {
    let post = Post::get(&dynamo.client, &params.post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::NotFound("Post not found".to_string()))?;

    match post.user_pk {
        Partition::Team(_) => {
            check_permission(
                &dynamo.client,
                auth.clone(),
                RatelResource::Team {
                    team_pk: post.user_pk.to_string(),
                },
                vec![TeamGroupPermission::PostDelete],
            )
            .await?;
        }
        Partition::User(_) => {
            let user = extract_user(&dynamo.client, auth).await?;
            if user.pk != post.user_pk {
                return Err(Error2::Unauthorized(
                    "You do not have permission to delete this post".into(),
                ));
            }
        }
        _ => return Err(Error2::InternalServerError("Invalid post author".into())),
    }
    let metadata = PostMetadata::query(&dynamo.client, post.pk.clone()).await?;
    for data in metadata.into_iter() {
        match data {
            PostMetadata::Post(v) => {
                Post::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            PostMetadata::PostAuthor(v) => {
                PostAuthor::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            PostMetadata::PostComment(v) => {
                PostComment::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            PostMetadata::PostArtwork(v) => {
                PostArtwork::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            PostMetadata::PostRepost(v) => {
                PostRepost::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
        };
    }

    Ok(())
}
