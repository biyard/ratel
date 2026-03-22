pub use thiserror::Error;

use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceActionQuizError {
    #[error("No remaining attempts")]
    #[translate(en = "No remaining attempts", ko = "남은 시도 횟수가 없습니다.")]
    NoRemainingAttempts,

    #[error("Quiz is not available in the current space status")]
    #[translate(
        en = "Quiz is not available in the current space status",
        ko = "현재 스페이스 상태에서는 퀴즈를 사용할 수 없습니다."
    )]
    NotAvailableInCurrentStatus,

    #[error("Quiz is not in progress")]
    #[translate(en = "Quiz is not in progress", ko = "퀴즈가 진행 중이 아닙니다.")]
    NotInProgress,

    #[error("Answers do not match questions")]
    #[translate(
        en = "Answers do not match questions",
        ko = "답변이 질문과 일치하지 않습니다."
    )]
    AnswersMismatch,

    #[error("Quiz cannot be edited after responses exist")]
    #[translate(
        en = "Quiz cannot be edited after responses exist",
        ko = "응답이 존재하는 퀴즈는 수정할 수 없습니다."
    )]
    CannotEditAfterResponses,

    #[error("started_at is required")]
    #[translate(en = "Start time is required", ko = "시작 시간이 필요합니다.")]
    StartedAtRequired,

    #[error("ended_at is required")]
    #[translate(en = "End time is required", ko = "종료 시간이 필요합니다.")]
    EndedAtRequired,

    #[error("Invalid time range")]
    #[translate(en = "Invalid time range", ko = "유효하지 않은 시간 범위입니다.")]
    InvalidTimeRange,

    #[error("Retry count must be >= 0")]
    #[translate(
        en = "Retry count must be zero or greater",
        ko = "재시도 횟수는 0 이상이어야 합니다."
    )]
    InvalidRetryCount,

    #[error("Retry count exceeds maximum allowed value")]
    #[translate(
        en = "Retry count exceeds the maximum allowed value",
        ko = "재시도 횟수가 최대 허용 값을 초과합니다."
    )]
    RetryCountExceedsMax,

    #[error("Questions cannot be empty")]
    #[translate(en = "Questions cannot be empty", ko = "질문은 비어있을 수 없습니다.")]
    EmptyQuestions,

    #[error("Quiz only supports choice questions")]
    #[translate(
        en = "Quiz only supports choice questions",
        ko = "퀴즈는 선택형 질문만 지원합니다."
    )]
    UnsupportedQuestionType,

    #[error("Pass score must be >= 0")]
    #[translate(
        en = "Pass score must be zero or greater",
        ko = "합격 점수는 0 이상이어야 합니다."
    )]
    InvalidPassScore,

    #[error("Answers length mismatch")]
    #[translate(
        en = "Number of answers does not match number of questions",
        ko = "답변 수가 질문 수와 일치하지 않습니다."
    )]
    AnswersLengthMismatch,

    #[error("Invalid single answer index")]
    #[translate(
        en = "Invalid answer index for single choice question",
        ko = "단일 선택 질문의 답변 인덱스가 유효하지 않습니다."
    )]
    InvalidSingleAnswerIndex,

    #[error("Invalid multiple answer index")]
    #[translate(
        en = "Invalid answer index for multiple choice question",
        ko = "다중 선택 질문의 답변 인덱스가 유효하지 않습니다."
    )]
    InvalidMultipleAnswerIndex,

    #[error("Answer type does not match question")]
    #[translate(
        en = "Answer type does not match question type",
        ko = "답변 유형이 질문 유형과 일치하지 않습니다."
    )]
    AnswerTypeMismatch,
}

#[cfg(feature = "server")]
impl SpaceActionQuizError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceActionQuizError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceActionQuizError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
