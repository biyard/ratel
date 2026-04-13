use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpacePollError {
    #[error("poll is not in progress")]
    #[translate(en = "Poll is not in progress", ko = "투표가 진행 중이 아닙니다.")]
    PollNotInProgress,

    #[error("answer mismatch")]
    #[translate(
        en = "Answers do not match questions",
        ko = "답변이 질문과 일치하지 않습니다."
    )]
    AnswerMismatch,

    #[error("edit not allowed")]
    #[translate(
        en = "Poll cannot be edited after responses exist",
        ko = "응답이 존재하는 투표는 수정할 수 없습니다."
    )]
    EditNotAllowed,

    #[error("questions empty")]
    #[translate(
        en = "At least one question is required",
        ko = "최소 하나의 질문이 필요합니다."
    )]
    QuestionsEmpty,

    #[error("invalid time range")]
    #[translate(
        en = "Start time must be before end time",
        ko = "시작 시간은 종료 시간보다 이전이어야 합니다."
    )]
    InvalidTimeRange,

    #[error("invalid question format")]
    #[translate(
        en = "Invalid question format",
        ko = "유효하지 않은 질문 형식입니다."
    )]
    InvalidQuestionFormat,

    #[error("vote verification failed")]
    #[translate(en = "Vote verification failed", ko = "투표 검증에 실패했습니다.")]
    VoteVerificationFailed,

    #[error("create failed")]
    #[translate(en = "Failed to create poll", ko = "투표 생성에 실패했습니다.")]
    CreateFailed,

    #[error("encryption failed")]
    #[translate(en = "Encryption failed", ko = "암호화에 실패했습니다.")]
    EncryptionFailed,

    #[error("decryption failed")]
    #[translate(en = "Decryption failed", ko = "복호화에 실패했습니다.")]
    DecryptionFailed,
}

#[cfg(feature = "server")]
impl SpacePollError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpacePollError::PollNotInProgress
            | SpacePollError::AnswerMismatch
            | SpacePollError::EditNotAllowed
            | SpacePollError::QuestionsEmpty
            | SpacePollError::InvalidTimeRange
            | SpacePollError::InvalidQuestionFormat => StatusCode::BAD_REQUEST,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpacePollError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpacePollError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
