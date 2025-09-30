use crate::models::feed::Post;
use crate::models::space::{PollSpace, SpaceCommon};
use crate::types::{EntityType, Partition};
use crate::utils::dynamo_extractor::extract_user_from_session;
use crate::{AppState, Error2};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreatePollSpaceRequest {
    #[schemars(description = "post PK")]
    post_pk: Partition,
    #[schemars(description = "The start time of the poll, in milliseconds since epoch")]
    started_at: i64, // milliseconds
    #[schemars(description = "The end time of the poll, in milliseconds since epoch")]
    ended_at: i64, // milliseconds
}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreatePollSpaceResponse {
    space_pk: Partition,
}

pub async fn create_poll_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Json(CreatePollSpaceRequest {
        post_pk,
        started_at,
        ended_at,
    }): Json<CreatePollSpaceRequest>,
) -> Result<Json<CreatePollSpaceResponse>, Error2> {
    let user = extract_user_from_session(&dynamo.client, &session).await?;

    let post = Post::get(&dynamo.client, &post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::NotFoundPost)?;

    // FIXME: Check Post Visibility

    let poll_space = PollSpace::new();

    poll_space.create(&dynamo.client).await?;
    SpaceCommon::new(poll_space.pk.clone(), post_pk, user.clone())
        .with_time(started_at, ended_at)
        .create(&dynamo.client)
        .await?;
    Post::updater(post.pk.clone(), post.sk.clone())
        .with_space_pk(poll_space.pk.clone())
        .execute(&dynamo.client)
        .await?;

    Ok(Json(CreatePollSpaceResponse {
        space_pk: poll_space.pk.clone(),
    }))
}
