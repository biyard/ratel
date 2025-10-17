use crate::{models::PollMetadata, types::*};
use bdk::prelude::*;

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct PollResponse {
    pub sk: EntityType,
    pub created_at: i64,
    pub updated_at: i64,
    pub started_at: i64,
    pub ended_at: i64,
    pub response_editable: bool, // Whether users can edit their responses
    pub user_response_count: i64, // Participants count
    pub questions: Vec<Question>, // Questions in the survey
    pub my_response: Option<Vec<Answer>>, // User responses to the survey
}

impl From<Vec<PollMetadata>> for PollResponse {
    fn from(entity: Vec<PollMetadata>) -> Self {
        let mut res = Self::default();
        for entry in entity {
            match entry {
                PollMetadata::Poll(poll) => {
                    res.sk = poll.sk;
                    res.started_at = poll.started_at;
                    res.ended_at = poll.ended_at;
                    res.response_editable = poll.response_editable;
                    res.user_response_count = poll.user_response_count;
                    res.created_at = poll.created_at;
                    res.updated_at = poll.updated_at;
                }
                PollMetadata::PollQuestion(question) => {
                    res.questions = question.questions;
                }
            }
        }
        res
    }
}
