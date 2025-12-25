use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TransactPointRequest {
    pub tx_type: String,
    pub to: Option<String>,
    pub from: Option<String>,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TransactPointResponse {
    pub transaction_id: String,
    pub month: String,
    pub meta_user_id: String,
    pub transaction_type: String,
    pub amount: i64,
}

// Legacy type alias for backward compatibility
pub type AwardPointResponse = TransactPointResponse;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct UserPointTransactionResponse {
    pub month: String,
    pub transaction_type: String,
    pub amount: i64,
    pub target_user_id: Option<String>,
    pub description: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct UserPointBalanceResponse {
    pub month: String,
    pub balance: i64,
    pub total_earned: i64,
    pub total_spent: i64,
    pub updated_at: i64,
}
