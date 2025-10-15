use crate::types::File;
use crate::{
    AppState, Error2,
    models::{
        feed::Post,
        space::{
            DeliberationDetailResponse, DeliberationDiscussionMember,
            DeliberationDiscussionMemberQueryOption, DeliberationMetadata,
            DeliberationSpaceContent, DeliberationSpaceDiscussion, DeliberationSpaceElearning,
            DeliberationSpaceQuestion, DeliberationSpaceQuestionQueryOption,
            DeliberationSpaceSurvey, DiscussionCreateRequest, SpaceCommon, SurveyCreateRequest,
        },
        user::User,
    },
    types::{EntityType, Partition, SpaceVisibility, TeamGroupPermission},
    utils::aws::DynamoClient,
};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::Deserialize;
use validator::Validate;

use aide::NoApi;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct UpdateDeliberationRequest {
    #[schemars(description = "Deliberation title")]
    pub title: Option<String>,
    #[schemars(description = "Deliberation HTML contents")]
    pub html_contents: Option<String>,
    #[schemars(description = "Deliberation summary files")]
    pub files: Vec<File>,

    #[schemars(description = "Discussion informations")]
    pub discussions: Vec<DiscussionCreateRequest>,
    #[schemars(description = "Deliberation elearning files")]
    pub elearning_files: Vec<File>,

    #[schemars(description = "Deliberation surveys")]
    pub surveys: Vec<SurveyCreateRequest>,

    #[schemars(description = "Final Recommendation HTML contents")]
    pub recommendation_html_contents: Option<String>,
    #[schemars(description = "Final Recommendation files")]
    pub recommendation_files: Vec<File>,

    #[schemars(description = "Deliberation visibility")]
    pub visibility: SpaceVisibility,
    #[schemars(description = "Deliberation start date")]
    pub started_at: i64,
    #[schemars(description = "Deliberation end date")]
    pub ended_at: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationPath {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

pub async fn update_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
    Json(req): Json<UpdateDeliberationRequest>,
) -> Result<Json<DeliberationDetailResponse>, Error2> {
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

    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::NotFound("Space Common not found".to_string()))?;

    let post_pk = space_common.post_pk;

    SpaceCommon::updater(&space_pk, EntityType::SpaceCommon)
        .with_visibility(req.visibility)
        .with_started_at(req.started_at)
        .with_ended_at(req.ended_at)
        .execute(&dynamo.client)
        .await?;

    Post::updater(&post_pk, EntityType::Post)
        .with_title(req.title.clone().unwrap_or_default())
        .with_html_contents(req.html_contents.clone().unwrap_or_default())
        .execute(&dynamo.client)
        .await?;

    update_summary(
        dynamo.clone(),
        space_pk.to_string(),
        req.html_contents,
        req.files,
    )
    .await?;
    update_discussion(
        dynamo.clone(),
        user.clone().unwrap_or_default(),
        space_pk.to_string(),
        req.discussions,
    )
    .await?;
    update_elearning(dynamo.clone(), space_pk.to_string(), req.elearning_files).await?;
    update_survey(dynamo.clone(), space_pk.to_string(), req.surveys).await?;
    update_recommendation(
        dynamo.clone(),
        space_pk.to_string(),
        req.recommendation_html_contents.unwrap_or_default(),
        req.recommendation_files,
    )
    .await?;

    // tracing::debug!("hello!!!: {:?}", space_pk.to_string());

    let metadata = match DeliberationMetadata::query(&dynamo.client, space_pk).await {
        Ok(v) => v,
        Err(e) => {
            tracing::debug!("deliberation metadata error: {:?}", e);
            return Err(e);
        }
    };

    // tracing::debug!("deliberation metadata: {:?}", metadata);

    let metadata: DeliberationDetailResponse = metadata.into();

    // tracing::debug!("deliberation metadata 1111: {:?}", metadata);

    Ok(Json(metadata))
}

pub async fn update_survey(
    dynamo: DynamoClient,
    space_pk: String,
    surveys: Vec<SurveyCreateRequest>,
) -> Result<(), Error2> {
    let id = space_pk
        .clone()
        .split("#")
        .last()
        .ok_or_else(|| Error2::BadRequest("Invalid space_pk format".into()))?
        .to_string();

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

            DeliberationSpaceSurvey::updater(
                &space_pk,
                EntityType::DeliberationSurvey(survey_id.clone()),
            )
            .with_started_at(survey.started_at)
            .with_ended_at(survey.ended_at)
            .with_status(survey.status)
            .execute(&dynamo.client)
            .await?;

            let option = DeliberationSpaceQuestionQueryOption::builder();

            let deleted_questions = DeliberationSpaceQuestion::find_by_survey_pk(
                &dynamo.client,
                survey.survey_pk.unwrap(),
                option,
            )
            .await?
            .0;

            for question in deleted_questions {
                DeliberationSpaceQuestion::delete(&dynamo.client, question.pk, Some(question.sk))
                    .await?;
            }

            let question = DeliberationSpaceQuestion::new(
                Partition::Space(id.clone()),
                Partition::Survey(survey_id.clone()),
                survey.questions,
            );

            question.create(&dynamo.client).await?;
        } else {
            let sur = DeliberationSpaceSurvey::new(
                Partition::Space(id.clone()),
                survey.status,
                survey.started_at,
                survey.ended_at,
            );

            sur.create(&dynamo.client).await?;

            let survey_id = match sur.clone().sk {
                EntityType::DeliberationSurvey(v) => v,
                _ => "".to_string(),
            };

            let question = DeliberationSpaceQuestion::new(
                Partition::Space(id.clone()),
                Partition::Survey(survey_id),
                survey.questions,
            );

            question.create(&dynamo.client).await?;
        }
    }

    Ok(())
}

pub async fn update_discussion(
    dynamo: DynamoClient,
    user: User,
    space_pk: String,
    discussions: Vec<DiscussionCreateRequest>,
) -> Result<(), Error2> {
    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;

    for data in metadata.into_iter() {
        match data {
            // DeliberationMetadata::DeliberationSpaceParticipant(v) => {
            //     DeliberationSpaceParticipant::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            // }
            DeliberationMetadata::DeliberationSpaceMember(v) => {
                DeliberationDiscussionMember::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceDiscussion(v) => {
                DeliberationSpaceDiscussion::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            _ => {}
        }
    }

    let id = space_pk
        .clone()
        .split("#")
        .last()
        .ok_or_else(|| Error2::BadRequest("Invalid space_pk format".into()))?
        .to_string();

    for discussion in discussions {
        if discussion.discussion_pk.is_some() {
            let discussion_id = discussion
                .discussion_pk
                .clone()
                .unwrap()
                .split("#")
                .last()
                .ok_or_else(|| Error2::BadRequest("Invalid discussion_pk format".into()))?
                .to_string();
            DeliberationSpaceDiscussion::updater(
                &space_pk.clone(),
                EntityType::DeliberationDiscussion(discussion_id.to_string()),
            )
            .with_started_at(discussion.started_at)
            .with_ended_at(discussion.ended_at)
            .with_name(discussion.name)
            .with_description(discussion.description)
            .execute(&dynamo.client)
            .await?;

            let option = DeliberationDiscussionMemberQueryOption::builder();

            let deleted_members = DeliberationDiscussionMember::find_by_discussion_pk(
                &dynamo.client,
                discussion.discussion_pk.unwrap(),
                option,
            )
            .await?
            .0;

            for member in deleted_members {
                DeliberationDiscussionMember::delete(&dynamo.client, member.pk, Some(member.sk))
                    .await?;
            }

            for member in discussion.user_ids {
                let user = User::get(&dynamo.client, member, Some(EntityType::User))
                    .await?
                    .ok_or(Error2::NotFound("User not found".into()))?;

                let m = DeliberationDiscussionMember::new(
                    Partition::Space(id.to_string()),
                    Partition::Discussion(discussion_id.to_string()),
                    user,
                );

                m.create(&dynamo.client).await?;
            }
        } else {
            let disc = DeliberationSpaceDiscussion::new(
                Partition::Space(id.clone()),
                discussion.name,
                discussion.description,
                discussion.started_at,
                discussion.ended_at,
                None,
                "".to_string(),
                None,
                None,
                user.clone(),
            );

            let disc_id = match disc.clone().sk {
                EntityType::DeliberationDiscussion(v) => v,
                _ => "".to_string(),
            };

            disc.create(&dynamo.client).await?;

            for member in discussion.user_ids {
                let user = User::get(&dynamo.client, member, Some(EntityType::User))
                    .await?
                    .ok_or(Error2::NotFound("User not found".into()))?;

                let m = DeliberationDiscussionMember::new(
                    Partition::Space(id.to_string()),
                    Partition::Discussion(disc_id.clone()),
                    user,
                );

                m.create(&dynamo.client).await?;
            }
        }
    }

    Ok(())
}

pub async fn update_recommendation(
    dynamo: DynamoClient,
    space_pk: String,
    html_contents: String,
    files: Vec<File>,
) -> Result<(), Error2> {
    let recommendation = DeliberationSpaceContent::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationRecommendation),
    )
    .await?;

    if recommendation.is_some() {
        DeliberationSpaceContent::updater(&space_pk, EntityType::DeliberationRecommendation)
            .with_html_contents(html_contents)
            .with_files(files)
            .execute(&dynamo.client)
            .await?;
    } else {
        let id = space_pk
            .clone()
            .split("#")
            .last()
            .ok_or_else(|| Error2::BadRequest("Invalid space_pk format".into()))?
            .to_string();
        let recommendation = DeliberationSpaceContent::new(
            Partition::Space(id.to_string()),
            EntityType::DeliberationRecommendation,
            html_contents,
            files,
        );
        recommendation.create(&dynamo.client).await?;
    }

    Ok(())
}

pub async fn update_elearning(
    dynamo: DynamoClient,
    space_pk: String,
    elearning_files: Vec<File>,
) -> Result<(), Error2> {
    let elearning = DeliberationSpaceElearning::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationElearning),
    )
    .await?;

    if elearning.is_some() {
        DeliberationSpaceElearning::updater(&space_pk, EntityType::DeliberationElearning)
            .with_files(elearning_files)
            .execute(&dynamo.client)
            .await?;
    } else {
        let pk = space_pk.split("#").last().unwrap_or_default().to_string();
        let elearning =
            DeliberationSpaceElearning::new(Partition::Space(pk.to_string()), elearning_files);
        elearning.create(&dynamo.client).await?;
    }

    Ok(())
}

pub async fn update_summary(
    dynamo: DynamoClient,
    space_pk: String,
    html_contents: Option<String>,
    files: Vec<File>,
) -> Result<(), Error2> {
    let deliberation_summary = DeliberationSpaceContent::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationSummary),
    )
    .await?;

    if deliberation_summary.is_some() {
        DeliberationSpaceContent::updater(&space_pk, EntityType::DeliberationSummary)
            .with_html_contents(html_contents.unwrap_or_default())
            .with_files(files)
            .execute(&dynamo.client)
            .await?;
    } else {
        let id = space_pk
            .clone()
            .split("#")
            .last()
            .ok_or_else(|| Error2::BadRequest("Invalid space_pk format".into()))?
            .to_string();
        let summary = DeliberationSpaceContent::new(
            Partition::Space(id),
            EntityType::DeliberationSummary,
            html_contents.unwrap_or_default(),
            files,
        );
        summary.create(&dynamo.client).await?;
    }

    Ok(())
}
