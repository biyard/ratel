pub use thiserror::Error;

use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceRewardError {
    #[error("Invalid entity type")]
    #[translate(en = "Invalid entity type", ko = "유효하지 않은 엔티티 타입입니다.")]
    InvalidEntityType,

    #[error("Space reward not found")]
    #[translate(en = "Space reward not found", ko = "스페이스 리워드를 찾을 수 없습니다.")]
    NotFound,

    #[error("Space reward already claimed in period")]
    #[translate(
        en = "Reward already claimed in this period",
        ko = "이 기간에 이미 리워드를 수령했습니다."
    )]
    AlreadyClaimedInPeriod,

    #[error("Space reward max claims reached")]
    #[translate(
        en = "Maximum reward claims reached",
        ko = "리워드 최대 수령 횟수에 도달했습니다."
    )]
    MaxClaimsReached,

    #[error("Space reward max points reached")]
    #[translate(
        en = "Maximum reward points reached",
        ko = "리워드 최대 포인트에 도달했습니다."
    )]
    MaxPointsReached,

    #[error("Space reward max user claims reached")]
    #[translate(
        en = "Maximum user reward claims reached",
        ko = "유저 리워드 최대 수령 횟수에 도달했습니다."
    )]
    MaxUserClaimsReached,

    #[error("Space reward max user points reached")]
    #[translate(
        en = "Maximum user reward points reached",
        ko = "유저 리워드 최대 포인트에 도달했습니다."
    )]
    MaxUserPointsReached,

    #[error("Reward not found")]
    #[translate(en = "Reward not found", ko = "리워드를 찾을 수 없습니다.")]
    RewardNotFound,

    #[error("Reward already exists")]
    #[translate(en = "Reward already exists", ko = "리워드가 이미 존재합니다.")]
    RewardAlreadyExists,
}

#[cfg(feature = "server")]
impl SpaceRewardError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceRewardError::NotFound | SpaceRewardError::RewardNotFound => StatusCode::NOT_FOUND,
            SpaceRewardError::RewardAlreadyExists => StatusCode::CONFLICT,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceRewardError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceRewardError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
