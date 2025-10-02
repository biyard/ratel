use super::*;
use crate::models::space::SpaceCommon;
use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PollSpaceMetadata {
    SpaceCommon(SpaceCommon),
    PollSpace(PollSpace),
    PollSpaceSurvey(PollSpaceSurvey),
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct PollSpaceResponse {
    #[serde(flatten)]
    pub common: SpaceCommon,

    pub user_response_count: i64,               // Participants count
    pub questions: Vec<SurveyQuestion>,         // Questions in the survey
    pub my_response: Option<Vec<SurveyAnswer>>, // User responses to the survey
}

impl From<Vec<PollSpaceMetadata>> for PollSpaceResponse {
    fn from(entity: Vec<PollSpaceMetadata>) -> Self {
        let mut res = Self::default();
        for entry in entity {
            match entry {
                PollSpaceMetadata::SpaceCommon(common) => res.common = common,
                PollSpaceMetadata::PollSpace(poll) => {
                    res.user_response_count = poll.user_response_count
                }
                PollSpaceMetadata::PollSpaceSurvey(survey) => {
                    res.questions = survey.questions;
                }
            }
        }
        res
    }
}
