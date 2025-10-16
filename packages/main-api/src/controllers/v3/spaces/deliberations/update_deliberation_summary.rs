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

use aide::NoApi;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateDeliberationSummaryRequest {
    #[schemars(description = "Deliberation Title")]
    pub title: Option<String>,
    #[schemars(description = "Deliberation HTML contents")]
    pub html_contents: Option<String>,
    #[schemars(description = "Deliberation summary files")]
    pub files: Vec<File>,

    #[schemars(description = "Deliberation visibility")]
    pub visibility: SpaceVisibility,
    #[schemars(description = "Deliberation start date")]
    pub started_at: i64,
    #[schemars(description = "Deliberation end date")]
    pub ended_at: i64,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateDeliberationSummaryResponse {
    #[serde(flatten)]
    pub space_common: SpaceCommon,
    pub summaries: DeliberationContentResponse,
}

//FIXME: implement with dynamodb upsert method
pub async fn update_deliberation_summary_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
    Json(req): Json<UpdateDeliberationSummaryRequest>,
) -> Result<Json<UpdateDeliberationSummaryResponse>, Error2> {
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

    let tx_common = update_common(
        &dynamo,
        space_pk.clone(),
        req.title,
        req.html_contents.clone(),
        req.visibility,
        req.started_at,
        req.ended_at,
    )
    .await?;

    let tx_summary =
        update_summary(&dynamo, space_pk.clone(), req.html_contents, req.files).await?;

    let mut tx = Vec::with_capacity(tx_common.len() + tx_summary.len());
    tx.extend(tx_common);
    tx.extend(tx_summary);

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(tx))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to update summary {}", e);
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

    let summaries = metadata.summary;
    let space_common = metadata.space_common;

    Ok(Json(UpdateDeliberationSummaryResponse {
        space_common,
        summaries,
    }))
}

pub async fn update_summary(
    dynamo: &DynamoClient,
    space_pk: Partition,

    html_contents: Option<String>,
    files: Vec<File>,
) -> Result<Vec<TransactWriteItem>, Error2> {
    let mut tx = vec![];

    let deliberation_summary = DeliberationSpaceContent::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationSummary),
    )
    .await?;

    if deliberation_summary.is_some() {
        let d = DeliberationSpaceContent::updater(&space_pk, EntityType::DeliberationSummary)
            .with_html_contents(html_contents.unwrap_or_default())
            .with_files(files)
            .transact_write_item();

        tx.push(d);
    } else {
        let d = DeliberationSpaceContent::new(
            space_pk.clone(),
            EntityType::DeliberationSummary,
            html_contents.unwrap_or_default(),
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
    html_contents: Option<String>,
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
        .with_html_contents(html_contents.unwrap_or_default())
        .transact_write_item();

    tx.push(d);

    Ok(tx)
}
