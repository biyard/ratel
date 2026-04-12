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

    #[error("create failed")]
    #[translate(
        en = "Failed to create discussion",
        ko = "토론 생성에 실패했습니다."
    )]
    CreateFailed,

    #[error("delete failed")]
    #[translate(
        en = "Failed to delete discussion",
        ko = "토론 삭제에 실패했습니다."
    )]
    DeleteFailed,

    #[error("invalid discussion id")]
    #[translate(
        en = "Invalid discussion ID",
        ko = "유효하지 않은 토론 ID입니다."
    )]
    InvalidDiscussionId,

    #[error("comment too short")]
    #[translate(en = "Comment is too short", ko = "댓글이 너무 짧습니다.")]
    CommentTooShort,
}

#[cfg(feature = "server")]
impl SpaceActionDiscussionError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceActionDiscussionError::NotFound => StatusCode::NOT_FOUND,
            SpaceActionDiscussionError::CreateFailed
            | SpaceActionDiscussionError::DeleteFailed => StatusCode::INTERNAL_SERVER_ERROR,
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
