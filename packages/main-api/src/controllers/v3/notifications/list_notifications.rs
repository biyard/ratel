use crate::{
    features::notification::{ListNotificationsResponse, Notification, NotificationQueryOption, NotificationResponse},
    types::{list_items_query::ListItemsQuery, Pagination},
    *,
};
use aide::NoApi;
use axum::{extract::{Query, State}, http::StatusCode, Json};
use bdk::prelude::*;

pub async fn list_notifications_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(Pagination { bookmark }): ListItemsQuery,
) -> std::result::Result<Json<ListNotificationsResponse>, Error> {
    let user_pk = &user.pk;

    // Get all notifications for the user, ordered by created_at descending
    let mut opt = NotificationQueryOption::builder()
        .limit(100)
        .scan_index_forward(false);

    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (notifications, bookmark) =
        Notification::find_by_user_notifications(&dynamo.client, user_pk.to_string(), opt)
            .await?;

    let items: Vec<NotificationResponse> = notifications.into_iter().map(|n| n.into()).collect();

    Ok(Json(ListNotificationsResponse {
        items,
        bookmark,
    }))
}
