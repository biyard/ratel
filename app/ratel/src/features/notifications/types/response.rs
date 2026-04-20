use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct InboxNotificationResponse {
    pub id: UserInboxNotificationEntityType,
    pub kind: InboxKind,
    pub payload: InboxPayload,
    pub is_read: bool,
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl From<crate::common::models::notification::UserInboxNotification>
    for InboxNotificationResponse
{
    fn from(n: crate::common::models::notification::UserInboxNotification) -> Self {
        Self {
            id: match n.sk {
                EntityType::UserInboxNotification(id) => UserInboxNotificationEntityType(id),
                _ => UserInboxNotificationEntityType(String::new()),
            },
            kind: n.kind,
            payload: n.payload,
            is_read: n.is_read,
            created_at: n.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UnreadCountResponse {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MarkAllReadResponse {
    pub affected: i64,
    pub has_more: bool,
}
