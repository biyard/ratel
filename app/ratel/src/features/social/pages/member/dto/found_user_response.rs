use super::super::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct FoundUserResponse {
    pub pk: String,
    pub nickname: String,
    pub username: String,
    pub profile_url: String,
    pub description: String,
}
