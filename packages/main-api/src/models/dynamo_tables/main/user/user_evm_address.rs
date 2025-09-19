use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserEvmAddress {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(name = "find_by_evm", prefix = "EVM", index = "gsi1", pk)]
    pub evm_address: String,
}

impl UserEvmAddress {
    pub fn new(pk: Partition, evm_address: String) -> Self {
        let sk = EntityType::UserEvmAddress;

        Self {
            pk,
            sk,
            evm_address,
        }
    }
}
