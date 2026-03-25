pub use thiserror::Error;

use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum ExchangePointsError {
    #[error("No points available")]
    #[translate(
        en = "No points available to exchange",
        ko = "교환 가능한 포인트가 없습니다"
    )]
    NoPointsAvailable,

    #[error("Estimated tokens zero")]
    #[translate(
        en = "Estimated token amount is zero",
        ko = "예상 토큰 수량이 0입니다"
    )]
    EstimatedTokensZero,
}

#[cfg(feature = "server")]
impl ExchangePointsError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        StatusCode::BAD_REQUEST
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for ExchangePointsError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for ExchangePointsError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
