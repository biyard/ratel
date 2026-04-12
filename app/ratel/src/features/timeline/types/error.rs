use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum TimelineError {
    #[error("fan out failed")]
    #[translate(en = "Timeline update failed", ko = "타임라인 업데이트에 실패했습니다.")]
    FanOutFailed,

    #[error("invalid user")]
    #[translate(en = "Invalid user", ko = "유효하지 않은 사용자입니다.")]
    InvalidUser,

    #[error("invalid bookmark")]
    #[translate(en = "Invalid bookmark", ko = "유효하지 않은 북마크입니다.")]
    InvalidBookmark,
}

#[cfg(feature = "server")]
impl TimelineError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            TimelineError::FanOutFailed => StatusCode::INTERNAL_SERVER_ERROR,
            TimelineError::InvalidUser | TimelineError::InvalidBookmark => {
                StatusCode::BAD_REQUEST
            }
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for TimelineError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for TimelineError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
