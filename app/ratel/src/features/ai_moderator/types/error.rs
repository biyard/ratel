use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum AiModeratorError {
    #[error("AI moderator config not found")]
    #[translate(en = "AI moderator configuration not found", ko = "AI 중재자 설정을 찾을 수 없습니다")]
    ConfigNotFound,

    #[error("Invalid reply interval")]
    #[translate(en = "Reply interval must be at least 1", ko = "답변 간격은 1 이상이어야 합니다")]
    InvalidReplyInterval,

    #[error("Premium feature required")]
    #[translate(en = "AI Moderator requires a paid membership", ko = "AI 중재자는 유료 멤버십이 필요합니다")]
    PremiumRequired,

    #[error("Material limit reached")]
    #[translate(en = "Maximum number of reference materials reached (10)", ko = "참고 자료 최대 개수(10)에 도달했습니다")]
    MaterialLimitReached,

    #[error("Material not found")]
    #[translate(en = "Reference material not found", ko = "참고 자료를 찾을 수 없습니다")]
    MaterialNotFound,
}

#[cfg(feature = "server")]
impl AiModeratorError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            AiModeratorError::ConfigNotFound | AiModeratorError::MaterialNotFound => {
                StatusCode::NOT_FOUND
            }
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for AiModeratorError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for AiModeratorError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
