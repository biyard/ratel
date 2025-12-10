use crate::*;

use super::CustomerRequest;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleBillingKeyRequest {
    pub payment: BillingKeyPaymentRequest,
    pub time_to_pay: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct BillingKeyPaymentRequest {
    pub store_id: String,
    pub billing_key: String,
    pub channel_key: String,
    pub order_name: String,
    pub customer: CustomerRequest,
    pub amount: PaymentAmountInput,
    pub currency: String,
    pub locale: Option<Locale>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAmountInput {
    pub total: i64,
    pub tax_free: Option<i64>,
    pub vat: Option<i64>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    #[default]
    Usd,
    Krw,
}

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
)]
pub enum Locale {
    #[default]
    EnUs,
    KoKr,
}
