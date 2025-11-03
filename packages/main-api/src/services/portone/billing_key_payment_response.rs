// controllers/v3/payments/pay_with_billing_key.rs:102: payment response: Object {"payment": Object {"paidAt": String("2025-11-03T11:01:50.08942321Z"), "pgTxId": String("merchantest95110320014910365PsWE0LKRWCC0")}}

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
