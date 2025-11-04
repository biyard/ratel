use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct BillingKeyPaymentResponse {
    pub payment: Payment,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub paid_at: String,
    pub pg_tx_id: String,
}
