use crate::{
    AppState, Error2,
    models::{
        space::{
            DeliberationDetailResponse, DeliberationMetadata, DeliberationSpace,
            DeliberationSpaceDiscussion, DeliberationSpaceElearning, DeliberationSpaceMember,
            DeliberationSpaceMemberQueryOption, DeliberationSpaceQuestion,
            DeliberationSpaceQuestionQueryOption, DeliberationSpaceRecommendation,
            DeliberationSpaceSummary, DeliberationSpaceSurvey, DiscussionCreateRequest,
            SurveyCreateRequest,
        },
        user::User,
    },
    types::{EntityType, Partition},
    utils::{aws::DynamoClient, dynamo_extractor::extract_user},
};
use dto::File;
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;
use validator::Validate;

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
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationPath {
    pub id: String,
}

pub async fn update_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(DeliberationPath { id }): Path<DeliberationPath>,
    Json(req): Json<UpdateDeliberationRequest>,
) -> Result<Json<DeliberationDetailResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;
    let _space = DeliberationSpace::get(
        &dynamo.client,
        &Partition::DeliberationSpace(id.to_string()),
        Some(EntityType::Space),
    )
    .await?
    .ok_or(Error2::NotFound(format!(
        "Space not found: (space ID: {:?})",
        id
    )))?;

    update_summary(dynamo.clone(), id.clone(), req.html_contents, req.files).await?;
    update_discussion(dynamo.clone(), user.clone(), id.clone(), req.discussions).await?;
    update_elearning(dynamo.clone(), id.clone(), req.elearning_files).await?;
    update_survey(dynamo.clone(), id.clone(), req.surveys).await?;
    update_recommendation(
        dynamo.clone(),
        id.clone(),
        req.recommendation_html_contents.unwrap_or_default(),
        req.recommendation_files,
    )
    .await?;

    let metadata =
        DeliberationMetadata::query(&dynamo.client, Partition::DeliberationSpace(id.to_string()))
            .await?;

    let metadata: DeliberationDetailResponse = metadata.into();

    Ok(Json(metadata))
}

pub async fn update_survey(
    dynamo: DynamoClient,
    id: String,
    surveys: Vec<SurveyCreateRequest>,
) -> Result<(), Error2> {
    for survey in surveys {
        if survey.id.is_some() {
            DeliberationSpaceSurvey::updater(
                &Partition::DeliberationSpace(id.to_string()),
                EntityType::DeliberationSpaceSurvey(survey.id.clone().unwrap_or_default().clone()),
            )
            .with_started_at(survey.started_at)
            .with_ended_at(survey.ended_at)
            .with_status(survey.status)
            .execute(&dynamo.client)
            .await?;

            let option = DeliberationSpaceQuestionQueryOption::builder();

            let deleted_questions = DeliberationSpaceQuestion::find_by_survey_pk(
                &dynamo.client,
                Partition::Survey(survey.id.clone().unwrap_or_default().clone()),
                option,
            )
            .await?
            .0;

            for question in deleted_questions {
                DeliberationSpaceQuestion::delete(&dynamo.client, question.pk, Some(question.sk))
                    .await?;
            }

            let question = DeliberationSpaceQuestion::new(
                Partition::DeliberationSpace(id.clone()),
                Partition::Survey(survey.id.clone().unwrap_or_default().clone()),
                survey.questions,
            );

            question.create(&dynamo.client).await?;
        } else {
            let sur = DeliberationSpaceSurvey::new(
                Partition::DeliberationSpace(id.clone()),
                survey.status,
                survey.started_at,
                survey.ended_at,
            );

            sur.create(&dynamo.client).await?;

            let survey_id = match sur.clone().sk {
                EntityType::DeliberationSpaceSurvey(v) => v,
                _ => "".to_string(),
            };

            let question = DeliberationSpaceQuestion::new(
                Partition::DeliberationSpace(id.clone()),
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
    id: String,
    discussions: Vec<DiscussionCreateRequest>,
) -> Result<(), Error2> {
    for discussion in discussions {
        if discussion.id.is_some() {
            DeliberationSpaceDiscussion::updater(
                &Partition::DeliberationSpace(id.to_string()),
                EntityType::DeliberationSpaceDiscussion(
                    discussion.id.clone().unwrap_or_default().clone(),
                ),
            )
            .with_started_at(discussion.started_at)
            .with_ended_at(discussion.ended_at)
            .with_name(discussion.name)
            .with_description(discussion.description)
            .execute(&dynamo.client)
            .await?;

            let option = DeliberationSpaceMemberQueryOption::builder();

            let deleted_members = DeliberationSpaceMember::find_by_discussion_pk(
                &dynamo.client,
                Partition::Discussion(discussion.id.clone().unwrap_or_default()),
                option,
            )
            .await?
            .0;

            for member in deleted_members {
                DeliberationSpaceMember::delete(&dynamo.client, member.pk, Some(member.sk)).await?;
            }

            for member in discussion.user_ids {
                let user = User::get(
                    &dynamo.client,
                    Partition::User(member),
                    Some(EntityType::User),
                )
                .await?
                .ok_or(Error2::NotFound("User not found".into()))?;

                let m = DeliberationSpaceMember::new(
                    Partition::DeliberationSpace(id.to_string()),
                    Partition::Discussion(discussion.id.clone().unwrap_or_default()),
                    user,
                );

                m.create(&dynamo.client).await?;
            }
        } else {
            let disc = DeliberationSpaceDiscussion::new(
                Partition::DeliberationSpace(id.clone()),
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
                EntityType::DeliberationSpaceDiscussion(v) => v,
                _ => "".to_string(),
            };

            disc.create(&dynamo.client).await?;

            for member in discussion.user_ids {
                let user = User::get(
                    &dynamo.client,
                    Partition::User(member),
                    Some(EntityType::User),
                )
                .await?
                .ok_or(Error2::NotFound("User not found".into()))?;

                let m = DeliberationSpaceMember::new(
                    Partition::DeliberationSpace(id.to_string()),
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
    id: String,
    html_contents: String,
    files: Vec<File>,
) -> Result<(), Error2> {
    let recommendation = DeliberationSpaceRecommendation::get(
        &dynamo.client,
        &Partition::DeliberationSpace(id.to_string()),
        Some(EntityType::DeliberationSpaceRecommendation),
    )
    .await?;

    if recommendation.is_some() {
        DeliberationSpaceRecommendation::updater(
            &Partition::DeliberationSpace(id.to_string()),
            EntityType::DeliberationSpaceRecommendation,
        )
        .with_html_contents(html_contents)
        .with_file(set_files(files))
        .execute(&dynamo.client)
        .await?;
    } else {
        let recommendation = DeliberationSpaceRecommendation::new(
            Partition::DeliberationSpace(id.to_string()),
            html_contents,
            files,
        );
        recommendation.create(&dynamo.client).await?;
    }

    Ok(())
}

pub async fn update_elearning(
    dynamo: DynamoClient,
    id: String,
    elearning_files: Vec<File>,
) -> Result<(), Error2> {
    let elearning = DeliberationSpaceElearning::get(
        &dynamo.client,
        &Partition::DeliberationSpace(id.to_string()),
        Some(EntityType::DeliberationSpaceElearning),
    )
    .await?;

    if elearning.is_some() {
        DeliberationSpaceElearning::updater(
            &Partition::DeliberationSpace(id.to_string()),
            EntityType::DeliberationSpaceElearning,
        )
        .with_file(set_files(elearning_files))
        .execute(&dynamo.client)
        .await?;
    } else {
        let elearning = DeliberationSpaceElearning::new(
            Partition::DeliberationSpace(id.to_string()),
            elearning_files,
        );
        elearning.create(&dynamo.client).await?;
    }

    Ok(())
}

pub async fn update_summary(
    dynamo: DynamoClient,
    id: String,
    html_contents: Option<String>,
    files: Vec<File>,
) -> Result<(), Error2> {
    let deliberation_summary = DeliberationSpaceSummary::get(
        &dynamo.client,
        &Partition::DeliberationSpace(id.to_string()),
        Some(EntityType::DeliberationSpaceSummary),
    )
    .await?;

    if deliberation_summary.is_some() {
        DeliberationSpaceSummary::updater(
            &Partition::DeliberationSpace(id.to_string()),
            EntityType::DeliberationSpaceSummary,
        )
        .with_html_contents(html_contents.unwrap_or_default())
        .with_file(set_files(files))
        .execute(&dynamo.client)
        .await?;
    } else {
        let summary = DeliberationSpaceSummary::new(
            Partition::DeliberationSpace(id.to_string()),
            html_contents.unwrap_or_default(),
            files,
        );
        summary.create(&dynamo.client).await?;
    }

    Ok(())
}

pub fn set_files(files: Vec<File>) -> String {
    serde_json::to_string(&files).unwrap_or_else(|_| "[]".to_string())
}
