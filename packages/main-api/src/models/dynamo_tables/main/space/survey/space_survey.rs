use crate::types::*;

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceSurvey {
    pub pk: Partition,
    pub sk: EntityType,

    pub status: SurveyStatus,
    pub started_at: i64,
    pub ended_at: i64,
    pub questions: Vec<SurveyQuestion>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct SpaceSurveyCreateRequest {
    pub survey_pk: Option<String>,
    pub started_at: i64,
    pub ended_at: i64,
    pub status: SurveyStatus,

    pub questions: Vec<SurveyQuestion>,
}

impl SpaceSurvey {
    pub fn new(
        pk: Partition,
        status: SurveyStatus,
        started_at: i64,
        ended_at: i64,
        questions: Vec<SurveyQuestion>,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let sk = EntityType::SpacePoll(uid);

        Self {
            pk,
            sk,
            status,
            started_at,
            ended_at,

            questions,
        }
    }
}
