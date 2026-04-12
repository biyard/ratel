use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceAppError {
    #[error("install failed")]
    #[translate(en = "Failed to install app", ko = "앱 설치에 실패했습니다.")]
    InstallFailed,

    #[error("uninstall failed")]
    #[translate(en = "Failed to uninstall app", ko = "앱 제거에 실패했습니다.")]
    UninstallFailed,

    #[error("deploy failed")]
    #[translate(en = "Failed to deploy app", ko = "앱 배포에 실패했습니다.")]
    DeployFailed,

    #[error("archive provider failed")]
    #[translate(
        en = "Failed to access archive provider",
        ko = "아카이브 제공자 접근에 실패했습니다."
    )]
    ArchiveProviderFailed,

    #[error("archive block failed")]
    #[translate(
        en = "Failed to access archive block",
        ko = "아카이브 블록 접근에 실패했습니다."
    )]
    ArchiveBlockFailed,

    #[error("archive logs failed")]
    #[translate(
        en = "Failed to access archive logs",
        ko = "아카이브 로그 접근에 실패했습니다."
    )]
    ArchiveLogsFailed,

    #[error("excel export failed")]
    #[translate(en = "Failed to export to Excel", ko = "엑셀 내보내기에 실패했습니다.")]
    ExcelExportFailed,

    #[error("chart render failed")]
    #[translate(en = "Failed to render chart", ko = "차트 렌더링에 실패했습니다.")]
    ChartRenderFailed,

    #[error("copy text failed")]
    #[translate(en = "Failed to copy text", ko = "텍스트 복사에 실패했습니다.")]
    CopyTextFailed,

    #[error("panel quota create failed")]
    #[translate(
        en = "Failed to create panel quota",
        ko = "패널 정원 생성에 실패했습니다."
    )]
    PanelQuotaCreateFailed,

    #[error("panel quota delete failed")]
    #[translate(
        en = "Failed to delete panel quota",
        ko = "패널 정원 삭제에 실패했습니다."
    )]
    PanelQuotaDeleteFailed,

    #[error("invalid EVM address")]
    #[translate(en = "Invalid EVM address", ko = "유효하지 않은 EVM 주소입니다.")]
    InvalidEvmAddress,

    #[error("invalid invitation email")]
    #[translate(
        en = "Invalid invitation email",
        ko = "유효하지 않은 초대 이메일입니다."
    )]
    InvalidInvitationEmail,

    #[error("creator cannot be removed")]
    #[translate(
        en = "The creator cannot be removed",
        ko = "생성자는 제거할 수 없습니다."
    )]
    CreatorCannotBeRemoved,

    #[error("incentive address required")]
    #[translate(
        en = "Incentive address is required",
        ko = "인센티브 주소가 필요합니다."
    )]
    IncentiveAddressRequired,

    #[error("incentive chain required")]
    #[translate(
        en = "Incentive chain is required",
        ko = "인센티브 체인이 필요합니다."
    )]
    IncentiveChainRequired,

    #[error("unsupported on server")]
    #[translate(
        en = "This operation is not supported on the server",
        ko = "이 작업은 서버에서 지원되지 않습니다."
    )]
    UnsupportedOnServer,
}

#[cfg(feature = "server")]
impl SpaceAppError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceAppError::InvalidEvmAddress
            | SpaceAppError::InvalidInvitationEmail
            | SpaceAppError::CreatorCannotBeRemoved
            | SpaceAppError::IncentiveAddressRequired
            | SpaceAppError::IncentiveChainRequired
            | SpaceAppError::UnsupportedOnServer => StatusCode::BAD_REQUEST,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceAppError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceAppError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
