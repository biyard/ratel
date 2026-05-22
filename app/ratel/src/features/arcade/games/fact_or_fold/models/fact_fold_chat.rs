use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Roadmap §FR-11 (영구 보존) — chat is part of the durable round
/// record. v1 SSE fan-out is driven *by* this row's INSERT event
/// rather than a separate broadcast (design doc § A12 reversed).
///
/// One row per chat message. pk groups by round so a single pk
/// query returns the whole transcript; sk uses a time-sortable
/// uuid_v7 so the prefix query yields chronological order.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldChatMessage {
    pub pk: Partition,  // Partition::FactFold(round_id)
    pub sk: EntityType, // EntityType::FactFoldChat(msg_id)

    pub created_at: i64,
    pub updated_at: i64,

    pub author_pk: Partition,
    /// 80-char cap enforced by the controller (roadmap §FR-25).
    pub text: String,
    /// Millis-since-epoch when the server accepted the message.
    /// Mirrors created_at; kept as an explicit field so the wire
    /// format (RoomChannel payload) doesn't have to leak the DDB
    /// timestamp naming.
    pub sent_at: i64,
}

#[cfg(feature = "server")]
impl FactFoldChatMessage {
    pub fn keys(round_id: &str, msg_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFold(round_id.to_string()),
            EntityType::FactFoldChat(msg_id.to_string()),
        )
    }

    pub fn new(round_id: &str, author_pk: Partition, text: String) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let msg_id = uuid::Uuid::now_v7().to_string();
        let (pk, sk) = Self::keys(round_id, &msg_id);
        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            author_pk,
            text,
            sent_at: now,
        }
    }

    pub fn id(&self) -> Option<String> {
        match &self.sk {
            EntityType::FactFoldChat(id) => Some(id.clone()),
            _ => None,
        }
    }
}
