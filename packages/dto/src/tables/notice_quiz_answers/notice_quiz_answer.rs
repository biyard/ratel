pub use bdk::prelude::*;

use std::collections::{HashMap, HashSet};

use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/notice-quiz-answers", table = notice_quiz_answers, action = [create_answer(space_id = i64, answers = NoticeAnswer)], action_by_id = [update_answer(answers = NoticeAnswer)])]
pub struct NoticeQuizAnswer {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, one_to_one = spaces)]
    pub space_id: i64,

    #[api_model(summary, type=JSONB, nullable)]
    #[serde(default)]
    pub answers: NoticeAnswer,
}


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Validate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct NoticeAnswer {
    pub answers: HashMap<String, HashSet<String>>, // question_id -> HashSet<option_id>
}

impl Default for NoticeAnswer {
    fn default() -> Self {
        NoticeAnswer {
            answers: HashMap::new(),
        }
    }
}