use crate::aide::OperationIo;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct PurchaseMembershipRequest {
    pub membership_id: String,
    pub payment_method: Option<String>,
    pub transaction_id: Option<String>,
}
