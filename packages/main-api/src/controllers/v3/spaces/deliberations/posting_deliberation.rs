use crate::{
    AppState, Error2,
    controllers::v3::spaces::deliberations::update_deliberation::DeliberationPath,
    models::space::{
        DeliberationDetailResponse, DeliberationMetadata, DeliberationSpace, SpaceCommon,
    },
    types::{EntityType, Partition, SpaceStatus, TeamGroupPermission},
    utils::{
        dynamo_extractor::extract_user_from_session,
        security::{RatelResource, check_permission_from_session},
    },
};
use bdk::prelude::axum::{
    Extension,
    extract::{Json, Path, State},
};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use urlencoding::decode;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct PostingDeliberationRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct PostingDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

pub async fn posting_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
    Json(_req): Json<PostingDeliberationRequest>,
) -> Result<Json<PostingDeliberationResponse>, Error2> {
    let space_pk = decode(&space_pk).unwrap_or_default().to_string();
    let _user = extract_user_from_session(&dynamo.client, &session).await?;

    let space = DeliberationSpace::get(&dynamo.client, &space_pk, Some(EntityType::Space))
        .await?
        .ok_or(Error2::NotFound("Space not found".to_string()))?;
    let _ = match space.user_pk.clone() {
        Partition::Team(_) => {
            check_permission_from_session(
                &dynamo.client,
                &session,
                RatelResource::Team {
                    team_pk: space.user_pk.to_string(),
                },
                vec![TeamGroupPermission::SpaceEdit],
            )
            .await?;
        }
        Partition::User(_) => {
            let user = extract_user_from_session(&dynamo.client, &session).await?;
            if user.pk != space.user_pk {
                return Err(Error2::Unauthorized(
                    "You do not have permission to posting this deliberation".into(),
                ));
            }
        }
        _ => return Err(Error2::InternalServerError("Invalid post author".into())),
    };

    SpaceCommon::updater(&space_pk, EntityType::SpaceCommon)
        .with_status(SpaceStatus::InProgress)
        .execute(&dynamo.client)
        .await?;
    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;
    let metadata: DeliberationDetailResponse = metadata.into();
    Ok(Json(PostingDeliberationResponse { metadata }))
}
