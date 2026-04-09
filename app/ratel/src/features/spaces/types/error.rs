pub use thiserror::Error;

use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceError {
    #[error("No required attributes")]
    #[translate(
        en = "You must verify all required attributes",
        ko = "모든 속성을 인증해야합니다."
    )]
    NoEligibleCredential,
}

#[cfg(feature = "server")]
impl SpaceError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
