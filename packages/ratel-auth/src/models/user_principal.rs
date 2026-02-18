use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserPrincipal {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(name = "find_by_principal", prefix = "PRINCIPAL", index = "gsi1", pk)]
    pub principal: String,
}

impl UserPrincipal {
    pub fn new(pk: Partition, principal: String) -> Self {
        let sk = EntityType::UserPrincipal;

        Self { pk, sk, principal }
    }
}
