use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserProfileResponse {
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserDetailResponse {
    pub user: UserProfileResponse,
    pub email: String,
    pub evm_address: Option<String>,
}
