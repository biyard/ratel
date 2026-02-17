use common::models::*;
use common::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogoutResponse {
    pub status: String,
}

#[post("/api/auth/logout", session: Extension<TowerSession>)]
pub async fn logout_handler() -> std::result::Result<LogoutResponse, ServerFnError> {
    let _ = session.flush().await.map_err(|e| {
        ServerFnError::new(format!("Failed to flush session: {:?}", e))
    })?;

    Ok(LogoutResponse {
        status: "OK".to_string(),
    })
}
