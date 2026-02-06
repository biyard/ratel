use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct PaymentListResponse {
    pub items: Vec<PaymentItem>,
    pub page: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct PaymentItem {
    pub id: String,
    pub status: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    pub order_name: String,
    pub customer: PaymentCustomer,
    pub amount: PaymentAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_key: Option<String>,
}

impl PaymentItem {
    pub fn user_partition(&self) -> Option<crate::types::Partition> {
        self.customer
            .id
            .parse::<crate::types::CompositePartition>()
            .ok()
            .map(|cp| cp.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCustomer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAmount {
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<i64>,
    pub paid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub number: i32,
    pub size: i32,
    pub total_count: i64,
}
