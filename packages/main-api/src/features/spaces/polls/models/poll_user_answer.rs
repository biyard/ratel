use std::collections::HashMap;

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

    pub respondent: Option<RespondentAttr>,
}
// /controllers/
// /features/features/models, utils, types
impl PollUserAnswer {
    pub fn new(
        space_pk: Partition,
        poll_pk: Partition,
        user_pk: Partition,
        answers: Vec<Answer>,
        respondent: Option<RespondentAttr>,
    ) -> Self {
        let created_at = get_now_timestamp_millis();
        let (pk, sk) = Self::keys(&user_pk, &poll_pk, &space_pk);
        Self {
            pk,
            sk,
            created_at,
            answers,
            respondent,
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

    // pub async fn summarize_responses(
    //     cli: &aws_sdk_dynamodb::Client,
    //     space_pk: &Partition,
    //     poll_pk: &Partition,
    // ) -> crate::Result<Vec<PollSummary>> {
    //     // Loop until next_bookmark is None

    //     let question =
    //         PollQuestion::get(cli, &space_pk, Some(EntityType::SpacePollQuestion)).await?;

    //     if question.is_none() {
    //         return Ok(vec![]);
    //     }

    //     let question: PollQuestion = question.unwrap_or_default();
    //     let mut summaries: Vec<PollSummary> = question
    //         .questions
    //         .into_iter()
    //         .map(PollSummary::from)
    //         .collect();

    //     let mut bookmark = None::<String>;
    //     loop {
    //         let (responses, new_bookmark) = Self::find_by_space_pk(
    //             cli,
    //             &EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_pk.to_string()),
    //             if let Some(b) = &bookmark {
    //                 PollUserAnswerQueryOption::builder().bookmark(b.clone())
    //             } else {
    //                 PollUserAnswerQueryOption::builder()
    //             },
    //         )
    //         .await?;

    //         for response in responses {
    //             for (question_idx, answer) in response.answers.into_iter().enumerate() {
    //                 summaries
    //                     .get_mut(question_idx)
    //                     .map(|summary| summary.aggregate_answer(answer));
    //             }
    //         }

    //         match new_bookmark {
    //             Some(b) => bookmark = Some(b),
    //             None => break,
    //         }
    //     }
    //     Ok(summaries)
    // }

    pub async fn summarize_responses_with_attribute(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        poll_pk: &Partition,
    ) -> crate::Result<(
        Vec<PollSummary>,
        HashMap<String, Vec<PollSummary>>,
        HashMap<String, Vec<PollSummary>>,
        HashMap<String, Vec<PollSummary>>,
    )> {
        let question =
            PollQuestion::get(cli, &space_pk, Some(EntityType::SpacePollQuestion)).await?;
        let Some(question) = question else {
            return Ok((vec![], HashMap::new(), HashMap::new(), HashMap::new()));
        };

        let seed: Vec<PollSummary> = question
            .questions
            .iter()
            .cloned()
            .map(PollSummary::from)
            .collect();

        use std::collections::HashMap as Map;
        use std::hash::Hash;

        let mut overall = seed.clone();
        let mut gender_map: Map<Gender, Vec<PollSummary>> = Map::new();
        let mut age_map: Map<AgeBand, Vec<PollSummary>> = Map::new();
        let mut school_map: Map<String, Vec<PollSummary>> = Map::new();

        let mut bookmark: Option<String> = None;

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

            for resp in responses {
                for (qi, ans) in resp.answers.iter().cloned().enumerate() {
                    if let Some(s) = overall.get_mut(qi) {
                        s.aggregate_answer(ans);
                    }
                }

                if let Some(g) = resp.respondent.as_ref().and_then(|r| r.gender.clone()) {
                    let entry = gender_map.entry(g).or_insert_with(|| seed.clone());
                    for (qi, ans) in resp.answers.iter().cloned().enumerate() {
                        if let Some(s) = entry.get_mut(qi) {
                            s.aggregate_answer(ans);
                        }
                    }
                }

                if let Some(a) = resp.respondent.as_ref().and_then(|r| r.age.clone()) {
                    let band = age_to_band(&a);
                    let entry = age_map.entry(band).or_insert_with(|| seed.clone());
                    for (qi, ans) in resp.answers.iter().cloned().enumerate() {
                        if let Some(s) = entry.get_mut(qi) {
                            s.aggregate_answer(ans);
                        }
                    }
                }

                if let Some(school) = resp.respondent.as_ref().and_then(|r| r.school.clone()) {
                    let key = if school.is_empty() {
                        "UNKNOWN".to_string()
                    } else {
                        school
                    };
                    let entry = school_map.entry(key).or_insert_with(|| seed.clone());
                    for (qi, ans) in resp.answers.into_iter().enumerate() {
                        if let Some(s) = entry.get_mut(qi) {
                            s.aggregate_answer(ans);
                        }
                    }
                }
            }

            if let Some(b) = new_bookmark {
                bookmark = Some(b);
            } else {
                break;
            }
        }

        let by_gender: HashMap<String, Vec<PollSummary>> = gender_map
            .into_iter()
            .map(|(k, v)| {
                let key = match k {
                    Gender::Male => "male",
                    Gender::Female => "female",
                }
                .to_string();
                (key, v)
            })
            .collect();

        let by_age: HashMap<String, Vec<PollSummary>> = age_map
            .into_iter()
            .map(|(band, v)| (band.label().to_string(), v))
            .collect();

        Ok((overall, by_gender, by_age, school_map))
    }
}
