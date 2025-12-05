use crate::types::*;
use bdk::prelude::*;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct PhoneVerification {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(
        prefix = "PHONE#VERIFICATION",
        name = "find_by_phone",
        index = "gsi1",
        pk
    )]
    #[dynamo(
        prefix = "PHONE#CODE",
        name = "find_by_phone_and_code",
        index = "gsi2",
        pk
    )]
    pub phone: String,
    #[dynamo(index = "gsi2", sk)]
    pub value: String,
    pub expired_at: i64,
    pub attempt_count: i32,
}

impl PhoneVerification {
    pub fn new(phone: String, value: String, expired_at: i64) -> Self {
        let pk = Partition::Phone(phone.clone());
        let sk = EntityType::PhoneVerification;
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            phone,
            created_at,
            value,
            expired_at,
            attempt_count: 0,
        }
    }
}
