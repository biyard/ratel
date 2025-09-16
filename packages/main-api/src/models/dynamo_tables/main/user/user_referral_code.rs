use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserReferralCode {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(
        name = "find_by_referral_code",
        prefix = "REFERRAL",
        index = "gsi1",
        pk
    )]
    pub referral_code: String,
}

impl UserReferralCode {
    pub fn new(pk: Partition, referral_code: String) -> Self {
        let sk = EntityType::UserReferralCode;

        Self {
            pk,
            sk,
            referral_code,
        }
    }
}
