use std::collections::HashMap;

use crate::{
    features::spaces::polls::{Poll, PollQueryOption},
    models::User,
    types::*,
    utils::time::get_now_timestamp_millis,
};
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

    pub user_pk: Option<Partition>,
    pub display_name: Option<String>,
    pub profile_url: Option<String>,
    pub username: Option<String>,
}
// /controllers/
// /features/features/models, utils, types
impl PollUserAnswer {
    pub fn new(
        space_pk: Partition,
        poll_pk: Partition,
        answers: Vec<Answer>,
        respondent: Option<RespondentAttr>,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let user_pk = pk;
        let created_at = get_now_timestamp_millis();
        let (pk, sk) = Self::keys(&user_pk, &poll_pk, &space_pk);
        Self {
            pk: pk.clone(),
            sk,
            created_at,
            answers,
            respondent,
            user_pk: Some(user_pk),
            display_name: Some(display_name),
            profile_url: Some(profile_url),
            username: Some(username),
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

    pub async fn summarize_responses_with_attribute(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        poll_pk: &Partition,
    ) -> crate::Result<(
        Vec<PollSummary>,
        HashMap<String, Vec<PollSummary>>,
        HashMap<String, Vec<PollSummary>>,
        HashMap<String, Vec<PollSummary>>,
        Vec<PollUserAnswer>,
        Vec<PollUserAnswer>,
    )> {
        let polls = Poll::query(
            &cli,
            space_pk,
            PollQueryOption::builder().sk("SPACE_POLL#".to_string()),
        )
        .await?;
        let final_pk = poll_pk.clone();
        let mut sample_pk = poll_pk.clone();

        for poll in polls.0 {
            let id = match poll.sk {
                EntityType::SpacePoll(id) => id,
                _ => "".to_string(),
            };
            if poll.response_editable {
                sample_pk = Partition::Poll(id.clone());
                break;
            }
        }

        let question =
            PollQuestion::get(cli, &space_pk, Some(EntityType::SpacePollQuestion)).await?;
        let Some(question) = question else {
            return Ok((
                vec![],
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                vec![],
                vec![],
            ));
        };

        let seed: Vec<PollSummary> = question
            .questions
            .iter()
            .cloned()
            .map(PollSummary::from)
            .collect();

        use std::collections::HashMap as Map;

        let mut overall = seed.clone();
        let mut gender_map: Map<Gender, Vec<PollSummary>> = Map::new();
        let mut age_map: Map<AgeBand, Vec<PollSummary>> = Map::new();
        let mut school_map: Map<String, Vec<PollSummary>> = Map::new();

        let mut bookmark: Option<String> = None;
        let mut final_all: Vec<PollUserAnswer> = Vec::new();

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

            for resp in &responses {
                final_all.push(resp.clone());
            }

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

        let (sample_aligned, final_aligned) = if sample_pk == final_pk {
            (final_all, vec![])
        } else {
            let mut sample_all: Vec<PollUserAnswer> = Vec::new();
            let mut sb: Option<String> = None;
            loop {
                let (chunk, next) = Self::find_by_space_pk(
                    cli,
                    &EntityType::SpacePollUserAnswer(space_pk.to_string(), sample_pk.to_string()),
                    if let Some(b) = &sb {
                        PollUserAnswerQueryOption::builder().bookmark(b.clone())
                    } else {
                        PollUserAnswerQueryOption::builder()
                    },
                )
                .await?;
                sample_all.extend(chunk);
                if let Some(b) = next {
                    sb = Some(b);
                } else {
                    break;
                }
            }

            let mut sample_by_user: HashMap<String, PollUserAnswer> = HashMap::new();
            for s in sample_all {
                if let Partition::SpacePollUserAnswer(user) = &s.pk {
                    sample_by_user.insert(user.clone(), s);
                }
            }

            let mut sample_out = Vec::new();
            let mut final_out = Vec::new();
            for f in final_all {
                if let Partition::SpacePollUserAnswer(user) = &f.pk {
                    if let Some(s) = sample_by_user.remove(user) {
                        sample_out.push(s);
                        final_out.push(f);
                    }
                }
            }
            (sample_out, final_out)
        };

        let by_gender: HashMap<String, Vec<PollSummary>> = gender_map
            .into_iter()
            .map(|(k, v)| {
                let key = match k {
                    Gender::Male => "male".to_string(),
                    Gender::Female => "female".to_string(),
                };
                (key, v)
            })
            .collect();

        let by_age: HashMap<String, Vec<PollSummary>> = age_map
            .into_iter()
            .map(|(band, v)| (band.label().to_string(), v))
            .collect();

        Ok((
            overall,
            by_gender,
            by_age,
            school_map,
            sample_aligned,
            final_aligned,
        ))
    }
}
