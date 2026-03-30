use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RewardsResponse {
    pub project_name: String,
    pub token_symbol: String,
    pub month: String,
    pub total_points: i64,
    pub points: i64,
    pub monthly_token_supply: i64,
}
