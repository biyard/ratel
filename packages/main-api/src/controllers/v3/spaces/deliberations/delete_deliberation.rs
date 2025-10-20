use crate::{
    AppState, Error2,
    models::{
        DeliberationSpaceParticipant, User,
        space::{
            DeliberationDiscussionMember, DeliberationMetadata, DeliberationSpaceContent,
            DeliberationSpaceDiscussion, DeliberationSpaceElearning, SpaceCommon,
        },
    },
    types::{Partition, TeamGroupPermission},
};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

use aide::NoApi;

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct DeleteDeliberationResponse {
    pub space_pk: Partition,
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
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationDeletePath { space_pk }): Path<DeliberationDeletePath>,
) -> Result<Json<DeleteDeliberationResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundDeliberationSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceDelete,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;

    for data in metadata.into_iter() {
        match data {
            // DeliberationMetadata::DeliberationSpace(v) => {
            //     DeliberationSpace::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            // }
            DeliberationMetadata::DeliberationSpaceContent(v) => {
                DeliberationSpaceContent::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }

            DeliberationMetadata::DeliberationSpaceParticipant(v) => {
                DeliberationSpaceParticipant::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceMember(v) => {
                DeliberationDiscussionMember::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
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

    Ok(Json(DeleteDeliberationResponse { space_pk }))
}
