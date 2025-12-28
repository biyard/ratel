use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TransactPointResponse {
    pub transaction_id: String,
    pub month: String,
    pub meta_user_id: String,
    pub transaction_type: String,
    pub amount: i64,
}

pub type AwardPointResponse = TransactPointResponse;
