use std::sync::Arc;

mod sprint_league;

use dto::{
    Result, TelegramNotificationPayload,
    by_axum::axum::{Json, extract::State},
};

use crate::AppState;

pub async fn notify_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TelegramNotificationPayload>,
) -> Result<()> {
    match payload {
        TelegramNotificationPayload::SprintLeague(req) => {
            sprint_league::handler(&state.pool, &state.bot, req.space_id).await?;
        }
    }

    Ok(())
}
