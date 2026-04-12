use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum FileUploadError {
    #[error("unsupported file type")]
    #[translate(
        en = "This file type is not supported",
        ko = "지원되지 않는 파일 형식입니다."
    )]
    UnsupportedFileType,

    #[error("file size limit exceeded")]
    #[translate(
        en = "File size exceeds the allowed limit",
        ko = "파일 크기가 허용 한도를 초과했습니다."
    )]
    FileSizeLimitExceeded,

    #[error("upload failed")]
    #[translate(en = "File upload failed", ko = "파일 업로드에 실패했습니다.")]
    UploadFailed,

    #[error("invalid upload response")]
    #[translate(en = "File upload failed", ko = "파일 업로드에 실패했습니다.")]
    InvalidUploadResponse,
}

#[cfg(feature = "server")]
impl FileUploadError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            FileUploadError::UnsupportedFileType | FileUploadError::FileSizeLimitExceeded => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for FileUploadError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for FileUploadError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
