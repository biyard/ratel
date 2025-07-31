use bdk::prelude::*;
use validator::Validate;

use crate::*;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/notice-quiz-attempts", table = notice_quiz_attempts, action = [submit_answers(answers = Vec<NoticeQuestion>)])]
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
    pub user_answers: Vec<NoticeQuestionWithAnswer>,
    
    #[api_model(summary)]
    pub is_successful: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct NoticeQuestionWithAnswer {
    pub title: String,
    #[validate(custom(function = "validate_image_files"))]
    pub images: Vec<File>,
    pub options: Vec<NoticeOptionWithAnswer>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct NoticeOptionWithAnswer {
    pub content: String,
    pub is_correct: bool, // Indicates if the option is correct
}
