use crate::{Error, types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

use super::{super::PollSummary, PollQuestion};
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PollUserResponse {
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

    pub answers: Vec<Answer>, // User responses to the survey
}
// /controllers/
// /features/features/models, utils, types
impl PollUserResponse {
    pub fn new(space_pk: Partition, user_pk: Partition, answers: Vec<Answer>) -> Self {
        let created_at = get_now_timestamp_millis();

        Self {
            pk: Partition::SpacePollUserResponse(user_pk.to_string()),
            sk: EntityType::SpacePollUserResponse(space_pk.to_string()),
            created_at,
            answers,
        }
    }
    pub async fn find_one(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        user_pk: &Partition,
    ) -> crate::Result<Option<Self>> {
        Self::get(
            cli,
            &Partition::SpacePollUserResponse(user_pk.to_string()),
            Some(EntityType::SpacePollUserResponse(space_pk.to_string())),
        )
        .await
    }

    pub async fn summarize_responses(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> crate::Result<Vec<PollSummary>> {
        // Loop until next_bookmark is None

        let question: PollQuestion =
            PollQuestion::get(cli, &space_pk, Some(EntityType::SpacePollQuestion))
                .await?
                .ok_or(Error::NotFoundPoll)?;
        let mut summaries: Vec<PollSummary> = question
            .questions
            .into_iter()
            .map(PollSummary::from)
            .collect();

        let mut bookmark = None::<String>;
        loop {
            let (responses, new_bookmark) = Self::find_by_space_pk(
                cli,
                &EntityType::SpacePollUserResponse(space_pk.to_string()),
                if let Some(b) = &bookmark {
                    PollUserResponseQueryOption::builder().bookmark(b.clone())
                } else {
                    PollUserResponseQueryOption::builder()
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
        Ok(summaries)
    }
}
