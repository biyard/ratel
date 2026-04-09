pub use thiserror::Error;

use crate::common::*;

/// Typed error enum for the Quest Map / gamification feature.
///
/// Registered in `common::Error` via `#[from]` + `#[translate(from)]`
/// so that server functions can return `Result<T>` and propagate these
/// errors with localized messages out-of-the-box.
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum GamificationError {
    #[error("chapter must have at least one action before deletion")]
    #[translate(
        en = "Chapter still has actions — move or delete them first",
        ko = "챕터에 퀘스트가 남아 있습니다. 먼저 이동하거나 삭제하세요"
    )]
    ChapterNotEmpty,

    #[error("chapter not found")]
    #[translate(en = "Chapter not found", ko = "챕터를 찾을 수 없습니다")]
    ChapterNotFound,

    #[error("dependency cycle detected")]
    #[translate(
        en = "That would create a dependency loop",
        ko = "의존 관계가 순환됩니다"
    )]
    CycleDetected,

    #[error("cross-chapter dependency forbidden")]
    #[translate(
        en = "Dependencies must stay within a single chapter",
        ko = "의존 관계는 같은 챕터 안에서만 설정할 수 있습니다"
    )]
    CrossChapterDependency,

    #[error("action locked: prerequisites not met")]
    #[translate(
        en = "Complete the prerequisite quests first",
        ko = "선행 퀘스트를 먼저 완료하세요"
    )]
    ActionLocked,

    #[error("prior chapter incomplete")]
    #[translate(
        en = "Finish the previous chapter first",
        ko = "이전 챕터를 먼저 완료하세요"
    )]
    PriorChapterIncomplete,

    #[error("role mismatch for chapter actor")]
    #[translate(
        en = "You're not at the right role for this chapter",
        ko = "이 챕터를 진행할 수 있는 역할이 아닙니다"
    )]
    RoleMismatch,
}

#[cfg(feature = "server")]
impl GamificationError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            GamificationError::ChapterNotFound => StatusCode::NOT_FOUND,
            GamificationError::RoleMismatch => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for GamificationError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for GamificationError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
