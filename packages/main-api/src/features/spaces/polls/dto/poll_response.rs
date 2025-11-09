use bdk::prelude::*;

use crate::types::{Answer, EntityType, Question};

use crate::features::spaces::polls::{Poll, PollStatus};
#[derive(Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
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
    pub status: PollStatus,
    pub default: bool,
}

impl From<Poll> for PollResponse {
    fn from(poll: Poll) -> Self {
        let mut res = Self::default();
        res.sk = poll.clone().sk;
        res.started_at = poll.started_at;
        res.ended_at = poll.ended_at;
        res.response_editable = poll.response_editable;
        res.user_response_count = poll.user_response_count;
        res.created_at = poll.created_at;
        res.updated_at = poll.updated_at;
        res.questions = poll.clone().questions;
        res.status = poll.status();
        res.default = poll.is_default_poll();

        res
    }
}
