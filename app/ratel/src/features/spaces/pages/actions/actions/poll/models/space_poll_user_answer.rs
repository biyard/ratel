use std::collections::HashMap;

use crate::common::attribute::*;
use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::actions::poll::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePollUserAnswer {
    pub pk: Partition, // User Partition
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

#[cfg(feature = "server")]
impl SpacePollUserAnswer {
    pub fn new(
        space_pk: Partition,
        poll_sk: EntityType,
        answers: Vec<Answer>,
        respondent: Option<RespondentAttr>,
        author: crate::common::models::space::SpaceAuthor,
    ) -> Self {
        let user_pk = author.pk;
        let created_at = get_now_timestamp_millis();
        let (pk, sk) = Self::keys(&user_pk, &poll_sk, &space_pk);
        Self {
            pk: pk.clone(),
            sk,
            created_at,
            answers,
            respondent,
            user_pk: Some(user_pk),
            display_name: Some(author.display_name),
            profile_url: Some(author.profile_url),
            username: Some(author.username),
        }
    }
    // FIXME: Because of EntityType(String, String) Type cannot deserialize from string
    // So we need to parse it manually
    /*
        This test code is failed.
        #[cfg(test)]
        mod tests {
            use super::*;
            use std::str::FromStr;

            #[test]
            fn test_dynamo_key_roundtrip() {
                let space_pk = Partition::Space("UID1".to_string());
                let poll_sk = EntityType::SpacePoll("UID2".to_string());
                let original = EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_sk.to_string());
                let serialized = original.to_string();
                let deserialized = EntityType::from_str(&serialized).unwrap();
                assert_eq!(original, deserialized);
            }
        }
    */
    pub fn parse_wrong_sk(sk: String) -> String {
        let (_, poll_sk) = sk.split_once('#').unwrap();
        poll_sk.to_string()
    }

    pub fn keys(
        user_pk: &Partition,
        poll_sk: &EntityType,
        space_pk: &Partition,
    ) -> (Partition, EntityType) {
        (
            Partition::SpacePollUserAnswer(user_pk.to_string()),
            EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_sk.to_string()),
        )
    }

    pub async fn find_one(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        poll_sk: &EntityType,
        user_pk: &Partition,
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<Option<Self>> {
        let (pk, sk) = Self::keys(user_pk, poll_sk, space_pk);
        Self::get(cli, &pk, Some(sk)).await
    }

    pub async fn summarize_responses_with_attribute(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        poll_sk: &EntityType,
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<(
        Vec<SpacePollSummary>,
        HashMap<String, Vec<SpacePollSummary>>,
        HashMap<String, Vec<SpacePollSummary>>,
        HashMap<String, Vec<SpacePollSummary>>,
        Vec<SpacePollUserAnswer>,
        Vec<SpacePollUserAnswer>,
    )> {
        let poll = SpacePoll::get(&cli, space_pk, Some(poll_sk.clone()))
            .await?
            .ok_or(Error::NotFound("Poll Not found".to_string()))?;

        let final_pk = poll_sk.clone();
        let mut sample_pk = poll_sk.clone();

        let question = poll.questions;

        let seed: Vec<SpacePollSummary> = question
            .iter()
            .cloned()
            .map(SpacePollSummary::from)
            .collect();

        use std::collections::HashMap as Map;

        let mut overall = seed.clone();
        let mut gender_map: Map<Gender, Vec<SpacePollSummary>> = Map::new();
        let mut age_map: Map<AgeBand, Vec<SpacePollSummary>> = Map::new();
        let mut school_map: Map<String, Vec<SpacePollSummary>> = Map::new();

        let mut bookmark: Option<String> = None;
        let mut final_all: Vec<SpacePollUserAnswer> = Vec::new();

        loop {
            let (responses, new_bookmark) = Self::find_by_space_pk(
                cli,
                &EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_sk.to_string()),
                if let Some(b) = &bookmark {
                    SpacePollUserAnswerQueryOption::builder().bookmark(b.clone())
                } else {
                    SpacePollUserAnswerQueryOption::builder()
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
            let mut sample_all: Vec<SpacePollUserAnswer> = Vec::new();
            let mut sb: Option<String> = None;
            loop {
                let (chunk, next) = Self::find_by_space_pk(
                    cli,
                    &EntityType::SpacePollUserAnswer(space_pk.to_string(), sample_pk.to_string()),
                    if let Some(b) = &sb {
                        SpacePollUserAnswerQueryOption::builder().bookmark(b.clone())
                    } else {
                        SpacePollUserAnswerQueryOption::builder()
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

            let mut sample_by_user: HashMap<String, SpacePollUserAnswer> = HashMap::new();
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

        let by_gender: HashMap<String, Vec<SpacePollSummary>> = gender_map
            .into_iter()
            .map(|(k, v)| {
                let key = match k {
                    Gender::Male => "male".to_string(),
                    Gender::Female => "female".to_string(),
                };
                (key, v)
            })
            .collect();

        let by_age: HashMap<String, Vec<SpacePollSummary>> = age_map
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
