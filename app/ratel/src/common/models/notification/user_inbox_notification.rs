use crate::common::*;

pub const INBOX_TTL_DAYS: i64 = 90;
pub const UNREAD_SENTINEL: &str = "R";

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserInboxNotification {
    #[dynamo(
        prefix = "UIN",
        index = "gsi1",
        name = "find_inbox_unread_by_user",
        pk
    )]
    pub pk: Partition, // User(user_id)

    pub sk: EntityType, // UserInboxNotification(uuid_v7)

    pub created_at: i64,

    /// Sparse GSI sort key. `"U#{created_at}"` while unread, `"R"` when read —
    /// entries with `"R"` are filtered out of the GSI query to implement cheap
    /// unread-only listing.
    #[dynamo(index = "gsi1", sk)]
    pub unread_created_at: String,

    pub is_read: bool,

    pub kind: InboxKind,
    pub payload: InboxPayload,

    /// DynamoDB TTL field (epoch seconds). `created_at_ms/1000 + 90*86400`.
    pub expires_at: i64,
}

#[cfg(feature = "server")]
impl UserInboxNotification {
    pub fn new(recipient_pk: Partition, payload: InboxPayload) -> Self {
        let uid = uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        let now_ms = crate::common::utils::time::get_now_timestamp_millis();
        let expires_at = (now_ms / 1000) + INBOX_TTL_DAYS * 86_400;

        Self {
            pk: recipient_pk,
            sk: EntityType::UserInboxNotification(uid),
            created_at: now_ms,
            unread_created_at: format!("U#{now_ms:020}"),
            is_read: false,
            kind: payload.kind(),
            payload,
            expires_at,
        }
    }
}
