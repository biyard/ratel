use crate::models::space::SurveyResponseResponse;
use crate::types::*;

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceSurvey {
    pub pk: Partition,
    pub sk: EntityType,

    pub status: SurveyStatus,
    pub started_at: i64,
    pub ended_at: i64,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct SurveyCreateRequest {
    pub id: Option<String>,
    pub started_at: i64,
    pub ended_at: i64,
    pub status: SurveyStatus,

    pub questions: Vec<SurveyQuestion>,
}

#[derive(Debug, Clone, Default, serde::Serialize, JsonSchema)]
pub struct DeliberationSurveyResponse {
    pub pk: String,

    pub started_at: i64,
    pub ended_at: i64,
    pub status: SurveyStatus,

    pub questions: Vec<SurveyQuestion>,
    pub responses: Vec<SurveyResponseResponse>,
    pub user_responses: Vec<SurveyResponseResponse>,
}

impl From<DeliberationSpaceSurvey> for DeliberationSurveyResponse {
    fn from(survey: DeliberationSpaceSurvey) -> Self {
        let pk = match survey.sk {
            EntityType::DeliberationSpaceSurvey(v) => v,
            _ => "".to_string(),
        };
        Self {
            pk,
            started_at: survey.started_at,
            ended_at: survey.ended_at,
            status: survey.status,
            questions: vec![],
            responses: vec![],
            user_responses: vec![],
        }
    }
}

impl DeliberationSpaceSurvey {
    pub fn new(pk: Partition, status: SurveyStatus, started_at: i64, ended_at: i64) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let sk = EntityType::DeliberationSpaceSurvey(uid);

        Self {
            pk,
            sk,
            status,
            started_at,
            ended_at,
        }
    }
}
