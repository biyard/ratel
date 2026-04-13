use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum MembershipPaymentError {
    #[error("invalid currency")]
    #[translate(en = "Invalid currency", ko = "유효하지 않은 통화입니다.")]
    InvalidCurrency,

    #[error("missing card info")]
    #[translate(en = "Card information is missing", ko = "카드 정보가 누락되었습니다.")]
    MissingCardInfo,

    #[error("missing billing key")]
    #[translate(en = "Billing key is missing", ko = "결제 키가 누락되었습니다.")]
    MissingBillingKey,

    #[error("PortOne request failed")]
    #[translate(en = "Payment service request failed", ko = "결제 서비스 요청에 실패했습니다.")]
    PortOneRequestFailed,

    #[error("PortOne payment failed")]
    #[translate(en = "Payment processing failed", ko = "결제 처리에 실패했습니다.")]
    PortOnePaymentFailed,

    #[error("PortOne schedule failed")]
    #[translate(
        en = "Payment scheduling failed",
        ko = "결제 예약에 실패했습니다."
    )]
    PortOneScheduleFailed,

    #[error("PortOne verify failed")]
    #[translate(
        en = "Payment verification failed",
        ko = "결제 인증에 실패했습니다."
    )]
    PortOneVerifyFailed,

    #[error("PortOne cancel failed")]
    #[translate(en = "Payment cancellation failed", ko = "결제 취소에 실패했습니다.")]
    PortOneCancelFailed,

    #[error("webhook processing failed")]
    #[translate(
        en = "Payment notification processing failed",
        ko = "결제 알림 처리에 실패했습니다."
    )]
    WebhookProcessingFailed,

    #[error("AWS conversion failed")]
    #[translate(
        en = "Internal data conversion failed",
        ko = "내부 데이터 변환에 실패했습니다."
    )]
    AwsConversionFailed,

    #[error("session conversion failed")]
    #[translate(
        en = "Session data conversion failed",
        ko = "세션 데이터 변환에 실패했습니다."
    )]
    SessionConversionFailed,
}

#[cfg(feature = "server")]
impl MembershipPaymentError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            MembershipPaymentError::InvalidCurrency
            | MembershipPaymentError::MissingCardInfo
            | MembershipPaymentError::MissingBillingKey
            | MembershipPaymentError::PortOneRequestFailed
            | MembershipPaymentError::PortOnePaymentFailed
            | MembershipPaymentError::PortOneScheduleFailed
            | MembershipPaymentError::PortOneVerifyFailed
            | MembershipPaymentError::PortOneCancelFailed => StatusCode::BAD_REQUEST,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for MembershipPaymentError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for MembershipPaymentError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
