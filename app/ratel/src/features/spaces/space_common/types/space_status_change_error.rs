pub use thiserror::Error;

use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceStatusChangeError {
    #[error("post not found for space")]
    #[translate(
        en = "Space post not found",
        ko = "스페이스 게시글을 찾을 수 없습니다"
    )]
    PostNotFound,
}

#[cfg(feature = "server")]
impl SpaceStatusChangeError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceStatusChangeError::PostNotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceStatusChangeError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceStatusChangeError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
