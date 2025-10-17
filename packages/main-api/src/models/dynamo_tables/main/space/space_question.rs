use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceSurveyQuestion {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "SURVEY_PK", name = "find_by_survey_pk", index = "gsi1", pk)]
    pub survey_pk: Partition,

    pub questions: Vec<SurveyQuestion>,
}

impl SpaceSurveyQuestion {
    pub fn new(pk: Partition, survey_pk: Partition, questions: Vec<SurveyQuestion>) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let sk = EntityType::Question(uid);
        Self {
            pk,
            sk,
            survey_pk,
            questions,
        }
    }

    pub fn question(&self) -> Vec<SurveyQuestion> {
        self.questions.clone()
    }
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct SpaceSurveyQuestionResponse {
    pub survey_pk: Partition,
    pub questions: Vec<SurveyQuestion>,
}

impl From<SpaceSurveyQuestion> for SpaceSurveyQuestionResponse {
    fn from(question: SpaceSurveyQuestion) -> Self {
        Self {
            survey_pk: question.clone().survey_pk,
            questions: question.clone().question(),
        }
    }
}
