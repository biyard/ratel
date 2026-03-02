use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct FoundUserResponse {
    pub pk: String,
    pub nickname: String,
    pub username: String,
    pub profile_url: String,
}
