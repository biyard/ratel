use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum NotificationsError {
    #[error("inbox entry not found")]
    #[translate(en = "Notification not found", ko = "알림을 찾을 수 없습니다")]
    InboxEntryNotFound,

    #[error("mark-read failed")]
    #[translate(en = "Failed to mark as read", ko = "읽음 처리에 실패했습니다")]
    MarkReadFailed,

    #[error("list failed")]
    #[translate(en = "Failed to load notifications", ko = "알림 불러오기에 실패했습니다")]
    ListFailed,
}

#[cfg(feature = "server")]
impl NotificationsError {
    pub fn status_code(&self) -> crate::axum::http::StatusCode {
        use crate::axum::http::StatusCode;
        match self {
            NotificationsError::InboxEntryNotFound => StatusCode::NOT_FOUND,
            NotificationsError::MarkReadFailed | NotificationsError::ListFailed => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for NotificationsError {
    fn into_response(self) -> crate::axum::response::Response {
        use crate::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for NotificationsError {
    fn as_status_code(&self) -> crate::axum::http::StatusCode {
        self.status_code()
    }
}
