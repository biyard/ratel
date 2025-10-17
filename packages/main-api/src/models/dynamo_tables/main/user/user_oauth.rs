use crate::types::*;
use bdk::prelude::*;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
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
