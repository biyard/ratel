use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct CancelPaymentResponse {
    pub cancellation: PaymentCancellation,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCancellation {
    pub status: String,
    pub id: String,
    pub total_amount: i64,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    pub requested_at: String,
}
