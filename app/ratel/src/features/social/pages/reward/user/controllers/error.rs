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

    #[error("No wallet connected")]
    #[translate(
        en = "Please install Kaia Wallet or MetaMask",
        ko = "Kaia Wallet 또는 MetaMask를 설치해주세요"
    )]
    NoWalletConnected,

    #[error("Claim transaction failed")]
    #[translate(
        en = "Token claim failed. Please try again.",
        ko = "토큰 클레임에 실패했습니다. 다시 시도해주세요."
    )]
    ClaimFailed,
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
