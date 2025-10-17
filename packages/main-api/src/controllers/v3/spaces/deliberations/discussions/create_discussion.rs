use crate::{
    AppState, Error2,
    models::{
        space::{
            DeliberationDiscussionMember, DeliberationDiscussionResponse,
            DeliberationSpaceDiscussion,
        },
        user::User,
    },
    types::{EntityType, Partition},
};
use bdk::prelude::*;
use bdk::prelude::{
    aide::NoApi,
    axum::extract::{Json, Path, State},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct CreateDiscussionRequest {
    #[schemars(description = "Discussion name")]
    pub name: String,
    #[schemars(description = "Discussion description")]
    pub description: String,
    #[schemars(description = "Discussion start date")]
    pub started_at: i64,
    #[schemars(description = "Discussion end date")]
    pub ended_at: i64,
    #[schemars(description = "Discussion participants")]
    pub members: Vec<String>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationDiscussionPath {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDiscussionResponse {
    pub space_pk: String,
}

pub async fn create_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationDiscussionPath { space_pk }): Path<DeliberationDiscussionPath>,
    Json(req): Json<CreateDiscussionRequest>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let user = user.unwrap_or_default();

    let disc = DeliberationSpaceDiscussion::new(
        space_pk.clone(),
        req.name,
        req.description,
        req.started_at,
        req.ended_at,
        None,
        "".to_string(),
        None,
        None,
        user.clone(),
    );

    let disc_id = match disc.clone().sk {
        EntityType::DeliberationDiscussion(v) => v,
        _ => "".to_string(),
    };

    disc.create(&dynamo.client).await?;

    for member in req.members.into_iter() {
        let user = User::get(
            &dynamo.client,
            Partition::User(member),
            Some(EntityType::User),
        )
        .await?
        .ok_or(Error2::NotFound("User not found".into()))?;

        let m = DeliberationDiscussionMember::new(
            space_pk.clone(),
            Partition::Discussion(disc_id.clone()),
            user,
        );

        m.create(&dynamo.client).await?;
    }

    let disc =
        DeliberationSpaceDiscussion::get(&dynamo.client, &space_pk, Some(disc.clone().sk)).await?;

    let disc = disc.unwrap().into();

    Ok(Json(disc))
}
