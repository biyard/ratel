use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TokenBalanceResponse {
    pub project_id: String,
    pub meta_user_id: String,
    pub balance: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
