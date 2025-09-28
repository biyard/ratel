use axum::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct LogoutResponse {
    pub status: String,
}

pub async fn logout_handler(
    Extension(session): Extension<tower_sessions::Session>,
) -> Result<Json<LogoutResponse>, crate::Error2> {
    tracing::debug!("Logging out session: {:?}", session.id());
    let _ = session.flush().await?;

    Ok(Json(LogoutResponse {
        status: "OK".to_string(),
    }))
}
