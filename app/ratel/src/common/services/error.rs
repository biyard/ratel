use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum ServiceError {
    #[error("biyard API request failed")]
    #[translate(en = "Service request failed", ko = "서비스 요청에 실패했습니다.")]
    BiyardApiRequestFailed,

    #[error("biyard API returned bad status")]
    #[translate(en = "Service returned an error", ko = "서비스에서 오류가 반환되었습니다.")]
    BiyardApiBadStatus,

    #[error("biyard API returned empty response")]
    #[translate(en = "Service returned empty response", ko = "서비스에서 빈 응답이 반환되었습니다.")]
    BiyardApiEmptyResponse,

    #[error("ICP agent creation failed")]
    #[translate(
        en = "Blockchain service unavailable",
        ko = "블록체인 서비스를 사용할 수 없습니다."
    )]
    IcpAgentFailed,

    #[error("ICP call failed")]
    #[translate(en = "Blockchain call failed", ko = "블록체인 호출에 실패했습니다.")]
    IcpCallFailed,

    #[error("ICP query failed")]
    #[translate(en = "Blockchain query failed", ko = "블록체인 조회에 실패했습니다.")]
    IcpQueryFailed,

    #[error("ICP candid encode failed")]
    #[translate(en = "Data encoding failed", ko = "데이터 인코딩에 실패했습니다.")]
    IcpCandidEncodeFailed,

    #[error("ICP candid decode failed")]
    #[translate(en = "Data decoding failed", ko = "데이터 디코딩에 실패했습니다.")]
    IcpCandidDecodeFailed,

    #[error("ICP root key fetch failed")]
    #[translate(
        en = "Blockchain service unavailable",
        ko = "블록체인 서비스를 사용할 수 없습니다."
    )]
    IcpRootKeyFailed,

    #[error("invalid canister ID")]
    #[translate(
        en = "Invalid blockchain configuration",
        ko = "잘못된 블록체인 설정입니다."
    )]
    InvalidCanisterId,

    #[error("OAuth request failed")]
    #[translate(en = "Authentication service failed", ko = "인증 서비스에 실패했습니다.")]
    OAuthRequestFailed,

    #[error("OAuth response parse failed")]
    #[translate(en = "Authentication service failed", ko = "인증 서비스에 실패했습니다.")]
    OAuthParseFailed,
}

#[cfg(feature = "server")]
impl ServiceError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            ServiceError::BiyardApiBadStatus | ServiceError::InvalidCanisterId => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for ServiceError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for ServiceError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
