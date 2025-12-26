use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct MintTokenRequest {
    pub amount: i64,
}
