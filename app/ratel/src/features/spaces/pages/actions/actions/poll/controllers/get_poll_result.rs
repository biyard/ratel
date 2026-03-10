use std::collections::HashMap;

use crate::features::spaces::pages::actions::actions::poll::*;

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

#[get("/api/spaces/{space_pk}/polls/{poll_sk}/results", role: SpaceUserRole)]
pub async fn get_poll_result(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
) -> Result<PollResultResponse> {
    SpacePoll::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

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
        created_at: crate::common::utils::time::get_now_timestamp_millis(),
        summaries,
        summaries_by_age,
        summaries_by_gender,
        summaries_by_school,
        sample_answers,
        final_answers,
    })
}
