use crate::common::models::space::{SpaceAuthor, SpaceCommon};
use crate::features::spaces::pages::actions::actions::poll::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RespondPollRequest {
    pub answers: Vec<Answer>,
}

#[post("/api/spaces/{space_pk}/polls/{poll_sk}/respond", role: SpaceUserRole, author: SpaceAuthor, space: SpaceCommon)]
pub async fn respond_poll(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
    req: RespondPollRequest,
) -> Result<String> {
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

    if existing.is_none() {
        let answer = SpacePollUserAnswer::new(
            space_pk.clone(),
            poll_sk_entity.clone(),
            req.answers,
            None,
            author,
        );
        answer.create(cli).await?;

        SpacePoll::updater(&space_pk, &poll_sk_entity)
            .increase_user_response_count(1)
            .execute(cli)
            .await?;

        let agg_item =
            crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_poll_responses(
                &space_pk, 1,
            );
        crate::transact_write_items!(cli, vec![agg_item]).ok();
    } else if poll.response_editable {
        let (pk, sk) = SpacePollUserAnswer::keys(&author.pk, &poll_sk_entity, &space_pk);
        let now = crate::common::utils::time::get_now_timestamp_millis();
        SpacePollUserAnswer::updater(pk, sk)
            .with_answers(req.answers)
            .with_created_at(now)
            .execute(cli)
            .await?;
    } else {
        return Err(Error::BadRequest("Response editing not allowed".into()));
    }

    Ok("success".to_string())
}
