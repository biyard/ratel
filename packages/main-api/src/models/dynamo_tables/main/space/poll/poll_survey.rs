use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PollSpaceSurvey {
    pub pk: Partition,
    pub sk: EntityType,

    pub questions: Vec<SurveyQuestion>, // Questions in the survey
}

impl PollSpaceSurvey {
    pub fn new(space_pk: Partition, questions: Vec<SurveyQuestion>) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::PollSpaceSurvey,
            questions,
        }
    }
}
