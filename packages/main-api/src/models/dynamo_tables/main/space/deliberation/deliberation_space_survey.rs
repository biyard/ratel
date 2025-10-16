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
    /* FIXME
        Why we need to store questions in another entity?
        if we store questions here, Because questions cannot be updated per question.
        Always questions are updated as a whole.
    */
    // pub questions: Vec<SurveyQuestion>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct SurveyCreateRequest {
    pub survey_pk: Option<String>,
    pub started_at: i64,
    pub ended_at: i64,
    pub status: SurveyStatus,

    pub questions: Vec<SurveyQuestion>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct DeliberationSurveyResponse {
    pub pk: Partition,

    pub started_at: i64,
    pub ended_at: i64,
    pub status: SurveyStatus,

    pub questions: Vec<SurveyQuestion>,

    //FIXME: responses should be paginated.
    // We Cannot return all responses at once. because dynamoDB has a limit of 1MB per response.
    // So, we should remove SurveyResponseResponse from
    pub responses: Vec<SurveyResponseResponse>,
    //FIXME: `user_response`` should be only one response per user.
    // So, we need to change the name to `user_response` and make it optional.
    // pub user_response: Option<SurveyResponseResponse>,
    pub user_responses: Vec<SurveyResponseResponse>,
}

impl From<DeliberationSpaceSurvey> for DeliberationSurveyResponse {
    fn from(survey: DeliberationSpaceSurvey) -> Self {
        let pk = match survey.sk {
            EntityType::DeliberationSurvey(v) => v,
            _ => "".to_string(),
        };
        Self {
            pk: Partition::Survey(pk.to_string()),
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
        let sk = EntityType::DeliberationSurvey(uid);

        Self {
            pk,
            sk,
            status,
            started_at,
            ended_at,
        }
    }
}
