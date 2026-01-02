use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct MonthlyPointSummary {
    pub month: String,
    pub balance: i64,
    pub total_earned: i64,
    pub total_spent: i64,
    pub is_current_month: bool,
    pub project_total_points: i64,
    pub monthly_token_supply: i64,
}
