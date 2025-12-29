use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct UserPointBalanceResponse {
    pub month: String,
    pub balance: i64,
    pub total_earned: i64,
    pub total_spent: i64,
    pub updated_at: i64,
    #[serde(default)]
    pub project_total_points: i64,
    #[serde(default)]
    pub monthly_token_supply: i64,
}
