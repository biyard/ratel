use super::dto::AdminPaymentResponse;
use crate::models::dynamo_tables::main::user::User;
use crate::types::{EntityType, ListItemsResponse, Pagination, Partition};
use crate::*;
use std::collections::{HashMap, HashSet};

const PAGE_SIZE: i32 = 10;

/// List all payments (ServiceAdmin only)
///
/// Returns payment history from Portone with user info from DynamoDB
pub async fn list_all_payments_handler(
    State(AppState { dynamo, portone, .. }): State<AppState>,
    Query(Pagination { bookmark }): Query<Pagination>,
) -> Result<Json<ListItemsResponse<AdminPaymentResponse>>> {
    let cli = &dynamo.client;

    let page: i32 = bookmark
        .as_ref()
        .and_then(|b| b.parse().ok())
        .unwrap_or(0);

    let payment_list = portone.list_payments(page, PAGE_SIZE).await?;

    // 1. Parse user partitions from all payments
    let payment_user_partitions: Vec<Option<Partition>> = payment_list
        .items
        .iter()
        .map(|p| p.user_partition())
        .collect();

    // 2. Extract unique user keys for batch get
    let user_keys: Vec<(Partition, EntityType)> = {
        let mut seen = HashSet::new();
        payment_user_partitions
            .iter()
            .filter_map(|p| {
                let partition = p.as_ref()?;
                if seen.insert(partition.to_string()) {
                    Some((partition.clone(), EntityType::User))
                } else {
                    None
                }
            })
            .collect()
    };

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
    let items: Vec<AdminPaymentResponse> = payment_list
        .items
        .into_iter()
        .zip(payment_user_partitions.iter())
        .map(|(payment, user_partition_opt)| {
            let mut detail: AdminPaymentResponse = payment.into();

            if let Some(user_partition) = user_partition_opt {
                if let Some(user) = users.get(&user_partition.to_string()) {
                    detail = detail.with_user(
                        user_partition.to_string(),
                        user.email.clone(),
                        user.display_name.clone(),
                    );
                }
            }

            detail
        })
        .collect();

    // 5. Calculate next bookmark
    let fetched = ((page + 1) * PAGE_SIZE) as i64;
    let next_bookmark = if fetched < payment_list.page.total_count {
        Some((page + 1).to_string())
    } else {
        None
    };

    Ok(Json(ListItemsResponse {
        items,
        bookmark: next_bookmark,
    }))
}
