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

    #[error("email verification failed")]
    #[translate(en = "Email verification failed", ko = "이메일 인증에 실패했습니다.")]
    EmailVerificationFailed,

    #[error("start now not supported")]
    #[translate(
        en = "Starting now is not supported",
        ko = "즉시 시작은 지원되지 않습니다."
    )]
    StartNowNotSupported,

    #[error("finish now not supported")]
    #[translate(
        en = "Finishing now is not supported",
        ko = "즉시 종료는 지원되지 않습니다."
    )]
    FinishNowNotSupported,

    #[error("update failed")]
    #[translate(en = "Failed to update space", ko = "스페이스 업데이트에 실패했습니다.")]
    UpdateFailed,

    #[error("invalid panel quota")]
    #[translate(en = "Invalid panel quota", ko = "유효하지 않은 패널 정원입니다.")]
    InvalidPanelQuota,

    #[error("reward distribution failed")]
    #[translate(
        en = "Reward distribution failed",
        ko = "보상 분배에 실패했습니다."
    )]
    RewardDistributionFailed,
}

#[cfg(feature = "server")]
impl SpaceError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceError::EmailVerificationFailed
            | SpaceError::UpdateFailed
            | SpaceError::RewardDistributionFailed => StatusCode::INTERNAL_SERVER_ERROR,

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
