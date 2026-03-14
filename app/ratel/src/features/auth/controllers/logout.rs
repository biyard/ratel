// Migrated from packages/main-api/src/controllers/v3/auth/logout.rs
use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LogoutResponse {
    pub status: String,
}

#[post("/api/auth/logout", session: Extension<tower_sessions::Session>)]
pub async fn logout_handler() -> Result<LogoutResponse> {
    let Extension(session) = session;
    tracing::debug!("Logging out session: {:?}", session.id());
    let _ = session.flush().await?;

    Ok(LogoutResponse {
        status: "OK".to_string(),
    })
}
