use crate::*;
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]

pub struct AwardPointRequest {
    pub tx_type: String,
    pub to: String,
    pub amount: i64,
    pub description: String,
    pub month: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]

pub struct AwardPointResponse {
    pub month: String,
    pub transaction_id: String,
}

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
