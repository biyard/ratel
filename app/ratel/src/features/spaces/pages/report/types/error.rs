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

    #[error("report list failed")]
    #[translate(en = "Failed to load reports", ko = "보고서 목록을 불러오지 못했습니다.")]
    ReportListFailed,

    #[error("report load failed")]
    #[translate(en = "Failed to load report", ko = "보고서를 불러오지 못했습니다.")]
    ReportLoadFailed,

    #[error("report create failed")]
    #[translate(en = "Failed to create report", ko = "보고서를 생성하지 못했습니다.")]
    ReportCreateFailed,

    #[error("report not found")]
    #[translate(en = "Report not found", ko = "보고서를 찾을 수 없습니다.")]
    ReportNotFound,

    #[error("report delete failed")]
    #[translate(en = "Failed to delete report", ko = "보고서를 삭제하지 못했습니다.")]
    ReportDeleteFailed,

    #[error("report update failed")]
    #[translate(en = "Failed to update report", ko = "보고서를 저장하지 못했습니다.")]
    ReportUpdateFailed,
}

#[cfg(feature = "server")]
impl SpaceReportError {
    pub fn status_code(&self) -> crate::axum::http::StatusCode {
        use crate::axum::http::StatusCode;
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceReportError {
    fn into_response(self) -> crate::axum::response::Response {
        use crate::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceReportError {
    fn as_status_code(&self) -> crate::axum::http::StatusCode {
        self.status_code()
    }
}
