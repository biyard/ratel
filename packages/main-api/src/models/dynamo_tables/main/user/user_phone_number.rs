use crate::types::*;
use bdk::prelude::*;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserPhoneNumber {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(name = "find_by_phone_number", prefix = "PHONE", index = "gsi1", pk)]
    pub phone_number: String,
}

impl UserPhoneNumber {
    pub fn new(pk: Partition, phone_number: String) -> Self {
        let sk = EntityType::UserPhoneNumber;

        Self {
            pk,
            sk,
            phone_number,
        }
    }
}
