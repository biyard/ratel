use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum FollowError {
    #[error("Cannot follow yourself")]
    #[translate(en = "You cannot follow yourself", ko = "자기 자신을 팔로우할 수 없습니다.")]
    CannotFollowSelf,

    #[error("Cannot unfollow yourself")]
    #[translate(en = "You cannot unfollow yourself", ko = "자기 자신을 언팔로우할 수 없습니다.")]
    CannotUnfollowSelf,

    #[error("Invalid follow target")]
    #[translate(en = "Invalid follow target", ko = "유효하지 않은 팔로우 대상입니다.")]
    InvalidTarget,
}

#[cfg(feature = "server")]
impl FollowError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        StatusCode::BAD_REQUEST
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for FollowError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for FollowError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
