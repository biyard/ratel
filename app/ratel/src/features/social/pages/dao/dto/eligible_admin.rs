use super::super::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct EligibleAdminResponse {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub profile_url: Option<String>,
    pub is_owner: bool,
    pub evm_address: String,
}
