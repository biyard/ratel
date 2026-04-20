use crate::common::*;

pub const DEDUP_TTL_DAYS: i64 = 7;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct InboxDedupMarker {
    #[dynamo(prefix = "INBOX_DEDUP", pk)]
    pub pk: Partition, // User(user_id)

    pub sk: EntityType, // InboxDedupMarker("{kind_prefix}#{source_hash}")

    pub created_at: i64,

    /// DynamoDB TTL field (epoch seconds).
    pub expires_at: i64,
}

#[cfg(feature = "server")]
impl InboxDedupMarker {
    pub fn new(recipient_pk: Partition, kind: InboxKind, source_id: &str) -> Self {
        let now_ms = crate::common::utils::time::get_now_timestamp_millis();
        let expires_at = (now_ms / 1000) + DEDUP_TTL_DAYS * 86_400;
        let hash = Self::hash_source(source_id);
        Self {
            pk: recipient_pk,
            sk: EntityType::InboxDedupMarker(format!("{}#{hash}", kind.as_prefix())),
            created_at: now_ms,
            expires_at,
        }
    }

    fn hash_source(source_id: &str) -> String {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        source_id.hash(&mut h);
        format!("{:016x}", h.finish())
    }
}
