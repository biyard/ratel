use crate::{
    AppState, Error2,
    models::space::{
        DeliberationMetadata, DeliberationSpace, DeliberationSpaceContent,
        DeliberationSpaceDiscussion, DeliberationSpaceElearning, DeliberationSpaceMember,
        DeliberationSpaceParticipant, DeliberationSpaceQuestion, DeliberationSpaceResponse,
        DeliberationSpaceSurvey, SpaceCommon,
    },
    types::{EntityType, Partition, TeamGroupPermission},
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct DeleteDeliberationResponse {
    pub space_pk: String,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationDeletePath {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

pub async fn delete_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationDeletePath { space_pk }): Path<DeliberationDeletePath>,
) -> Result<Json<DeleteDeliberationResponse>, Error2> {
    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;

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
                vec![TeamGroupPermission::SpaceDelete],
            )
            .await?;
        }
        Partition::User(_) => {
            let user = extract_user_from_session(&dynamo.client, &session).await?;
            if user.pk != space.user_pk {
                return Err(Error2::Unauthorized(
                    "You do not have permission to delete this deliberation".into(),
                ));
            }
        }
        _ => {
            return Err(Error2::InternalServerError(
                "Invalid deliberation author".into(),
            ));
        }
    };

    for data in metadata.into_iter() {
        match data {
            DeliberationMetadata::DeliberationSpace(v) => {
                DeliberationSpace::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceSurvey(v) => {
                DeliberationSpaceSurvey::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceContent(v) => {
                DeliberationSpaceContent::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceResponse(v) => {
                DeliberationSpaceResponse::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceQuestion(v) => {
                DeliberationSpaceQuestion::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceParticipant(v) => {
                DeliberationSpaceParticipant::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceMember(v) => {
                DeliberationSpaceMember::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceElearning(v) => {
                DeliberationSpaceElearning::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceDiscussion(v) => {
                DeliberationSpaceDiscussion::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::SpaceCommon(v) => {
                SpaceCommon::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
        }
    }

    Ok(Json(DeleteDeliberationResponse {
        space_pk: space_pk.to_string(),
    }))
}
