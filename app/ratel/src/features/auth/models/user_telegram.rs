use crate::features::auth::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UserTelegram {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(name = "find_by_telegram_id", prefix = "TELEGRAM", index = "gsi1", pk)]
    pub telegram_id: i64,
    pub telegram_raw: String,
}

impl UserTelegram {
    pub fn new(pk: Partition, telegram_id: i64, telegram_raw: String) -> Self {
        let sk = EntityType::UserTelegram;

        Self {
            pk,
            sk,
            telegram_id,
            telegram_raw,
        }
    }
}
