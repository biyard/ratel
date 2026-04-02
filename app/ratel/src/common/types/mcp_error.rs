pub use thiserror::Error;

use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum McpServerError {
    #[error("Unpublish is not supported")]
    #[translate(en = "Unpublish is not currently supported", ko = "게시 취소는 현재 지원되지 않습니다.")]
    UnpublishNotSupported,

    #[error("Start is not available for the current status")]
    #[translate(en = "Start is not available for the current space status", ko = "현재 스페이스 상태에서는 시작할 수 없습니다.")]
    StartNotAvailable,

    #[error("Cannot undo start")]
    #[translate(en = "Cannot undo start", ko = "시작을 취소할 수 없습니다.")]
    CannotUndoStart,

    #[error("Finish is not available for the current status")]
    #[translate(en = "Finish is not available for the current space status", ko = "현재 스페이스 상태에서는 완료할 수 없습니다.")]
    FinishNotAvailable,

    #[error("Cannot undo finish")]
    #[translate(en = "Cannot undo finish", ko = "완료를 취소할 수 없습니다.")]
    CannotUndoFinish,

    #[error("Invalid panel quota")]
    #[translate(en = "The specified quota is invalid (would result in negative remaining slots)", ko = "지정한 정원이 유효하지 않습니다 (잔여 인원이 음수가 됩니다).")]
    InvalidPanelQuota,

    #[error("Invalid time range")]
    #[translate(en = "Start time must be before end time", ko = "시작 시간은 종료 시간보다 이전이어야 합니다.")]
    InvalidTimeRange,

    #[error("Questions cannot be empty")]
    #[translate(en = "At least one question is required", ko = "최소 하나의 질문이 필요합니다.")]
    EmptyQuestions,

    #[error("Invalid update data")]
    #[translate(en = "Invalid update data format", ko = "잘못된 업데이트 데이터 형식입니다.")]
    InvalidUpdateData,

    #[error("Required field missing")]
    #[translate(en = "A required field is missing", ko = "필수 항목이 누락되었습니다.")]
    RequiredFieldMissing,

    #[error("Deprecated operation")]
    #[translate(en = "This operation is deprecated", ko = "이 작업은 더 이상 지원되지 않습니다.")]
    DeprecatedOperation,
}

#[cfg(feature = "server")]
impl McpServerError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        StatusCode::BAD_REQUEST
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for McpServerError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for McpServerError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
