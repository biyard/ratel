use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum CrossPostingError {
    #[error("connection not found")]
    #[translate(en = "Connection not found", ko = "연결을 찾을 수 없습니다")]
    ConnectionNotFound,

    #[error("connection failed")]
    #[translate(en = "Failed to connect account", ko = "계정 연결에 실패했습니다")]
    ConnectFailed,

    #[error("list failed")]
    #[translate(en = "Failed to load connections", ko = "연결 목록을 불러오지 못했습니다")]
    ListFailed,

    #[error("update failed")]
    #[translate(en = "Failed to update connection", ko = "연결 업데이트에 실패했습니다")]
    UpdateFailed,

    #[error("invalid bluesky credentials")]
    #[translate(
        en = "Invalid Bluesky handle or app password",
        ko = "Bluesky 핸들 또는 앱 비밀번호가 잘못되었습니다"
    )]
    BlueskyAuthFailed,

    #[error("oauth state mismatch")]
    #[translate(
        en = "OAuth verification failed. Please try again.",
        ko = "OAuth 검증에 실패했습니다. 다시 시도해주세요."
    )]
    OAuthStateMismatch,

    #[error("threads requires instagram professional account")]
    #[translate(
        en = "To connect Threads, please switch to an Instagram Professional account.",
        ko = "Threads 연결을 위해 인스타그램 프로페셔널 계정 전환이 필요합니다."
    )]
    ThreadsRequiresInstagramProfessional,

    #[error("syndication job not found")]
    #[translate(en = "Syndication record not found", ko = "외부 게시 기록을 찾을 수 없습니다")]
    SyndicationJobNotFound,

    #[error("retry not allowed")]
    #[translate(
        en = "This syndication cannot be retried in its current state",
        ko = "현재 상태에서는 재시도할 수 없습니다"
    )]
    RetryNotAllowed,

    #[error("not authorized")]
    #[translate(
        en = "You don't have permission to view this syndication panel",
        ko = "이 외부 게시 패널을 볼 권한이 없습니다"
    )]
    NotAuthorized,

    /// Stage 2 dispatcher couldn't acquire / contend the per-job lock
    /// (unexpected DynamoDB error path — `ConditionalCheckFailedException`
    /// itself is handled as "lock held elsewhere", not surfaced here).
    /// Server-only — never user-facing; the surface is logged and the
    /// EventBridge retry policy decides what to do next.
    #[error("dispatch lock acquisition failed")]
    #[translate(
        en = "Dispatch lock acquisition failed",
        ko = "발송 잠금 획득에 실패했습니다"
    )]
    DispatchLockFailed,

    /// Stage 2 dispatcher couldn't write the terminal-state row
    /// (`commit_skipped` / `commit_published` / `commit_failed`). Same
    /// server-side path as `DispatchLockFailed` — surfaced via logs only.
    #[error("commit failed")]
    #[translate(
        en = "Failed to commit syndication state",
        ko = "외부 게시 상태 저장에 실패했습니다"
    )]
    CommitFailed,
}

#[cfg(feature = "server")]
impl CrossPostingError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            CrossPostingError::ConnectionNotFound | CrossPostingError::SyndicationJobNotFound => {
                StatusCode::NOT_FOUND
            }
            CrossPostingError::NotAuthorized => StatusCode::FORBIDDEN,
            CrossPostingError::BlueskyAuthFailed
            | CrossPostingError::OAuthStateMismatch
            | CrossPostingError::ThreadsRequiresInstagramProfessional
            | CrossPostingError::RetryNotAllowed => StatusCode::BAD_REQUEST,
            CrossPostingError::ConnectFailed
            | CrossPostingError::ListFailed
            | CrossPostingError::UpdateFailed
            | CrossPostingError::DispatchLockFailed
            | CrossPostingError::CommitFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for CrossPostingError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for CrossPostingError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
