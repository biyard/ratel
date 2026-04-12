use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum InfraError {
    #[error("S3 operation failed")]
    #[translate(
        en = "File storage operation failed",
        ko = "파일 저장소 작업에 실패했습니다."
    )]
    S3OperationFailed,

    #[error("Bedrock serialize failed")]
    #[translate(en = "AI service failed", ko = "AI 서비스에 실패했습니다.")]
    BedrockSerializeFailed,

    #[error("Bedrock invoke failed")]
    #[translate(en = "AI service failed", ko = "AI 서비스에 실패했습니다.")]
    BedrockInvokeFailed,

    #[error("Bedrock parse failed")]
    #[translate(en = "AI service failed", ko = "AI 서비스에 실패했습니다.")]
    BedrockParseFailed,

    #[error("Bedrock no embedding")]
    #[translate(en = "AI service failed", ko = "AI 서비스에 실패했습니다.")]
    BedrockNoEmbedding,

    #[error("Stream deserialize failed")]
    #[translate(en = "Data processing failed", ko = "데이터 처리에 실패했습니다.")]
    StreamDeserializeFailed,

    #[error("Stream missing image")]
    #[translate(en = "Data processing failed", ko = "데이터 처리에 실패했습니다.")]
    StreamMissingImage,

    #[error("Qdrant operation failed")]
    #[translate(en = "Search service failed", ko = "검색 서비스에 실패했습니다.")]
    QdrantFailed,
}

#[cfg(feature = "server")]
impl InfraError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for InfraError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for InfraError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
