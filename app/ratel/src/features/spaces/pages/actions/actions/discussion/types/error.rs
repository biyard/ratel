pub use thiserror::Error;

use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceActionDiscussionError {
    #[error("Discussion is not available in the current space status")]
    #[translate(
        en = "Discussion is not available in the current space status",
        ko = "현재 스페이스 상태에서는 토론을 하실 수 없습니다."
    )]
    NotAvailableInCurrentStatus,

    #[error("Discussion not found")]
    #[translate(en = "Discussion not found", ko = "토론을 찾을 수 없습니다.")]
    NotFound,
}

#[cfg(feature = "server")]
impl SpaceActionDiscussionError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceActionDiscussionError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceActionDiscussionError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceActionDiscussionError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
