use crate::{
    AppState, Error2,
    controllers::v3::spaces::deliberations::update_deliberation::DeliberationPath,
    models::{
        DeliberationContentResponse, User,
        space::{DeliberationDetailResponse, DeliberationMetadata, SpaceCommon},
    },
    types::{Partition, TeamGroupPermission},
};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn get_deliberation_recommendation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
) -> Result<Json<DeliberationContentResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundDeliberationSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;
    tracing::debug!("Deliberation metadata retrieved: {:?}", metadata);
    let metadata: DeliberationDetailResponse = metadata.into();

    tracing::debug!("DeliberationDetailResponse formed: {:?}", metadata);

    Ok(Json(metadata.recommendation))
}
