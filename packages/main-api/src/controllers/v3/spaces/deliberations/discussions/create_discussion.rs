use crate::{
    AppState, Error2,
    models::{
        space::{
            DeliberationDiscussionResponse, DeliberationSpaceDiscussion, DeliberationSpaceMember,
        },
        user::User,
    },
    types::{EntityType, Partition},
    utils::dynamo_extractor::extract_user,
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};

use dto::{JsonSchema, aide, schemars};
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
    pub deliberation_id: String,
}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDiscussionResponse {
    pub deliberation_id: String,
}

pub async fn create_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(DeliberationDiscussionPath { deliberation_id }): Path<DeliberationDiscussionPath>,
    Json(req): Json<CreateDiscussionRequest>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;

    let disc = DeliberationSpaceDiscussion::new(
        crate::types::Partition::DeliberationSpace(deliberation_id.clone()),
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
        EntityType::DeliberationSpaceDiscussion(v) => v,
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

        let m = DeliberationSpaceMember::new(
            Partition::DeliberationSpace(deliberation_id.to_string()),
            Partition::Discussion(disc_id.clone()),
            user,
        );

        m.create(&dynamo.client).await?;
    }

    let disc = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &Partition::DeliberationSpace(deliberation_id.to_string()),
        Some(EntityType::DeliberationSpaceDiscussion(disc_id.to_string())),
    )
    .await?;

    let disc = disc.unwrap().into();

    Ok(Json(disc))
}
