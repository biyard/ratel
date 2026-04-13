use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceReportError {
    #[error("analyze load failed")]
    #[translate(en = "Failed to load analysis", ko = "분석 로드에 실패했습니다.")]
    AnalyzeLoadFailed,

    #[error("analyze update failed")]
    #[translate(en = "Failed to update analysis", ko = "분석 업데이트에 실패했습니다.")]
    AnalyzeUpdateFailed,
}

#[cfg(feature = "server")]
impl SpaceReportError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceReportError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceReportError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
