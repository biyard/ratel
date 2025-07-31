pub use bdk::prelude::*;

use crate::*;

use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/notice-quiz-answers", table = notice_quiz_answers)]
pub struct NoticeQuizAnswer {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, one_to_one = spaces)]
    pub space_id: i64,

    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, type=JSONB, nullable)]
    #[serde(default)]
    pub notice_quiz: Vec<NoticeQuestionWithAnswer>,
}