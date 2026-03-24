use crate::common::models::space::{SpaceAuthor, SpaceCommon};
use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RespondPollRequest {
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RespondPollResponse {}

#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize)]
struct PollMetadata {
    poll_sk: String,
    submitted_at_ms: i64,
}

#[cfg(feature = "server")]
fn age_to_respondent_age(age: u32) -> crate::common::attribute::Age {
    use crate::common::attribute::Age;

    match age {
        0..=17 => Age::Range {
            inclusive_min: 0,
            inclusive_max: 17,
        },
        18..=29 => Age::Range {
            inclusive_min: 18,
            inclusive_max: 29,
        },
        30..=39 => Age::Range {
            inclusive_min: 30,
            inclusive_max: 39,
        },
        40..=49 => Age::Range {
            inclusive_min: 40,
            inclusive_max: 49,
        },
        50..=59 => Age::Range {
            inclusive_min: 50,
            inclusive_max: 59,
        },
        60..=69 => Age::Range {
            inclusive_min: 60,
            inclusive_max: 69,
        },
        _ => Age::Range {
            inclusive_min: 70,
            inclusive_max: 100,
        },
    }
}

#[cfg(feature = "server")]
fn respondent_from_panel_attributes(
    attributes: &[crate::features::spaces::models::PanelAttribute],
    verified_attributes: &crate::common::models::did::VerifiedAttributes,
) -> Option<RespondentAttr> {
    use crate::features::spaces::models::{CollectiveAttribute, PanelAttribute, VerifiableAttribute};

    let mut respondent = RespondentAttr::default();

    for attribute in attributes {
        match attribute {
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
                respondent.school = verified_attributes
                    .university
                    .clone()
                    .filter(|value| !value.is_empty());
            }
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age) => {
                respondent.age = verified_attributes.age().map(age_to_respondent_age);
            }
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => {
                respondent.gender = verified_attributes.gender;
            }
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(age)) => {
                respondent.age = Some(*age);
            }
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(gender)) => {
                respondent.gender = Some(*gender);
            }
            _ => {}
        }
    }

    if respondent.is_empty() {
        None
    } else {
        Some(respondent)
    }
}

#[cfg(feature = "server")]
async fn get_respondent_from_panels(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
) -> Result<Option<RespondentAttr>> {
    let (pk, sk) = crate::common::models::did::VerifiedAttributes::keys(user_pk);
    let verified_attributes = crate::common::models::did::VerifiedAttributes::get(cli, pk, Some(sk))
        .await?
        .unwrap_or_default();
    let matched_attributes = crate::features::spaces::controllers::panel_requirements::matched_panel_attributes(
        cli,
        space_pk,
        &verified_attributes,
    )
    .await?;

    Ok(respondent_from_panel_attributes(
        &matched_attributes,
        &verified_attributes,
    ))
}

#[post("/api/spaces/{space_pk}/polls/{poll_sk}/respond", role: SpaceUserRole, author: SpaceAuthor, space: SpaceCommon)]
pub async fn respond_poll(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
    req: RespondPollRequest,
) -> Result<RespondPollResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_id = space_pk.clone();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.clone().into();

    let poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    let space_action = SpaceAction::get(
        cli,
        &CompositePartition(space_id, poll_sk.to_string()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    if !crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        space_action.prerequisite,
        space.status,
        space.join_anytime,
    ) {
        return Err(Error::BadRequest(
            "Poll is not available in the current space status".into(),
        ));
    }

    if poll.status() != PollStatus::InProgress {
        return Err(Error::BadRequest("Poll is not in progress".into()));
    }

    if !validate_answers(poll.questions.clone(), req.answers.clone()) {
        return Err(Error::BadRequest("Answers do not match questions".into()));
    }

    let existing =
        SpacePollUserAnswer::find_one(cli, &space_pk, &poll_sk_entity, &author.pk).await?;

    if existing.is_some() && !poll.response_editable {
        return Err(Error::BadRequest("Response editing not allowed".into()));
    }

    if poll.canister_upload_enabled {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        use crate::features::spaces::pages::actions::services::vote_crypto::VOTE_CRYPTO_SERVICE;
        let crypto = &*VOTE_CRYPTO_SERVICE;
        let metadata = PollMetadata {
            poll_sk: poll_sk_entity.to_string(),
            submitted_at_ms: now,
        };
        let envelope =
            crypto.encrypt(&poll_sk_entity, &author.pk, &req.answers, Some(&metadata))?;
        let selections: Vec<ratel_canister::types::QuestionSelection> = req
            .answers
            .iter()
            .enumerate()
            .flat_map(|(q_idx, answer)| {
                answer.to_option_indices().into_iter().map(move |opt_idx| {
                    ratel_canister::types::QuestionSelection {
                        question_index: q_idx as u32,
                        option_index: opt_idx,
                    }
                })
            })
            .collect();
        let ballot = ratel_canister::types::VoteBallot {
            ciphertext_hash: envelope.ciphertext_hash,
            ciphertext_blob: envelope.ciphertext_json.into_bytes(),
            submitted_at_ms: now,
            selections,
        };
        let canister = common_config.canister();
        canister
            .upsert_vote(&poll_sk_entity.to_string(), &envelope.voter_tag, ballot)
            .await?;
    }

    // DynamoDB record
    if existing.is_some() {
        let (pk, sk) = SpacePollUserAnswer::keys(&author.pk, &poll_sk_entity, &space_pk);
        let now = crate::common::utils::time::get_now_timestamp_millis();
        SpacePollUserAnswer::updater(pk, sk)
            .with_answers(req.answers)
            .with_created_at(now)
            .execute(cli)
            .await?;
    } else {
        let respondent = get_respondent_from_panels(cli, &space_pk, &author.pk).await?;
        let answer_record = SpacePollUserAnswer::new(
            space_pk.clone(),
            poll_sk_entity.clone(),
            req.answers,
            respondent,
            author,
        );
        answer_record.create(cli).await?;

        SpacePoll::updater(&space_pk, &poll_sk_entity)
            .increase_user_response_count(1)
            .execute(cli)
            .await?;

        let agg_item =
            crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_poll_responses(
                &space_pk, 1,
            );
        crate::transact_write_items!(cli, vec![agg_item]).ok();
    }

    Ok(RespondPollResponse {})
}
