use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use crate::utils::email::send_email;
use bdk::prelude::*;
use dto::{sqlx::PgPool, *};
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct DiscussionController {
    repo: DiscussionRepository,
    pool: PgPool,
}

static INSTANCE: OnceLock<bool> = OnceLock::new();

impl DiscussionController {
    pub async fn new(pool: PgPool) -> Self {
        let repo = Discussion::get_repository(pool.clone());

        let ctrl = Self { repo, pool };
        let arc_ctrl = Arc::new(ctrl.clone());
        if INSTANCE.get().is_none() {
            let res = INSTANCE.set(true);
            if let Err(e) = res {
                tracing::error!("Failed to initialize INSTANCE on {e:?}");
            }
            tokio::spawn(async move {
                let _ = arc_ctrl.notify_discussion_participants().await;
                sleep(Duration::from_secs(1)).await;
            });
        }

        ctrl
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new().with_state(self.clone()))
    }
}

impl DiscussionController {
    async fn notify_discussion_participants(&self) -> Result<()> {
        tracing::debug!("notify discussion participants start");
        let now = chrono::Utc::now().timestamp();

        let sql = r#"
SELECT 
    p.*,
    COALESCE((
        SELECT jsonb_agg(to_jsonb(m))
        FROM (
            SELECT DISTINCT ON (u.id) u.*
            FROM users u
            JOIN discussion_members dm ON u.id = dm.user_id
            WHERE dm.discussion_id = p.id
        ) m
    ), '[]') AS members,
    COALESCE((
        SELECT jsonb_agg(to_jsonb(pu))
        FROM (
            SELECT DISTINCT ON (dp.id) dp.*
            FROM discussion_participants dp
            WHERE dp.discussion_id = p.id
        ) pu
    ), '[]') AS participants
FROM discussions p
WHERE p.started_at BETWEEN $1 AND $2
  AND p.invite_status IS DISTINCT FROM $3
GROUP BY p.id
"#;

        let discussions = sqlx::query(sql)
            .bind(now)
            .bind(now + 60 * 10)
            .bind(DiscussionInviteStatus::Invited as i32)
            .map(Discussion::from)
            .fetch_all(&self.pool)
            .await?;

        let mut result = (0, 0);

        tracing::debug!("notify discussions: {}", discussions.len());

        for discussion in discussions {
            let discussion_id = discussion.id;
            let members = discussion.members;

            let res = self
                .repo
                .update(
                    discussion.id,
                    DiscussionRepositoryUpdateRequest {
                        invite_status: Some(DiscussionInviteStatus::Invited),
                        ..Default::default()
                    },
                )
                .await;

            if let Err(e) = res {
                tracing::error!("Failed to update discussion {}: {:?}", discussion_id, e);
                result.1 += 1;
                continue;
            } else {
                result.0 += 1;
            }

            tracing::debug!("notify discussion members: {}", members.len());

            for member in members {
                tracing::debug!("discussion member: {}", member.id);
                let notification_data = NotificationData::ParticipateDiscussion {
                    discussion_id,
                    user_id: member.id,
                    image_url: None,
                    description: "Your scheduled meeting starts in 10 minutes.".to_string(),
                };

                if let Err(e) = self
                    .send_notification(
                        &self.pool,
                        discussion.space_id,
                        discussion.id,
                        member.id,
                        member.email,
                        notification_data,
                    )
                    .await
                {
                    tracing::error!(
                        "Failed to send InviteDiscussion notification to user {}: {:?}",
                        member.id,
                        e
                    );
                }
            }
        }

        tracing::info!(
            "notification discussions: {} success, {} errors",
            result.0,
            result.1
        );

        Ok(())
    }

    async fn send_notification(
        &self,
        pool: &sqlx::Pool<sqlx::Postgres>,
        space_id: i64,
        discussion_id: i64,
        user_id: i64,
        user_email: String,
        content: NotificationData,
    ) -> Result<Notification> {
        use aws_sdk_sesv2::types::Content;
        use regex::Regex;
        let conf = crate::config::get();

        let notification_type = match content {
            NotificationData::BoostingSpace { .. } => NotificationType::BoostingSpace,
            NotificationData::ConnectNetwork { .. } => NotificationType::ConnectNetwork,
            NotificationData::InviteDiscussion { .. } => NotificationType::InviteDiscussion,
            NotificationData::InviteTeam { .. } => NotificationType::InviteTeam,
            NotificationData::ParticipateDiscussion { .. } => {
                NotificationType::ParticipateDiscussion
            }
            NotificationData::None => NotificationType::Unknown,
        };

        let email_regex = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();

        if email_regex.is_match(&user_email) {
            let _ = send_email(
                user_email,
        Content::builder()
                    .data("Alert Meeting Start")
                    .build()
                    .unwrap(),
            Content::builder()
                    .data(format!("Your scheduled meeting starts in 10 minutes.\nAccess link: {}/spaces/{}/discussions/{}", conf.base_url, space_id, discussion_id))
                    .build()
                    .unwrap(),
            )
            .await
            .map_err(|e| {
                tracing::error!("Email Send Error: {:?}", e);
                Error::SESServiceError(e.to_string())
            })?;
        }

        let repo = Notification::get_repository(pool.clone());
        repo.insert(user_id, content, notification_type, false)
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert notification: {:?}", e);
                Error::DatabaseException(e.to_string())
            })
    }
}
