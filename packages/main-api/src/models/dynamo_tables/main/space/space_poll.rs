use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpacePoll {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "SURVEY_PK", name = "find_by_survey_pk", index = "gsi1", pk)]
    pub survey_pk: Partition,

    pub questions: Vec<SurveyQuestion>,
}

impl SpacePoll {
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
pub struct SpacePollResponse {
    pub survey_pk: Partition,
    pub questions: Vec<SurveyQuestion>,
}

impl From<SpacePoll> for SpacePollResponse {
    fn from(poll: SpacePoll) -> Self {
        Self {
            survey_pk: poll.clone().survey_pk,
            questions: poll.clone().question(),
        }
    }
}
