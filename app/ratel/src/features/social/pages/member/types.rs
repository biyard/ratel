use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum MemberError {
    #[error("User not found")]
    #[translate(en = "User not found", ko = "사용자를 찾을 수 없습니다.")]
    UserNotFound,

    #[error("Too many invitations")]
    #[translate(en = "You can invite up to 100 members at once.", ko = "한번에 최대 100명까지 초대할 수 있습니다.")]
    TooManyInvitations,

    #[error("Cannot change your own role")]
    #[translate(en = "You cannot change your own role.", ko = "본인의 역할은 변경할 수 없습니다.")]
    CannotChangeOwnRole,

    #[error("Cannot change owner role")]
    #[translate(en = "The team owner's role cannot be changed.", ko = "팀 소유자의 역할은 변경할 수 없습니다.")]
    CannotChangeOwnerRole,
}

#[cfg(feature = "server")]
impl MemberError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            MemberError::UserNotFound => StatusCode::NOT_FOUND,
            MemberError::TooManyInvitations => StatusCode::BAD_REQUEST,
            MemberError::CannotChangeOwnRole => StatusCode::FORBIDDEN,
            MemberError::CannotChangeOwnerRole => StatusCode::FORBIDDEN,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for MemberError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for MemberError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
