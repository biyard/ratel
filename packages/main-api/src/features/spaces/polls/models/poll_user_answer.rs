use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

use crate::features::spaces::polls::{PollQuestion, PollSummary};
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PollUserAnswer {
    pub pk: Partition,
    #[dynamo(prefix = "POLL_PK", index = "gsi1", name = "find_by_space_pk", pk)]
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    pub answers: Vec<Answer>, // User responses to the survey
}
// /controllers/
// /features/features/models, utils, types
impl PollUserAnswer {
    pub fn new(
        space_pk: Partition,
        poll_pk: Partition,
        user_pk: Partition,
        answers: Vec<Answer>,
    ) -> Self {
        let created_at = get_now_timestamp_millis();
        let (pk, sk) = Self::keys(&user_pk, &poll_pk, &space_pk);
        Self {
            pk,
            sk,
            created_at,
            answers,
        }
    }
    pub fn keys(
        user_pk: &Partition,
        poll_pk: &Partition,
        space_pk: &Partition,
    ) -> (Partition, EntityType) {
        (
            Partition::SpacePollUserAnswer(user_pk.to_string()),
            EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_pk.to_string()),
        )
    }
    pub async fn find_one(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        poll_pk: &Partition,
        user_pk: &Partition,
    ) -> crate::Result<Option<Self>> {
        let (pk, sk) = Self::keys(user_pk, poll_pk, space_pk);
        Self::get(cli, &pk, Some(sk)).await
    }

    pub async fn summarize_responses(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        poll_pk: &Partition,
    ) -> crate::Result<Vec<PollSummary>> {
        // Loop until next_bookmark is None

        let question =
            PollQuestion::get(cli, &space_pk, Some(EntityType::SpacePollQuestion)).await?;

        if question.is_none() {
            return Ok(vec![]);
        }

        let question: PollQuestion = question.unwrap_or_default();
        let mut summaries: Vec<PollSummary> = question
            .questions
            .into_iter()
            .map(PollSummary::from)
            .collect();

        let mut bookmark = None::<String>;
        loop {
            let (responses, new_bookmark) = Self::find_by_space_pk(
                cli,
                &EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_pk.to_string()),
                if let Some(b) = &bookmark {
                    PollUserAnswerQueryOption::builder().bookmark(b.clone())
                } else {
                    PollUserAnswerQueryOption::builder()
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
