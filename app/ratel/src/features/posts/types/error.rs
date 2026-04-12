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
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
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
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for PostError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
