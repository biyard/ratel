//! Idempotency ledger for Launchpad point deductions. The Biyard console
//! `exchange_points` has no idempotency key, so a retried Launchpad
//! `deduct` would double-spend without this guard.

#![cfg(feature = "server")]

use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::common::*;
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct LaunchpadDeduction {
    pub pk: Partition,
    pub sk: EntityType,

    pub idempotency_key: String,
    pub company_user_key: String,
    pub point_amount: i64,
    pub brand_tx_id: String,
    pub remaining_points: i64,
    pub created_at: i64,
}

impl LaunchpadDeduction {
    pub fn new(
        company_user_key: &str,
        idempotency_key: &str,
        point_amount: i64,
        brand_tx_id: &str,
        remaining_points: i64,
    ) -> Self {
        Self {
            pk: Partition::User(company_user_key.to_string()),
            sk: EntityType::LaunchpadDeduction(idempotency_key.to_string()),
            idempotency_key: idempotency_key.to_string(),
            company_user_key: company_user_key.to_string(),
            point_amount,
            brand_tx_id: brand_tx_id.to_string(),
            remaining_points,
            created_at: get_now_timestamp_millis(),
        }
    }
}
