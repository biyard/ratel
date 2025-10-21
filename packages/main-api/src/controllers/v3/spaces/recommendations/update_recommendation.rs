use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::recommendations::{SpaceRecommendation, SpaceRecommendationResponse};
use crate::models::space::SpaceCommon;

use crate::types::{File, Partition, TeamGroupPermission};
use crate::{AppState, Error};

use bdk::prelude::*;

use by_axum::axum::extract::{Json, Path, State};

use crate::controllers::v3::spaces::SpacePath;
use crate::models::user::User;
use aide::NoApi;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum UpdateRecommendationRequest {
    Content { html_contents: String },
    File { files: Vec<File> },
}

pub async fn update_recommendation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateRecommendationRequest>,
) -> crate::Result<Json<SpaceRecommendationResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundPoll);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    match req {
        UpdateRecommendationRequest::File { files } => {
            let recommendation =
                SpaceRecommendation::update_files(&dynamo.client, space_pk, files).await?;
            Ok(Json(recommendation.into()))
        }
        UpdateRecommendationRequest::Content { html_contents } => {
            let recommendation =
                SpaceRecommendation::update_contents(&dynamo.client, space_pk, html_contents)
                    .await?;
            Ok(Json(recommendation.into()))
        }
    }
}
