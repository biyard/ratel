use super::dto::AdminPaymentDetail;
use crate::models::dynamo_tables::main::user::User;
use crate::services::portone::PaymentItem;
use crate::types::{EntityType, ListItemsResponse, Pagination, Partition};
use crate::*;
use std::collections::{HashMap, HashSet};

const PAGE_SIZE: i32 = 10;

fn parse_user_pk(payment: &PaymentItem) -> Option<String> {
    payment.customer.id.strip_suffix("##PAYMENT").map(String::from)
}

fn parse_partition(user_pk_str: &str) -> Option<Partition> {
    user_pk_str.parse().ok()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PaymentBookmark {
    page: i32,
    page_size: i32,
    total_count: i64,
    total_pages: i32,
}

/// List all payments (ServiceAdmin only)
///
/// Returns payment history from Portone with user info from DynamoDB
pub async fn list_all_payments_handler(
    State(AppState { dynamo, portone, .. }): State<AppState>,
    Query(Pagination { bookmark }): Query<Pagination>,
) -> Result<Json<ListItemsResponse<AdminPaymentDetail>>> {
    let cli = &dynamo.client;

    let page = if let Some(ref bookmark_str) = bookmark {
        match serde_json::from_str::<PaymentBookmark>(bookmark_str) {
            Ok(bm) => bm.page,
            Err(_) => 0,
        }
    } else {
        0
    };

    let payment_list = portone.list_payments(page, PAGE_SIZE).await?;

    // 1. Pre-parse all user_pk strings
    let payment_user_pks: Vec<Option<String>> = payment_list
        .items
        .iter()
        .map(parse_user_pk)
        .collect();

    // 2. Extract unique user keys for batch get
    let mut seen = HashSet::new();
    let user_keys: Vec<(Partition, EntityType)> = payment_user_pks
        .iter()
        .filter_map(|pk| pk.as_ref())
        .filter(|pk_str| seen.insert(pk_str.to_string()))
        .filter_map(|pk_str| parse_partition(pk_str).map(|p| (p, EntityType::User)))
        .collect();

    // 3. Batch get all users
    let users: HashMap<String, User> = if !user_keys.is_empty() {
        User::batch_get(cli, user_keys)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|u| (u.pk.to_string(), u))
            .collect()
    } else {
        HashMap::new()
    };

    // 4. Build response with user info
    let items: Vec<AdminPaymentDetail> = payment_list
        .items
        .into_iter()
        .zip(payment_user_pks.iter())
        .map(|(payment, user_pk_opt)| {
            let mut detail: AdminPaymentDetail = payment.into();

            if let Some(user_pk_str) = user_pk_opt {
                if let Some(user) = users.get(user_pk_str) {
                    detail = detail.with_user(user.email.clone(), user.display_name.clone());
                }
            }

            detail
        })
        .collect();

    // 5. Create bookmark with pagination info
    let page_info = payment_list.page;
    let total_pages = ((page_info.total_count as i32) + PAGE_SIZE - 1) / PAGE_SIZE;

    let bookmark = serde_json::to_string(&PaymentBookmark {
        page: page_info.number,
        page_size: PAGE_SIZE,
        total_count: page_info.total_count,
        total_pages,
    })?;

    Ok(Json(ListItemsResponse {
        items,
        bookmark: Some(bookmark),
    }))
}
