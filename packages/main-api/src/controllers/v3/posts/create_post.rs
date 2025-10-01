use crate::{
    AppState, Error2,
    models::{
        feed::{Post, PostAuthor},
        team::Team,
    },
    types::{Author, EntityType, PostType, TeamGroupPermission},
    utils::{
        dynamo_extractor::extract_user,
        security::{RatelResource, check_permission},
    },
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreatePostRequest {
    pub team_pk: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreatePostResponse {
    pub post_pk: String,
}

pub async fn create_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<CreatePostResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth.clone()).await?;
    let author: Author = if let Some(team_pk) = req.team_pk {
        check_permission(
            &dynamo.client,
            auth,
            RatelResource::Team {
                team_pk: team_pk.clone(),
            },
            vec![TeamGroupPermission::PostWrite],
        )
        .await?;
        let team = Team::get(&dynamo.client, &team_pk, Some(EntityType::Team))
            .await?
            .ok_or(Error2::NotFound("Team not found".to_string()))?;
        team.into()
    } else {
        user.clone().into()
    };

    let post = Post::new("", "", PostType::default(), author);
    post.create(&dynamo.client).await?;
    PostAuthor::new(post.pk.clone(), user)
        .create(&dynamo.client)
        .await?;

    Ok(Json(CreatePostResponse {
        post_pk: post.pk.to_string(),
    }))
}
