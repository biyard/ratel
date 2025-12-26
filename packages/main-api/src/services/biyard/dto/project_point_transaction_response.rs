use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ProjectPointTransactionResponse {
    pub meta_user_id: String,
    pub month: String,
    pub transaction_type: String,
    pub amount: i64,
    pub target_user_id: Option<String>,
    pub description: Option<String>,
    pub created_at: i64,
}
