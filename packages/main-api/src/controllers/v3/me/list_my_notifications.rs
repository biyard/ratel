use crate::features::notification::Notification;
use crate::features::notification::NotificationQueryOption;
use crate::features::notification::NotificationResponse;
use crate::*;
use crate::{
    AppState,
    models::user::User,
    types::{
        list_items_query::ListItemsQuery, list_items_response::ListItemsResponse,
        notification_status::NotificationStatus,
    },
};
use aide::NoApi;
use axum::{
    Json,
    extract::{Query, State},
};
use bdk::prelude::*;

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct ListMyNotificationsQuery {
    pub bookmark: Option<String>,
    pub since: Option<i64>,
    pub status: Option<NotificationStatus>,
}

pub async fn list_my_notifications_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(ListMyNotificationsQuery {
        bookmark,
        since,
        status,
    }): Query<ListMyNotificationsQuery>,
) -> Result<Json<ListItemsResponse<NotificationResponse>>> {
    tracing::debug!(
        "Handling list_my_notifications: bookmark={:?}, since={:?}, status={:?}",
        bookmark,
        since,
        status,
    );

    let (items, next_bookmark) = if let Some(st) = status {
        let mut opt = NotificationQueryOption::builder();

        if let Some(bm) = bookmark {
            opt = opt.bookmark(bm);
        }
        opt = opt.limit(50);
        let opt = opt.sk(st.to_string());

        let (mut items, bookmark) =
            Notification::find_by_user_notifications_by_status(&dynamo.client, &user.pk, opt)
                .await?;

        if let Some(since_ts) = since {
            items.retain(|n| n.created_at >= since_ts);
        }

        (items, bookmark)
    } else if let Some(since_ts) = since {
        let mut opt = NotificationQueryOption::builder();

        if let Some(bm) = bookmark {
            opt = opt.bookmark(bm);
        }
        opt = opt.limit(50);

        Notification::list_by_user_since(&dynamo.client, &user.pk, since_ts, opt).await?
    } else {
        let mut opt = NotificationQueryOption::builder();

        if let Some(bm) = bookmark {
            opt = opt.bookmark(bm);
        }
        opt = opt.limit(50);

        Notification::find_by_user_notifications(&dynamo.client, &user.pk, opt).await?
    };

    let response_items: Vec<NotificationResponse> =
        items.into_iter().map(NotificationResponse::from).collect();

    Ok(Json(ListItemsResponse {
        items: response_items,
        bookmark: next_bookmark,
    }))
}
