use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum FactOrFoldError {
    // ── Headline CRUD ──────────────────────────────────────────────
    #[error("headline not found")]
    #[translate(
        en = "Headline not found",
        ko = "헤드라인을 찾을 수 없습니다.",
    )]
    HeadlineNotFound,

    #[error("headline already exists")]
    #[translate(
        en = "A headline with the same id already exists",
        ko = "동일한 ID의 헤드라인이 이미 존재합니다.",
    )]
    HeadlineAlreadyExists,

    #[error("headline is locked once a round is in progress")]
    #[translate(
        en = "This headline has a live or settled round; only verification sources may be appended",
        ko = "이미 라운드가 진행되었거나 완료된 헤드라인입니다. 검증 출처만 추가할 수 있습니다.",
    )]
    HeadlineLocked,

    #[error("headline field validation failed")]
    #[translate(
        en = "Headline field validation failed (length, range, or required field)",
        ko = "헤드라인 입력값이 올바르지 않습니다 (길이/범위/필수 항목).",
    )]
    HeadlineInvalid,

    #[error("publish-time invariant violated")]
    #[translate(
        en = "Cannot schedule a headline in the past or publish a draft that is missing required fields",
        ko = "과거 시각으로 예약하거나 필수 항목이 빠진 초안을 발행할 수 없습니다.",
    )]
    PublishInvariantViolation,

    // ── Settings ──────────────────────────────────────────────────
    #[error("settings field out of range")]
    #[translate(
        en = "A settings value is outside the allowed range",
        ko = "설정값이 허용 범위를 벗어났습니다.",
    )]
    SettingsOutOfRange,

    // ── Generic admin failures ────────────────────────────────────
    #[error("storage failure")]
    #[translate(
        en = "Storage operation failed",
        ko = "저장 작업에 실패했습니다.",
    )]
    StorageFailure,
}

#[cfg(feature = "server")]
impl FactOrFoldError {
    pub fn status_code(&self) -> crate::axum::http::StatusCode {
        use crate::axum::http::StatusCode;
        match self {
            FactOrFoldError::HeadlineNotFound => StatusCode::NOT_FOUND,
            FactOrFoldError::HeadlineAlreadyExists => StatusCode::CONFLICT,
            FactOrFoldError::HeadlineLocked
            | FactOrFoldError::HeadlineInvalid
            | FactOrFoldError::PublishInvariantViolation
            | FactOrFoldError::SettingsOutOfRange => StatusCode::BAD_REQUEST,
            FactOrFoldError::StorageFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for FactOrFoldError {
    fn into_response(self) -> crate::axum::response::Response {
        use crate::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for FactOrFoldError {
    fn as_status_code(&self) -> crate::axum::http::StatusCode {
        self.status_code()
    }
}
