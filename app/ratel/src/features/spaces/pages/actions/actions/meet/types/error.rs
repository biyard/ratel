use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone, PartialEq)]
pub enum MeetActionError {
    #[error("create meet failed")]
    #[translate(en = "Could not create the meet", ko = "회의를 생성할 수 없습니다")]
    CreateFailed,

    #[error("update meet failed")]
    #[translate(en = "Could not save changes", ko = "변경 사항을 저장할 수 없습니다")]
    UpdateFailed,

    #[error("meet not found")]
    #[translate(en = "Meet not found", ko = "회의를 찾을 수 없습니다")]
    NotFound,

    #[error("invalid duration {0}")]
    #[translate(
        en = "Duration must be between 15 and 1440 minutes",
        ko = "지속 시간은 15~1440분 사이여야 합니다"
    )]
    InvalidDuration(i32),

    #[error("delete meet failed")]
    #[translate(en = "Could not delete the meet", ko = "회의를 삭제할 수 없습니다")]
    DeleteFailed,
}

#[cfg(feature = "server")]
impl MeetActionError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            MeetActionError::NotFound => StatusCode::NOT_FOUND,
            MeetActionError::InvalidDuration(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for MeetActionError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for MeetActionError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
