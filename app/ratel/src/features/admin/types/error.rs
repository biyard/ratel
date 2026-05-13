use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum AdminError {
    #[error("username required")]
    #[translate(en = "Username is required", ko = "사용자 이름이 필요합니다.")]
    UsernameRequired,

    #[error("invalid bookmark")]
    #[translate(en = "Invalid bookmark", ko = "유효하지 않은 북마크입니다.")]
    InvalidBookmark,
}

#[cfg(feature = "server")]
impl AdminError {
    pub fn status_code(&self) -> crate::axum::http::StatusCode {
        use crate::axum::http::StatusCode;
        StatusCode::BAD_REQUEST
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for AdminError {
    fn into_response(self) -> crate::axum::response::Response {
        use crate::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for AdminError {
    fn as_status_code(&self) -> crate::axum::http::StatusCode {
        self.status_code()
    }
}
