use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Singleton pointer row. Tracks the *current waiting Round* so the
/// matching service can find it in O(1) instead of querying every
/// Round entity. When the round fills up, this pointer is cleared
/// and the next join creates a fresh Round (which then takes the
/// pointer).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldLobby {
    pub pk: Partition,  // Partition::FactFoldLobbySingleton
    pub sk: EntityType, // EntityType::FactFoldLobby

    pub created_at: i64,
    pub updated_at: i64,

    /// `Some` while a round is accepting joins; `None` between
    /// rounds.
    pub current_round_id: Option<String>,
}

#[cfg(feature = "server")]
impl FactFoldLobby {
    pub fn keys() -> (Partition, EntityType) {
        (Partition::FactFoldLobbySingleton, EntityType::FactFoldLobby)
    }

    /// Read the singleton, returning a default empty lobby when the
    /// row doesn't exist yet (first deploy).
    pub async fn get_or_default(cli: &aws_sdk_dynamodb::Client) -> crate::common::Result<Self> {
        let (pk, sk) = Self::keys();
        let row = FactFoldLobby::get(cli, &pk, Some(sk.clone())).await?;
        Ok(row.unwrap_or_else(|| {
            let now = crate::common::utils::time::get_now_timestamp_millis();
            Self {
                pk,
                sk,
                created_at: now,
                updated_at: now,
                current_round_id: None,
            }
        }))
    }
}
