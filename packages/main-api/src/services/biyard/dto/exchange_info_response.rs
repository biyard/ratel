use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ExchangeInfoResponse {
    pub month: String,
    pub user_balance: i64,
    pub total_supplied_points: i64,
    pub monthly_token_supply: i64,
    pub exchange_ratio: f64,
    pub estimated_tokens: f64,
    pub can_exchange: bool,
}
