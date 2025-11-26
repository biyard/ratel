use crate::features::notification::NotificationResponse;
use crate::types::list_items_response::ListItemsResponse;

pub type ListNotificationsResponse = ListItemsResponse<NotificationResponse>;
