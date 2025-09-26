use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct EmailVerification {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(
        prefix = "EMAIL#VERIFICATION",
        name = "find_by_email",
        index = "gsi1",
        pk
    )]
    #[dynamo(
        prefix = "EMAIL#CODE",
        name = "find_by_email_and_code",
        index = "gsi2",
        pk
    )]
    pub email: String,
    #[dynamo(index = "gsi2", sk)]
    pub value: String,
    pub expired_at: i64,
    pub attempt_count: i32,
}

impl EmailVerification {
    pub fn new(email: String, value: String, expired_at: i64) -> Self {
        let pk = Partition::Email(email.clone());
        let sk = EntityType::EmailVerification;
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            email,
            created_at,
            value,
            expired_at,
            attempt_count: 0,
        }
    }
}
