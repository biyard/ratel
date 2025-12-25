use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TokenResponse {
    pub project_id: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: i64,
    pub circulating_supply: i64,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
