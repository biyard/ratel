use crate::common::models::space::{SpaceAuthor, SpaceCommon};
use crate::features::spaces::pages::actions::actions::poll::*;

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

#[post("/api/spaces/{space_pk}/polls/{poll_sk}/respond", role: SpaceUserRole, author: SpaceAuthor, space: SpaceCommon)]
pub async fn respond_poll(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
    req: RespondPollRequest,
) -> Result<RespondPollResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();
    if !space.is_active() {
        return Err(Error::BadRequest("Space is not active".into()));
    }

    let poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    poll.can_respond(&role)?;

    if !validate_answers(poll.questions.clone(), req.answers.clone()) {
        return Err(Error::BadRequest("Answers do not match questions".into()));
    }

    let existing =
        SpacePollUserAnswer::find_one(cli, &space_pk, &poll_sk_entity, &author.pk).await?;

    if existing.is_some() && !poll.response_editable {
        return Err(Error::BadRequest("Response editing not allowed".into()));
    }

    let env = crate::common::config::Environment::default();
    if poll.canister_upload_enabled && env != crate::common::config::Environment::Production {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        use crate::features::spaces::pages::actions::services::vote_crypto::VOTE_CRYPTO_SERVICE;
        let crypto = VOTE_CRYPTO_SERVICE
            .as_ref()
            .ok_or(Error::InternalServerError(
                "Encrypted voting is not configured (VOTER_TAG_SECRET / ATTR_VOTING_AUTHORITY_JSON missing)".into(),
            ))?;
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
        let answer_record = SpacePollUserAnswer::new(
            space_pk.clone(),
            poll_sk_entity.clone(),
            req.answers,
            None,
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
