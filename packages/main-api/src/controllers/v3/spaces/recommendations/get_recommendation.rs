use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::recommendations::{SpaceRecommendation, SpaceRecommendationResponse};

use crate::types::{EntityType, Partition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};

use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_recommendation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<SpaceRecommendationResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpaceRecommendation::keys(&space_pk);

    let recommendation =
        SpaceRecommendation::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    let recommendation = recommendation.unwrap_or_default();

    Ok(Json(recommendation.into()))
}
