use crate::features::notification::Notification;
use crate::*;
use crate::{AppState, models::user::User, types::notification_status::NotificationStatus};
use aide::NoApi;
use aws_sdk_dynamodb::types::AttributeValue;
use axum::{Json, extract::State};
use bdk::prelude::*;
use std::collections::HashMap;

#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(tag = "notification", rename_all = "snake_case")]
pub enum UpdateMyNotificationsStatusRequest {
    NotificationId { id: String },
    NotificationTime { time: i64 },
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct UpdateMyNotificationsStatusResponse {
    pub updated: usize,
}

pub async fn update_my_notifications_status_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<UpdateMyNotificationsStatusRequest>,
) -> Result<Json<UpdateMyNotificationsStatusResponse>> {
    match req {
        UpdateMyNotificationsStatusRequest::NotificationId { id } => {
            let pk = Partition::Notification(user.pk.to_string());
            let sk = EntityType::Notification(id);

            Notification::updater(pk, sk)
                .with_status(NotificationStatus::Read)
                .execute(&dynamo.client)
                .await?;

            Ok(Json(UpdateMyNotificationsStatusResponse { updated: 1 }))
        }

        UpdateMyNotificationsStatusRequest::NotificationTime { time } => {
            const MAX_UPDATE_PER_CALL: usize = 200;
            const PAGE_SIZE: i32 = 25;

            let mut updated_count = 0usize;
            let mut bookmark: Option<String> = None;

            loop {
                let mut opt = Notification::opt().limit(PAGE_SIZE);
                if let Some(bm) = bookmark.as_ref() {
                    opt = opt.bookmark(bm.clone());
                }

                opt = opt.sk(NotificationStatus::Unread.to_string());

                let (items, next_bookmark) = Notification::find_by_user_notifications_by_status(
                    &dynamo.client,
                    &user.pk,
                    opt,
                )
                .await?;

                if items.is_empty() {
                    break;
                }

                for n in items.into_iter() {
                    if n.created_at > time {
                        continue;
                    }

                    Notification::updater(n.pk.clone(), n.sk.clone())
                        .with_status(NotificationStatus::Read)
                        .execute(&dynamo.client)
                        .await?;

                    updated_count += 1;
                    if updated_count >= MAX_UPDATE_PER_CALL {
                        break;
                    }
                }

                if updated_count >= MAX_UPDATE_PER_CALL {
                    break;
                }

                if let Some(bm) = next_bookmark {
                    bookmark = Some(bm);
                } else {
                    break;
                }
            }

            Ok(Json(UpdateMyNotificationsStatusResponse {
                updated: updated_count,
            }))
        }
    }
}
