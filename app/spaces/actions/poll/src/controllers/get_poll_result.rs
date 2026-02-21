use std::collections::HashMap;

use crate::*;
use ratel_auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PollResultResponse {
    pub created_at: i64,
    pub summaries: Vec<SpacePollSummary>,
    pub summaries_by_gender: HashMap<String, Vec<SpacePollSummary>>,
    pub summaries_by_age: HashMap<String, Vec<SpacePollSummary>>,
    pub summaries_by_school: HashMap<String, Vec<SpacePollSummary>>,
    pub sample_answers: Vec<SpacePollUserAnswer>,
    pub final_answers: Vec<SpacePollUserAnswer>,
}

#[get("/api/polls/{space_pk}/{poll_sk}/results", user: User)]
pub async fn get_poll_result(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
) -> Result<PollResultResponse> {
    let cli = crate::config::get().common.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let (
        summaries,
        summaries_by_gender,
        summaries_by_age,
        summaries_by_school,
        sample_answers,
        final_answers,
    ) = SpacePollUserAnswer::summarize_responses_with_attribute(cli, &space_pk, &poll_sk_entity)
        .await?;

    Ok(PollResultResponse {
        created_at: common::utils::time::get_now_timestamp_millis(),
        summaries,
        summaries_by_age,
        summaries_by_gender,
        summaries_by_school,
        sample_answers,
        final_answers,
    })
}
