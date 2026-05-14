use super::super::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TeamRewardsResponse {
    pub project_name: String,
    pub token_symbol: String,
    pub month: String,
    pub total_points: i64,
    pub team_points: i64,
    pub monthly_token_supply: i64,
}
