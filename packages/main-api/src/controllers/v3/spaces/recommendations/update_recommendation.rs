use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::files::{FileLink, FileLinkTarget, SpaceFile};
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
            let (pk, sk) = SpaceRecommendation::keys(&space_pk);
            let existing = SpaceRecommendation::get(&dynamo.client, pk, Some(sk)).await?;
            let old_file_urls: Vec<String> = existing
                .as_ref()
                .map(|r| r.files.iter().filter_map(|f| f.url.clone()).collect())
                .unwrap_or_default();

            let recommendation =
                SpaceRecommendation::update_files(&dynamo.client, space_pk.clone(), files.clone())
                    .await?;

            if !files.is_empty() {
                SpaceFile::add_files(&dynamo.client, space_pk.clone(), files.clone()).await?;
            }

            let new_file_urls: Vec<String> = files.iter().filter_map(|f| f.url.clone()).collect();
            if !new_file_urls.is_empty() {
                FileLink::add_link_targets_batch(
                    &dynamo.client,
                    space_pk.clone(),
                    new_file_urls.clone(),
                    FileLinkTarget::Overview,
                )
                .await?;
            }

            let removed_urls: Vec<String> = old_file_urls
                .into_iter()
                .filter(|url| !new_file_urls.contains(url))
                .collect();
            if !removed_urls.is_empty() {
                FileLink::remove_link_targets_batch(
                    &dynamo.client,
                    &space_pk,
                    removed_urls.clone(),
                    &FileLinkTarget::Overview,
                )
                .await?;

                // Also remove from SpaceFile
                let (pk, sk) = SpaceFile::keys(&space_pk);
                if let Some(mut space_file) =
                    SpaceFile::get(&dynamo.client, &pk, Some(sk.clone())).await?
                {
                    space_file.files.retain(|f| {
                        if let Some(url) = &f.url {
                            !removed_urls.contains(url)
                        } else {
                            true
                        }
                    });

                    SpaceFile::updater(&pk, sk)
                        .with_files(space_file.files.clone())
                        .execute(&dynamo.client)
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
