use crate::{Error2, models::PollSpaceSurvey, types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PollSpaceSurveyResponse {
    pub pk: Partition,
    #[dynamo(
        prefix = "POLL_SPACE_PK",
        index = "gsi1",
        name = "find_by_space_pk",
        pk
    )]
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    pub answers: Vec<SurveyAnswer>, // User responses to the survey
}

impl PollSpaceSurveyResponse {
    pub fn new(space_pk: Partition, user_pk: Partition, answers: Vec<SurveyAnswer>) -> Self {
        let created_at = get_now_timestamp_millis();

        Self {
            pk: Partition::PollSpaceResponse(user_pk.to_string()),
            sk: EntityType::PollSpaceSurveyResponse(space_pk.to_string()),
            created_at,
            answers,
        }
    }

    pub async fn summarize_responses(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> Result<Vec<SurveySummary>, Error2> {
        // Loop until next_bookmark is None

        let survey = PollSpaceSurvey::get(cli, &space_pk, Some(EntityType::PollSpaceSurvey))
            .await?
            .ok_or(Error2::NotFoundPollSpace)?;
        let mut summaries: Vec<SurveySummary> = survey
            .questions
            .into_iter()
            .map(SurveySummary::from)
            .collect();

        let mut bookmark = None::<String>;
        loop {
            let (responses, new_bookmark) = Self::find_by_space_pk(
                cli,
                &EntityType::PollSpaceSurveyResponse(space_pk.to_string()),
                if let Some(b) = &bookmark {
                    PollSpaceSurveyResponseQueryOption::builder().bookmark(b.clone())
                } else {
                    PollSpaceSurveyResponseQueryOption::builder()
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
