//! arcade-level umbrella error. Covers wallet, realtime channel, and
//! stage scheduler concerns. Game-specific errors live next to each
//! game (e.g. `games::fact_or_fold::types::FactOrFoldError`).

use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum ArcadeError {
    // ── Wallet (chip) ─────────────────────────────────────────────
    #[error("insufficient chip balance")]
    #[translate(
        en = "Not enough chips for this action",
        ko = "칩이 부족합니다."
    )]
    WalletInsufficientChip,

    #[error("chip → RP redeem is disabled in v1")]
    #[translate(
        en = "Cashing out chips will be enabled in a future release",
        ko = "현금화는 추후 업데이트에서 지원됩니다."
    )]
    WalletRedeemDisabled,

    #[error("wallet amount out of allowed range")]
    #[translate(
        en = "Amount is outside the allowed range",
        ko = "금액이 허용 범위를 벗어났습니다."
    )]
    WalletAmountOutOfRange,

    #[error("insufficient RP for chip conversion")]
    #[translate(
        en = "Not enough RP to convert",
        ko = "환전에 필요한 RP가 부족합니다."
    )]
    WalletInsufficientRp,

    // ── Realtime channel ──────────────────────────────────────────
    #[error("channel not registered")]
    #[translate(
        en = "Unknown channel",
        ko = "알 수 없는 채널입니다."
    )]
    ChannelUnknown,

    #[error("not allowed to subscribe to this channel")]
    #[translate(
        en = "You are not allowed to access this channel",
        ko = "이 채널을 구독할 권한이 없습니다."
    )]
    ChannelForbidden,

    #[error("channel payload invalid")]
    #[translate(
        en = "Channel payload is invalid",
        ko = "채널 페이로드가 올바르지 않습니다."
    )]
    ChannelPayloadInvalid,

    // ── Stage scheduler ───────────────────────────────────────────
    #[error("no next stage from current state")]
    #[translate(
        en = "Game has reached its terminal stage",
        ko = "게임이 마지막 단계에 도달했습니다."
    )]
    SchedulerTerminalStage,

    // ── Generic ───────────────────────────────────────────────────
    #[error("arcade storage failure")]
    #[translate(
        en = "Storage operation failed",
        ko = "저장 작업에 실패했습니다."
    )]
    StorageFailure,
}

#[cfg(feature = "server")]
impl ArcadeError {
    pub fn status_code(&self) -> crate::axum::http::StatusCode {
        use crate::axum::http::StatusCode;
        match self {
            ArcadeError::WalletInsufficientChip
            | ArcadeError::WalletInsufficientRp
            | ArcadeError::WalletAmountOutOfRange
            | ArcadeError::ChannelPayloadInvalid
            | ArcadeError::SchedulerTerminalStage => StatusCode::BAD_REQUEST,
            ArcadeError::WalletRedeemDisabled => StatusCode::FORBIDDEN,
            ArcadeError::ChannelUnknown => StatusCode::NOT_FOUND,
            ArcadeError::ChannelForbidden => StatusCode::FORBIDDEN,
            ArcadeError::StorageFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for ArcadeError {
    fn into_response(self) -> crate::axum::response::Response {
        use crate::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for ArcadeError {
    fn as_status_code(&self) -> crate::axum::http::StatusCode {
        self.status_code()
    }
}
