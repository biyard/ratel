use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum AuthError {
    #[error("invalid credentials")]
    #[translate(
        en = "Invalid email or password",
        ko = "이메일 또는 비밀번호가 올바르지 않습니다."
    )]
    InvalidCredentials,

    #[error("invalid signature")]
    #[translate(en = "Invalid signature", ko = "유효하지 않은 서명입니다.")]
    InvalidSignature,

    #[error("nonce mismatch")]
    #[translate(
        en = "Authentication failed. Please try again.",
        ko = "인증에 실패했습니다. 다시 시도해주세요."
    )]
    NonceMismatch,

    #[error("nonce not found")]
    #[translate(
        en = "Session expired. Please try again.",
        ko = "세션이 만료되었습니다. 다시 시도해주세요."
    )]
    NonceNotFound,

    #[error("token revoked")]
    #[translate(en = "Your session has been revoked", ko = "세션이 취소되었습니다.")]
    TokenRevoked,

    #[error("token expired")]
    #[translate(en = "Your session has expired", ko = "세션이 만료되었습니다.")]
    TokenExpired,

    #[error("invalid refresh token")]
    #[translate(
        en = "Invalid session. Please sign in again.",
        ko = "유효하지 않은 세션입니다. 다시 로그인해주세요."
    )]
    InvalidRefreshToken,

    #[error("invalid telegram data")]
    #[translate(
        en = "Telegram authentication failed",
        ko = "텔레그램 인증에 실패했습니다."
    )]
    InvalidTelegramData,

    #[error("user not found")]
    #[translate(en = "User not found", ko = "사용자를 찾을 수 없습니다.")]
    UserNotFound,

    #[error("phone not found")]
    #[translate(en = "Phone number not registered", ko = "등록되지 않은 전화번호입니다.")]
    PhoneNotFound,

    #[error("session failed")]
    #[translate(
        en = "Session error. Please try again.",
        ko = "세션 오류가 발생했습니다. 다시 시도해주세요."
    )]
    SessionFailed,

    #[error("invalid input")]
    #[translate(en = "Invalid input", ko = "유효하지 않은 입력입니다.")]
    InvalidInput,

    #[error("invalid signature hex")]
    #[translate(en = "Invalid signature format", ko = "유효하지 않은 서명 형식입니다.")]
    InvalidSignatureHex,

    #[error("invalid recovery id")]
    #[translate(en = "Invalid signature", ko = "유효하지 않은 서명입니다.")]
    InvalidRecoveryId,

    #[error("public key recovery failed")]
    #[translate(en = "Signature verification failed", ko = "서명 검증에 실패했습니다.")]
    PublicKeyRecoveryFailed,

    #[error("signature length invalid")]
    #[translate(en = "Invalid signature length", ko = "유효하지 않은 서명 길이입니다.")]
    SignatureLengthInvalid,

    #[error("wallet connect failed")]
    #[translate(en = "Wallet connection failed", ko = "지갑 연결에 실패했습니다.")]
    WalletConnectFailed,

    #[error("user info parse failed")]
    #[translate(
        en = "Failed to load user information",
        ko = "사용자 정보 로드에 실패했습니다."
    )]
    UserInfoParseFailed,

    #[error("email template failed")]
    #[translate(en = "Email service failed", ko = "이메일 서비스에 실패했습니다.")]
    EmailTemplateFailed,

    #[error("telegram bot token missing")]
    #[translate(
        en = "Telegram service unavailable",
        ko = "텔레그램 서비스를 사용할 수 없습니다."
    )]
    TelegramBotTokenMissing,

    #[error("EVM address mismatch")]
    #[translate(
        en = "Wallet address does not match",
        ko = "지갑 주소가 일치하지 않습니다."
    )]
    EvmAddressMismatch,

    #[error("sign-in unsupported on this platform")]
    #[translate(
        en = "Google sign-in is not available on mobile yet. Please use email sign-in.",
        ko = "모바일에서는 Google 로그인을 지원하지 않습니다. 이메일 로그인을 이용해주세요."
    )]
    SignInUnsupportedOnPlatform,
}

#[cfg(feature = "server")]
impl AuthError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            AuthError::InvalidCredentials
            | AuthError::InvalidSignature
            | AuthError::NonceMismatch
            | AuthError::NonceNotFound
            | AuthError::TokenRevoked
            | AuthError::TokenExpired
            | AuthError::InvalidRefreshToken
            | AuthError::InvalidTelegramData
            | AuthError::UserNotFound
            | AuthError::PhoneNotFound
            | AuthError::EvmAddressMismatch => StatusCode::UNAUTHORIZED,

            AuthError::InvalidInput
            | AuthError::InvalidSignatureHex
            | AuthError::InvalidRecoveryId
            | AuthError::PublicKeyRecoveryFailed
            | AuthError::SignatureLengthInvalid => StatusCode::BAD_REQUEST,

            AuthError::SessionFailed
            | AuthError::WalletConnectFailed
            | AuthError::UserInfoParseFailed
            | AuthError::EmailTemplateFailed
            | AuthError::TelegramBotTokenMissing
            | AuthError::SignInUnsupportedOnPlatform => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for AuthError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for AuthError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
