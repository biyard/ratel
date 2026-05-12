use crate::features::auth::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

// Provider enum is now in crate::common::types::oauth_provider

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UserOAuth {
    pub pk: Partition,

    pub sk: EntityType,

    #[dynamo(
        name = "find_by_provider_and_uid",
        prefix = "PROVIDER",
        index = "gsi1",
        pk
    )]
    pub provider: OauthProvider,
    #[dynamo(index = "gsi1", sk)]
    pub uid: String,
}

impl UserOAuth {
    pub fn new(pk: Partition, provider: OauthProvider, uid: String) -> Self {
        let sk = EntityType::UserOAuth;

        Self {
            pk,
            sk,
            provider,
            uid,
        }
    }
}
