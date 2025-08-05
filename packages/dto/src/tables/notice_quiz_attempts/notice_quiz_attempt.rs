use bdk::prelude::*;
use validator::Validate;

use crate::*;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/notice-quiz-attempts", table = notice_quiz_attempts, action = [submit_answers(user_id = i64, answers = NoticeAnswer, is_successful = bool)])]
pub struct NoticeQuizAttempt {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,

    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, type = JSONB)]
    pub answers: NoticeAnswer,
    
    #[api_model(summary)]
    pub is_successful: bool,
}
