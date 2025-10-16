use crate::{
    models::space::{PollSpaceMetadata, SpaceCommon},
    types::*,
};
use bdk::prelude::*;

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

// #[derive(
//     Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo,
// )]
// pub struct PollSpaceSurveyAnswerDto {
//     pub answers: Vec<SurveyAnswer>,
// }

// impl From<PollSpaceSurveyResponse> for PollSpaceSurveyAnswerDto {
//     fn from(entity: PollSpaceSurveyResponse) -> Self {
//         Self {
//             answers: entity.answers,
//         }
//     }
// }

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PollSpaceSurveySummary {
    pub created_at: i64,
    pub summaries: Vec<SurveySummary>,
}
