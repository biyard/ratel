use crate::{Error2, models::SpaceSurvey, types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, schemars::JsonSchema,
)]
pub struct SpaceSurveyAnswer {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(prefix = "SPACE_PK", index = "gsi2", name = "find_by_space_pk", pk)]
    pub sk: EntityType,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub survey_pk: Partition,

    pub survey_type: SurveyType,
    pub answers: Vec<SurveyAnswer>,

    #[dynamo(index = "gsi2", sk)]
    pub created_at: i64,
}

impl SpaceSurveyAnswer {
    pub fn new(
        space_pk: Partition,
        user_pk: Partition,
        survey_pk: Partition,
        survey_type: SurveyType,
        answers: Vec<SurveyAnswer>,
    ) -> Self {
        let created_at = get_now_timestamp_millis();

        let sk = EntityType::SpaceSurveyResponse(space_pk.to_string());

        Self {
            pk: Partition::SurveyResponse(user_pk.to_string()),
            sk,
            user_pk,
            survey_pk,
            survey_type,
            created_at,
            answers,
        }
    }

    pub async fn summarize_responses(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        survey_sk: &EntityType,
    ) -> Result<Vec<SurveySummary>, Error2> {
        let survey_id = match survey_sk {
            EntityType::SpacePoll(v) => v.to_string(),
            _ => "".to_string(),
        };

        // Loop until next_bookmark is None
        let survey = SpaceSurvey::get(cli, &space_pk, Some(EntityType::SpacePoll(survey_id)))
            .await?
            .ok_or(Error2::NotFoundSpace)?;
        let mut summaries: Vec<SurveySummary> = survey
            .questions
            .into_iter()
            .map(SurveySummary::from)
            .collect();

        let mut bookmark = None::<String>;
        loop {
            let (responses, new_bookmark) = Self::find_by_space_pk(
                cli,
                &EntityType::SpaceSurveyResponse(space_pk.to_string()),
                if let Some(b) = &bookmark {
                    SpaceSurveyAnswerQueryOption::builder().bookmark(b.clone())
                } else {
                    SpaceSurveyAnswerQueryOption::builder()
                },
            )
            .await?;

            for response in responses {
                for (question_idx, answer) in response.answers.into_iter().enumerate() {
                    summaries
                        .get_mut(question_idx)
                        .map(|summary| summary.aggregate_answer(answer));
                }
            }

            match new_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }
        // PollSpaceSurveyResult::new(space_pk.clone(), summaries.clone())
        //     .create(cli)
        //     .await?;
        Ok(summaries)
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct SpaceSurveyAnswerResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub user_pk: Partition,
    pub survey_type: SurveyType,
    pub answers: Vec<SurveyAnswer>,

    pub created_at: i64,
}

impl From<SpaceSurveyAnswer> for SpaceSurveyAnswerResponse {
    fn from(responses: SpaceSurveyAnswer) -> Self {
        Self {
            pk: responses.clone().pk,
            sk: responses.clone().sk,
            user_pk: responses.clone().user_pk,
            survey_type: responses.clone().survey_type,
            answers: responses.answers,
            created_at: responses.created_at,
        }
    }
}
