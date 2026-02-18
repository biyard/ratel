use crate::*;

// Provider enum is now in common::types::oauth_provider

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserOAuth {
    pub pk: Partition,

    pub sk: EntityType,

    #[dynamo(
        name = "find_by_provider_and_uid",
        prefix = "PROVIDER",
        index = "gsi1",
        pk
    )]
    pub provider: Provider,
    #[dynamo(index = "gsi1", sk)]
    pub uid: String,
}

impl UserOAuth {
    pub fn new(pk: Partition, provider: Provider, uid: String) -> Self {
        let sk = EntityType::UserOAuth;

        Self {
            pk,
            sk,
            provider,
            uid,
        }
    }
}
