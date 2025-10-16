use crate::models::{
    DeliberationPath, DeliberationSpaceQuestion, DeliberationSpaceQuestionQueryOption,
    DeliberationSpaceSurvey, DeliberationSurveyResponse, Post, SpaceCommon, SurveyCreateRequest,
};
use crate::types::{Partition, SpaceVisibility, TeamGroupPermission};
use crate::utils::aws::DynamoClient;
use crate::{
    AppState, Error2,
    models::{
        space::{DeliberationDetailResponse, DeliberationMetadata},
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
pub struct UpdateDeliberationPollRequest {
    #[schemars(description = "Deliberation surveys")]
    pub surveys: Vec<SurveyCreateRequest>,

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
pub struct UpdateDeliberationPollResponse {
    #[serde(flatten)]
    pub space_common: SpaceCommon,
    pub surveys: DeliberationSurveyResponse,
}

pub async fn update_deliberation_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
    Json(req): Json<UpdateDeliberationPollRequest>,
) -> Result<Json<UpdateDeliberationPollResponse>, Error2> {
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
        req.visibility,
        req.started_at,
        req.ended_at,
    )
    .await?;

    let tx_survey = update_survey(&dynamo, space_pk.clone(), req.surveys).await?;

    let mut tx = Vec::with_capacity(tx_common.len() + tx_survey.len());
    tx.extend(tx_common);
    tx.extend(tx_survey);

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(tx))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to update poll {}", e);
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

    let surveys = metadata.surveys;
    let space_common = metadata.space_common;

    Ok(Json(UpdateDeliberationPollResponse {
        space_common,
        surveys,
    }))
}

pub async fn update_survey(
    dynamo: &DynamoClient,
    space_pk: Partition,
    surveys: Vec<SurveyCreateRequest>,
) -> Result<Vec<TransactWriteItem>, Error2> {
    let mut tx = vec![];

    for survey in surveys {
        if survey.survey_pk.is_some() {
            let survey_id = survey
                .survey_pk
                .clone()
                .unwrap_or_default()
                .split("#")
                .last()
                .ok_or_else(|| Error2::BadRequest("Invalid survey_pk format".into()))?
                .to_string();

            let d = DeliberationSpaceSurvey::updater(
                &space_pk,
                EntityType::DeliberationSurvey(survey_id.clone()),
            )
            .with_started_at(survey.started_at)
            .with_ended_at(survey.ended_at)
            .with_status(survey.status)
            .transact_write_item();

            tx.push(d);

            let option = DeliberationSpaceQuestionQueryOption::builder();

            let deleted_questions = DeliberationSpaceQuestion::find_by_survey_pk(
                &dynamo.client,
                survey.survey_pk.unwrap(),
                option,
            )
            .await?
            .0;

            for question in deleted_questions {
                let d = DeliberationSpaceQuestion::delete(
                    &dynamo.client,
                    question.pk,
                    Some(question.sk),
                )
                .await?
                .create_transact_write_item();

                tx.push(d);
            }

            let d = DeliberationSpaceQuestion::new(
                space_pk.clone(),
                Partition::Survey(survey_id.clone()),
                survey.questions,
            )
            .create_transact_write_item();

            tx.push(d);
        } else {
            let sur = DeliberationSpaceSurvey::new(
                space_pk.clone(),
                survey.status,
                survey.started_at,
                survey.ended_at,
            );

            sur.create(&dynamo.client).await?;

            let survey_id = match sur.clone().sk {
                EntityType::DeliberationSurvey(v) => v,
                _ => "".to_string(),
            };

            let d = DeliberationSpaceQuestion::new(
                space_pk.clone(),
                Partition::Survey(survey_id),
                survey.questions,
            )
            .create_transact_write_item();

            tx.push(d);
        }
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
