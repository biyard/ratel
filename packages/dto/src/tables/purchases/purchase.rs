use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(table = purchases)]
pub struct Purchase {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users)]
    pub user_id: i64,
    #[api_model(type = INTEGER, indexed)]
    #[serde(default)]
    pub status: PurchaseStatus,
    #[serde(default)]
    pub payment_id: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum PurchaseStatus {
    #[default]
    InProgress = 1,
    Purchased = 2,
    Refunded = 3,
}
