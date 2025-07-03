use bdk::prelude::by_axum::axum::Json;
use dto::{Error, Result};
use tower_sessions::Session;

pub async fn logout_handler(session: Session) -> Result<Json<()>> {
    if let Err(e) = session.delete().await {
        tracing::error!("Failed to delete session: {}", e);

        return Err(Error::Unknown("failed to delete session".to_string()));
    }

    Ok(Json(()))
}
