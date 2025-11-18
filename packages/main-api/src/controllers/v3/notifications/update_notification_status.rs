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
            let gsi_pk = Notification::compose_gsi2_pk(user.pk.to_string());
            let status_av: AttributeValue =
                serde_dynamo::to_attribute_value(NotificationStatus::Unread)?;

            const MAX_UPDATE_PER_CALL: usize = 200;
            const PAGE_SIZE: i32 = 25;

            let mut updated_count = 0usize;
            let mut last_evaluated_key: Option<HashMap<String, AttributeValue>> = None;

            loop {
                let mut req = dynamo
                    .client
                    .query()
                    .table_name(Notification::table_name())
                    .index_name("gsi2-index")
                    .key_condition_expression("#pk = :pk")
                    .expression_attribute_names("#pk", "gsi2_pk")
                    .expression_attribute_values(":pk", AttributeValue::S(gsi_pk.clone()))
                    .filter_expression("#status = :status AND #created_at <= :before")
                    .expression_attribute_names("#status", "status")
                    .expression_attribute_names("#created_at", "created_at")
                    .expression_attribute_values(":status", status_av.clone())
                    .expression_attribute_values(":before", AttributeValue::N(time.to_string()))
                    .limit(PAGE_SIZE);

                if let Some(ref lek) = last_evaluated_key {
                    req = req.set_exclusive_start_key(Some(lek.clone()));
                }

                let resp = req
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                let raw_items = resp.items.unwrap_or_default();
                if raw_items.is_empty() {
                    break;
                }

                let notifications: Vec<Notification> = serde_dynamo::from_items(raw_items)?;

                for n in notifications.iter() {
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

                if let Some(lek) = resp.last_evaluated_key {
                    last_evaluated_key = Some(lek);
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
