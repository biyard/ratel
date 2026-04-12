use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SocialError {
    #[error("failed to delete team")]
    #[translate(en = "Failed to delete team", ko = "팀 삭제에 실패했습니다.")]
    TeamDeleteFailed,

    #[error("failed to delete group")]
    #[translate(en = "Failed to delete group", ko = "그룹 삭제에 실패했습니다.")]
    GroupDeleteFailed,

    #[error("invalid gender")]
    #[translate(en = "Invalid gender", ko = "유효하지 않은 성별입니다.")]
    InvalidGender,

    #[error("invalid membership tier")]
    #[translate(en = "Invalid membership tier", ko = "유효하지 않은 멤버십 등급입니다.")]
    InvalidMembershipTier,

    #[error("password too short")]
    #[translate(en = "Password is too short", ko = "비밀번호가 너무 짧습니다.")]
    PasswordTooShort,

    #[error("password mismatch")]
    #[translate(en = "Passwords do not match", ko = "비밀번호가 일치하지 않습니다.")]
    PasswordMismatch,

    #[error("incorrect current password")]
    #[translate(
        en = "Current password is incorrect",
        ko = "현재 비밀번호가 올바르지 않습니다."
    )]
    IncorrectCurrentPassword,

    #[error("invalid team name")]
    #[translate(en = "Invalid team name", ko = "유효하지 않은 팀 이름입니다.")]
    InvalidTeamName,

    #[error("team name taken")]
    #[translate(en = "Team name is already taken", ko = "이미 사용 중인 팀 이름입니다.")]
    TeamNameTaken,

    #[error("PortOne request failed")]
    #[translate(en = "Payment service request failed", ko = "결제 서비스 요청에 실패했습니다.")]
    PortOneRequestFailed,

    #[error("PortOne returned bad status")]
    #[translate(
        en = "Payment service returned an error",
        ko = "결제 서비스에서 오류가 반환되었습니다."
    )]
    PortOneBadStatus,

    #[error("DAO registration failed")]
    #[translate(en = "DAO registration failed", ko = "DAO 등록에 실패했습니다.")]
    DaoRegistrationFailed,

    #[error("wallet connect failed")]
    #[translate(en = "Wallet connection failed", ko = "지갑 연결에 실패했습니다.")]
    WalletConnectFailed,

    #[error("invalid verification attribute")]
    #[translate(
        en = "Invalid verification attribute",
        ko = "유효하지 않은 인증 속성입니다."
    )]
    InvalidVerificationAttribute,

    #[error("session not found")]
    #[translate(en = "Session not found", ko = "세션을 찾을 수 없습니다.")]
    SessionNotFound,

    #[error("user not found")]
    #[translate(en = "User not found", ko = "사용자를 찾을 수 없습니다.")]
    UserNotFound,
}

#[cfg(feature = "server")]
impl SocialError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SocialError::InvalidGender
            | SocialError::InvalidMembershipTier
            | SocialError::PasswordTooShort
            | SocialError::PasswordMismatch
            | SocialError::InvalidTeamName
            | SocialError::TeamNameTaken
            | SocialError::PortOneBadStatus
            | SocialError::InvalidVerificationAttribute => StatusCode::BAD_REQUEST,

            SocialError::IncorrectCurrentPassword
            | SocialError::SessionNotFound
            | SocialError::UserNotFound => StatusCode::UNAUTHORIZED,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SocialError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SocialError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
