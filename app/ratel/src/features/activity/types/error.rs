use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum ActivityError {
    #[error("activity already recorded for this action")]
    #[translate(en = "You have already completed this action", ko = "이미 완료한 활동입니다")]
    AlreadyRecorded,

    #[error("score aggregation failed")]
    #[translate(en = "Score update failed, please try again", ko = "점수 업데이트에 실패했습니다")]
    AggregationFailed,

    #[error("invalid activity data")]
    #[translate(en = "Invalid activity data", ko = "잘못된 활동 데이터입니다")]
    InvalidData,

    #[error("score load failed")]
    #[translate(en = "Failed to load score", ko = "점수 로드에 실패했습니다.")]
    ScoreLoadFailed,

    #[error("ranking load failed")]
    #[translate(en = "Failed to load ranking", ko = "랭킹 로드에 실패했습니다.")]
    RankingLoadFailed,
}

#[cfg(feature = "server")]
impl ActivityError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            ActivityError::AlreadyRecorded => StatusCode::CONFLICT,
            ActivityError::AggregationFailed
            | ActivityError::ScoreLoadFailed
            | ActivityError::RankingLoadFailed => StatusCode::INTERNAL_SERVER_ERROR,
            ActivityError::InvalidData => StatusCode::BAD_REQUEST,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for ActivityError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for ActivityError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
