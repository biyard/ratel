use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::files::{FileLink, FileLinkTarget};
use crate::features::spaces::recommendations::{SpaceRecommendation, SpaceRecommendationResponse};

use crate::types::{File, Partition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};

use bdk::prelude::*;

use by_axum::axum::extract::{Json, Path, State};

use crate::controllers::v3::spaces::SpacePath;
use aide::NoApi;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum UpdateRecommendationRequest {
    Content { html_contents: String },
    File { files: Vec<File> },
}

pub async fn update_recommendation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateRecommendationRequest>,
) -> crate::Result<Json<SpaceRecommendationResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundPoll);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    match req {
        UpdateRecommendationRequest::File { files } => {
            let recommendation =
                SpaceRecommendation::update_files(&dynamo.client, space_pk.clone(), files.clone())
                    .await?;

            // Link files to both Files tab and Overview
            for file in &files {
                if let Some(url) = &file.url {
                    // Link to Files tab
                    FileLink::add_link_target(
                        &dynamo.client,
                        space_pk.clone(),
                        url.clone(),
                        FileLinkTarget::Files,
                    )
                    .await?;

                    // Link to Overview
                    FileLink::add_link_target(
                        &dynamo.client,
                        space_pk.clone(),
                        url.clone(),
                        FileLinkTarget::Overview,
                    )
                    .await?;
                }
            }

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
