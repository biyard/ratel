use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceActionError {
    #[error("action load failed")]
    #[translate(en = "Failed to load action", ko = "액션 로드에 실패했습니다.")]
    ActionLoadFailed,

    #[error("action update failed")]
    #[translate(en = "Failed to update action", ko = "액션 업데이트에 실패했습니다.")]
    ActionUpdateFailed,

    #[error("action delete failed")]
    #[translate(en = "Failed to delete action", ko = "액션 삭제에 실패했습니다.")]
    ActionDeleteFailed,

    #[error("transaction failed")]
    #[translate(en = "Transaction failed", ko = "트랜잭션에 실패했습니다.")]
    TransactionFailed,

    #[error("reward template failed")]
    #[translate(
        en = "Failed to apply reward template",
        ko = "보상 템플릿 적용에 실패했습니다."
    )]
    RewardTemplateFailed,

    #[error("membership check failed")]
    #[translate(
        en = "Failed to check membership",
        ko = "멤버십 확인에 실패했습니다."
    )]
    MembershipCheckFailed,

    #[error("invalid time range")]
    #[translate(
        en = "Start time must be before end time",
        ko = "시작 시간은 종료 시간보다 이전이어야 합니다."
    )]
    InvalidTimeRange,

    #[error("invalid status transition")]
    #[translate(
        en = "Invalid action status transition",
        ko = "허용되지 않은 액션 상태 전이입니다."
    )]
    InvalidStatusTransition,

    #[error("invalid dependency")]
    #[translate(
        en = "Invalid action dependency",
        ko = "유효하지 않은 액션 의존성입니다."
    )]
    InvalidDependency,

    #[error("dependencies not met")]
    #[translate(
        en = "Complete the required actions first",
        ko = "선행 액션을 먼저 완료하세요."
    )]
    DependenciesNotMet,

    #[error("action not ongoing")]
    #[translate(
        en = "Action is not currently ongoing",
        ko = "이 액션은 현재 진행 중이 아닙니다."
    )]
    ActionNotOngoing,
}

#[cfg(feature = "server")]
impl SpaceActionError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceActionError::InvalidTimeRange
            | SpaceActionError::InvalidStatusTransition
            | SpaceActionError::InvalidDependency
            | SpaceActionError::DependenciesNotMet
            | SpaceActionError::ActionNotOngoing => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceActionError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceActionError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
