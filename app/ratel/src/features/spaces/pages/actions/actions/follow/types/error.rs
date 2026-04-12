use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceFollowError {
    #[error("cannot follow self")]
    #[translate(
        en = "You cannot follow yourself",
        ko = "자기 자신을 팔로우할 수 없습니다."
    )]
    CannotFollowSelf,

    #[error("invalid target")]
    #[translate(en = "Invalid follow target", ko = "유효하지 않은 팔로우 대상입니다.")]
    InvalidTarget,

    #[error("invalid follow target")]
    #[translate(
        en = "Invalid follow target user",
        ko = "유효하지 않은 팔로우 대상 사용자입니다."
    )]
    InvalidFollowTarget,

    #[error("creator cannot be removed")]
    #[translate(
        en = "The creator cannot be removed",
        ko = "생성자는 제거할 수 없습니다."
    )]
    CreatorCannotBeRemoved,

    #[error("follow failed")]
    #[translate(en = "Failed to follow user", ko = "팔로우에 실패했습니다.")]
    FollowFailed,

    #[error("unfollow failed")]
    #[translate(en = "Failed to unfollow user", ko = "언팔로우에 실패했습니다.")]
    UnfollowFailed,

    #[error("create failed")]
    #[translate(
        en = "Failed to create follow action",
        ko = "팔로우 액션 생성에 실패했습니다."
    )]
    CreateFailed,
}

#[cfg(feature = "server")]
impl SpaceFollowError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceFollowError::CannotFollowSelf
            | SpaceFollowError::InvalidTarget
            | SpaceFollowError::InvalidFollowTarget
            | SpaceFollowError::CreatorCannotBeRemoved => StatusCode::BAD_REQUEST,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceFollowError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceFollowError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
