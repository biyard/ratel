use super::dto::{AdminPaymentDetail, AdminPaymentListResponse};
use crate::models::dynamo_tables::main::user::User;
use crate::services::portone::PaymentItem;
use crate::types::{EntityType, Partition};
use crate::*;
use axum::{
    Json,
    extract::{Query, State},
};
use bdk::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ListPaymentsQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
}

fn default_page() -> i32 {
    0
}

fn default_page_size() -> i32 {
    10
}

/// Parse user_pk from Portone customer.id
/// Format: "USER#{uuid}##PAYMENT" -> "USER#{uuid}"
fn parse_user_pk(payment: &PaymentItem) -> Option<String> {
    payment.customer.id.strip_suffix("##PAYMENT").map(String::from)
}

/// Extract UUID from user_pk string
/// Format: "USER#{uuid}" -> Partition::User(uuid)
fn parse_partition(user_pk_str: &str) -> Option<Partition> {
    user_pk_str
        .strip_prefix("USER#")
        .map(|uuid| Partition::User(uuid.to_string()))
}

/// List all payments (ServiceAdmin only)
///
/// Returns payment history from Portone with user info from DynamoDB
pub async fn list_all_payments_handler(
    State(AppState { dynamo, portone, .. }): State<AppState>,
    Query(query): Query<ListPaymentsQuery>,
) -> Result<Json<AdminPaymentListResponse>> {
    let cli = &dynamo.client;

    let payment_list = portone.list_payments(query.page, query.page_size).await?;

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
        .iter()
        .zip(payment_user_pks.iter())
        .map(|(payment, user_pk_opt)| {
            let mut detail = AdminPaymentDetail::from(payment);

            if let Some(user_pk_str) = user_pk_opt {
                if let Some(user) = users.get(user_pk_str) {
                    if let Partition::User(user_id) = &user.pk {
                        detail = detail.with_user(user_id.clone(), user.email.clone(), user.display_name.clone());
                    }
                }
            }

            detail
        })
        .collect();

    Ok(Json(AdminPaymentListResponse {
        items,
        page: payment_list.page,
    }))
}
