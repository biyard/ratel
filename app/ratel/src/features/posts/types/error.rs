use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum PostError {
    #[error("invalid post author")]
    #[translate(en = "Invalid post author", ko = "유효하지 않은 게시물 작성자입니다.")]
    InvalidAuthor,

    #[error("failed to like post")]
    #[translate(en = "Failed to like post", ko = "게시물 좋아요에 실패했습니다.")]
    LikeFailed,

    #[error("failed to unlike post")]
    #[translate(en = "Failed to unlike post", ko = "게시물 좋아요 취소에 실패했습니다.")]
    UnlikeFailed,

    #[error("failed to create comment")]
    #[translate(en = "Failed to create comment", ko = "댓글 작성에 실패했습니다.")]
    CommentFailed,

    #[error("failed to like comment")]
    #[translate(en = "Failed to like comment", ko = "댓글 좋아요에 실패했습니다.")]
    CommentLikeFailed,

    #[error("failed to unlike comment")]
    #[translate(
        en = "Failed to unlike comment",
        ko = "댓글 좋아요 취소에 실패했습니다."
    )]
    CommentUnlikeFailed,

    #[error("failed to create reply")]
    #[translate(en = "Failed to create reply", ko = "답글 작성에 실패했습니다.")]
    ReplyFailed,

    #[error("invalid comment key")]
    #[translate(en = "Invalid comment key", ko = "유효하지 않은 댓글 키입니다.")]
    InvalidCommentKey,

    #[error("content too short")]
    #[translate(en = "Content is too short", ko = "내용이 너무 짧습니다.")]
    ContentTooShort,

    #[error("post has dependencies")]
    #[translate(
        en = "Cannot delete post with dependencies",
        ko = "의존 관계가 있는 게시물은 삭제할 수 없습니다."
    )]
    HasDependencies,

    #[error("invalid team context")]
    #[translate(en = "Invalid team context", ko = "유효하지 않은 팀 컨텍스트입니다.")]
    InvalidTeamContext,

    #[error("team not found")]
    #[translate(en = "Team not found", ko = "팀을 찾을 수 없습니다.")]
    TeamNotFound,

    #[error("category name required")]
    #[translate(en = "Category name is required", ko = "카테고리 이름이 필요합니다.")]
    CategoryNameRequired,

    #[error("failed to list posts")]
    #[translate(en = "Failed to load posts", ko = "게시물 로드에 실패했습니다.")]
    ListFailed,

    #[error("not accessible")]
    #[translate(en = "You do not have access to this post", ko = "이 게시물에 접근할 수 없습니다.")]
    NotAccessible,
}

#[cfg(feature = "server")]
impl PostError {
    pub fn status_code(&self) -> crate::axum::http::StatusCode {
        use crate::axum::http::StatusCode;
        match self {
            PostError::InvalidCommentKey
            | PostError::ContentTooShort
            | PostError::HasDependencies
            | PostError::InvalidTeamContext
            | PostError::TeamNotFound
            | PostError::CategoryNameRequired => StatusCode::BAD_REQUEST,

            PostError::NotAccessible => StatusCode::UNAUTHORIZED,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for PostError {
    fn into_response(self) -> crate::axum::response::Response {
        use crate::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for PostError {
    fn as_status_code(&self) -> crate::axum::http::StatusCode {
        self.status_code()
    }
}

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum AiPostDraftError {
    #[error("ai draft is a paid-only feature")]
    #[translate(
        en = "AI draft is available on Pro or higher membership.",
        ko = "AI 초안 작성은 Pro 이상 멤버십에서 사용할 수 있습니다."
    )]
    PaidOnly,

    #[error("ai draft already used on this post")]
    #[translate(
        en = "AI draft has already been used on this post.",
        ko = "이 포스트는 이미 AI 초안을 사용했습니다."
    )]
    AlreadyUsed,

    #[error("required ai draft input missing")]
    #[translate(
        en = "Required fields are missing.",
        ko = "필수 입력 항목이 비어있습니다."
    )]
    InvalidInput,

    #[error("ai model call failed")]
    #[translate(
        en = "AI generation failed. Please try again.",
        ko = "AI 초안 생성에 실패했습니다. 다시 시도해 주세요."
    )]
    BedrockFailed,

    #[error("ai response could not be parsed")]
    #[translate(
        en = "AI returned an unexpected response. Please try again.",
        ko = "AI 응답을 처리하지 못했습니다. 다시 시도해 주세요."
    )]
    GenerationFailed,
}

#[cfg(feature = "server")]
impl AiPostDraftError {
    pub fn status_code(&self) -> crate::axum::http::StatusCode {
        use crate::axum::http::StatusCode;
        match self {
            AiPostDraftError::PaidOnly => StatusCode::FORBIDDEN,
            AiPostDraftError::AlreadyUsed => StatusCode::CONFLICT,
            AiPostDraftError::InvalidInput => StatusCode::BAD_REQUEST,
            AiPostDraftError::BedrockFailed | AiPostDraftError::GenerationFailed => {
                StatusCode::BAD_GATEWAY
            }
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for AiPostDraftError {
    fn into_response(self) -> crate::axum::response::Response {
        use crate::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for AiPostDraftError {
    fn as_status_code(&self) -> crate::axum::http::StatusCode {
        self.status_code()
    }
}
