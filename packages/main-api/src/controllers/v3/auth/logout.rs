use axum::*;
use bdk::prelude::*;
use response::IntoResponse;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct LogoutResponse {
    pub status: String,
}

pub async fn logout_handler(session: tower_sessions::Session) -> impl IntoResponse {
    tracing::debug!("Logging out session: {:?}", session.id());
    // let _ = session.delete().await;
    let _ = session.flush().await;

    (
        axum::http::StatusCode::OK,
        Json(LogoutResponse {
            status: "OK".to_string(),
        }),
    )
    // Ok(Json(LogoutResponse {
    //     status: "OK".to_string(),
    // }))
}
