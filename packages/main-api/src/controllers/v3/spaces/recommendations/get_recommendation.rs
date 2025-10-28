use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::recommendations::{SpaceRecommendation, SpaceRecommendationResponse};
use crate::models::space::SpaceCommon;

use crate::models::user::User;
use crate::types::{Partition, TeamGroupPermission};
use crate::{AppState, Error};

use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_recommendation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<SpaceRecommendationResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpaceRecommendation::keys(&space_pk);

    let recommendation =
        SpaceRecommendation::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    let recommendation = recommendation.unwrap_or_default();

    Ok(Json(recommendation.into()))
}
