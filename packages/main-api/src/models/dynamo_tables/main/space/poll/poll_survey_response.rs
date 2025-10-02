use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PollSpaceSurveyResponse {
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

    pub answers: Vec<SurveyAnswer>, // User responses to the survey
}

impl PollSpaceSurveyResponse {
    pub fn new(space_pk: Partition, user_pk: Partition, answers: Vec<SurveyAnswer>) -> Self {
        let created_at = get_now_timestamp_millis();

        Self {
            pk: Partition::PollSpaceResponse(user_pk.to_string()),
            sk: EntityType::PollSpaceSurveyResponse(space_pk.to_string()),
            created_at,
            answers,
        }
    }
}
