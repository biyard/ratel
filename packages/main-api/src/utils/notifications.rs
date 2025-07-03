use bdk::prelude::by_axum::auth::Authorization;
use bdk::prelude::*;
use dto::*;

use crate::utils::users::extract_user_id;

#[derive(Debug, Clone)]
pub enum RatelNotification {
    BoostingSpace { title: String, image_url: String, space_id: i64 },
    ConnectNetwork { display_name: String, profile_url: String },
}

impl RatelNotification {
    fn to_notification_data(&self) -> (Option<String>, String, Option<String>, Option<String>, Option<i64>, NotificationType) {
        match self {
            RatelNotification::BoostingSpace { title, image_url, space_id } => {
                (
                    Some(title.clone()),
                    "Boosting space notification".to_string(),
                    Some(image_url.clone()),
                    None,
                    Some(*space_id),
                    NotificationType::BoostingSpace,
                )
            }
            RatelNotification::ConnectNetwork { display_name, profile_url } => {
                (
                    Some(format!("{} has connected to your network", display_name)),
                    "Network connection notification".to_string(),
                    None,
                    Some(profile_url.clone()),
                    None,
                    NotificationType::ConnectNetwork,
                )
            }
        }
    }
}

pub async fn send_notification(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
    user_id: i64,
    content: RatelNotification,
) -> Result<Notification> {
    let from_user_id = extract_user_id(pool, auth).await?;
    let (title, metadata, image_url, profile_url, space_id, notification_type) = content.to_notification_data();

    Notification::get_repository(pool.clone())
        .insert(
            Some(user_id),
            from_user_id,
            title,
            metadata,
            image_url,
            profile_url,
            space_id,
            notification_type,
            false
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert notification: {:?}", e);
            Error::DatabaseException(e.to_string())
        })
}