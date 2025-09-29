use crate::types::*;
use bdk::prelude::*;
use serde_json;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceQuestion {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "SURVEY_PK", name = "find_by_survey_pk", index = "gsi1", pk)]
    pub survey_pk: Partition,

    // INFO: Serialize multiple question vectors and save them in String format
    pub questions: String,
}

impl DeliberationSpaceQuestion {
    pub fn new(pk: Partition, survey_pk: Partition, questions: Vec<SurveyQuestion>) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let sk = EntityType::DeliberationSpaceQuestion(uid);
        let questions = Self::serialize_question(&questions);
        Self {
            pk,
            sk,
            survey_pk,
            questions,
        }
    }

    pub fn question(&self) -> Vec<SurveyQuestion> {
        serde_json::from_str(&self.questions).unwrap_or_default()
    }

    pub fn set_question(&mut self, q: Vec<SurveyQuestion>) {
        self.questions = Self::serialize_question(&q);
    }

    pub fn try_question(&self) -> Result<Vec<SurveyQuestion>, serde_json::Error> {
        serde_json::from_str(&self.questions)
    }

    #[inline]
    fn serialize_question(q: &Vec<SurveyQuestion>) -> String {
        serde_json::to_string(q).unwrap_or_else(|_| "{}".to_string())
    }
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct QuestionResponse {
    pub questions: Vec<SurveyQuestion>,
}

impl From<DeliberationSpaceQuestion> for QuestionResponse {
    fn from(questions: DeliberationSpaceQuestion) -> Self {
        Self {
            questions: questions.question(),
        }
    }
}
