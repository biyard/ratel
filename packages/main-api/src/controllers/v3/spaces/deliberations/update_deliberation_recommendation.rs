use crate::models::{DeliberationContentResponse, DeliberationPath, Post, SpaceCommon};
use crate::types::{File, Partition, SpaceVisibility, TeamGroupPermission};
use crate::utils::aws::DynamoClient;
use crate::{
    AppState, Error2,
    models::{
        space::{DeliberationDetailResponse, DeliberationMetadata, DeliberationSpaceContent},
        user::User,
    },
    types::EntityType,
};
use aws_sdk_dynamodb::types::TransactWriteItem;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::Deserialize;
use validator::Validate;

use aide::NoApi;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct UpdateDeliberationRecommendationRequest {
    #[schemars(description = "Final Recommendation HTML contents")]
    pub recommendation_html_contents: Option<String>,
    #[schemars(description = "Final Recommendation files")]
    pub recommendation_files: Vec<File>,

    #[schemars(description = "Deliberation Title")]
    pub title: Option<String>,
    #[schemars(description = "Deliberation visibility")]
    pub visibility: SpaceVisibility,
    #[schemars(description = "Deliberation start date")]
    pub started_at: i64,
    #[schemars(description = "Deliberation end date")]
    pub ended_at: i64,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateDeliberationRecommendationResponse {
    #[serde(flatten)]
    pub space_common: SpaceCommon,
    pub recommendations: DeliberationContentResponse,
}

//FIXME: implement with dynamodb upsert method
pub async fn update_deliberation_recommendation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
    Json(req): Json<UpdateDeliberationRecommendationRequest>,
) -> Result<Json<UpdateDeliberationRecommendationResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundDeliberationSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let html_contents = req.recommendation_html_contents.unwrap_or_default();
    let files = req.recommendation_files;

    let tx_common = update_common(
        &dynamo,
        space_pk.clone(),
        req.title,
        req.visibility,
        req.started_at,
        req.ended_at,
    )
    .await?;

    let tx_recommendation =
        update_recommendation(&dynamo, space_pk.clone(), html_contents, files).await?;

    let mut tx = Vec::with_capacity(tx_common.len() + tx_recommendation.len());
    tx.extend(tx_common);
    tx.extend(tx_recommendation);

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(tx))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to update recommendation {}", e);
            crate::Error2::ServerError(e.to_string())
        })?;

    let metadata = match DeliberationMetadata::query(&dynamo.client, space_pk).await {
        Ok(v) => v,
        Err(e) => {
            tracing::debug!("deliberation metadata error: {:?}", e);
            return Err(e);
        }
    };

    let metadata: DeliberationDetailResponse = metadata.into();

    let space_common = metadata.space_common;
    let recommendations = metadata.recommendation;

    Ok(Json(UpdateDeliberationRecommendationResponse {
        space_common,
        recommendations,
    }))
}

pub async fn update_recommendation(
    dynamo: &DynamoClient,
    space_pk: Partition,

    html_contents: String,
    files: Vec<File>,
) -> Result<Vec<TransactWriteItem>, Error2> {
    let mut tx = vec![];

    let recommendation = DeliberationSpaceContent::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationRecommendation),
    )
    .await?;

    if recommendation.is_some() {
        let d =
            DeliberationSpaceContent::updater(&space_pk, EntityType::DeliberationRecommendation)
                .with_html_contents(html_contents)
                .with_files(files)
                .transact_write_item();

        tx.push(d);
    } else {
        let d = DeliberationSpaceContent::new(
            space_pk.clone(),
            EntityType::DeliberationRecommendation,
            html_contents,
            files,
        )
        .create_transact_write_item();

        tx.push(d);
    }

    Ok(tx)
}

pub async fn update_common(
    dynamo: &DynamoClient,
    space_pk: Partition,

    title: Option<String>,
    visibility: SpaceVisibility,
    started_at: i64,
    ended_at: i64,
) -> Result<Vec<TransactWriteItem>, Error2> {
    let mut tx = vec![];

    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::NotFound("Space Common not found".to_string()))?;

    let post_pk = space_common.post_pk;

    let d = SpaceCommon::updater(&space_pk, EntityType::SpaceCommon)
        .with_visibility(visibility)
        .with_started_at(started_at)
        .with_ended_at(ended_at)
        .transact_write_item();

    tx.push(d);

    let d = Post::updater(&post_pk, EntityType::Post)
        .with_title(title.unwrap_or_default())
        .transact_write_item();

    tx.push(d);

    Ok(tx)
}
