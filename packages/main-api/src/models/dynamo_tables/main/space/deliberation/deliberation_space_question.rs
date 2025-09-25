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
    pub question: String,
}

impl DeliberationSpaceQuestion {
    pub fn new(pk: Partition, survey_pk: Partition, question: SurveyQuestion) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let sk = EntityType::DeliberationSpaceQuestion(uid);
        let question = Self::serialize_question(&question);
        Self {
            pk,
            sk,
            survey_pk,
            question,
        }
    }

    pub fn question(&self) -> SurveyQuestion {
        serde_json::from_str(&self.question).unwrap_or_default()
    }

    pub fn set_question(&mut self, q: SurveyQuestion) {
        self.question = Self::serialize_question(&q);
    }

    pub fn try_question(&self) -> Result<SurveyQuestion, serde_json::Error> {
        serde_json::from_str(&self.question)
    }

    #[inline]
    fn serialize_question(q: &SurveyQuestion) -> String {
        serde_json::to_string(q).unwrap_or_else(|_| "{}".to_string())
    }
}
