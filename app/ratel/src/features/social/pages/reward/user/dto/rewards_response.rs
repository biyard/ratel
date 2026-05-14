use super::super::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct RewardsResponse {
    pub project_name: String,
    pub token_symbol: String,
    pub month: String,
    pub total_points: i64,
    pub points: i64,
    pub monthly_token_supply: i64,
    #[serde(default)]
    pub chain_id: Option<u64>,
    #[serde(default)]
    pub contract_address: Option<String>,
}
