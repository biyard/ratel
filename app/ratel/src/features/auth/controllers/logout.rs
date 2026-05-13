// Migrated from packages/main-api/src/controllers/v3/auth/logout.rs
use crate::features::auth::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
