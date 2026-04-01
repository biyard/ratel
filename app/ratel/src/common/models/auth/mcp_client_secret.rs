use crate::common::*;

/// Stores a per-user MCP client secret for authenticating MCP tool calls.
///
/// - pk: USER#<user_id>
/// - sk: McpClientSecret
/// - gsi1 pk: MCS#<secret_value> (for lookup by secret)
#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct McpClientSecret {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "MCS", name = "find_by_secret", index = "gsi1", pk)]
    pub secret: String,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl McpClientSecret {
    pub fn new(user_pk: Partition) -> Self {
        let secret = uuid::Uuid::now_v7().to_string();
        let now = chrono::Utc::now().timestamp_millis();

        Self {
            pk: user_pk,
            sk: EntityType::McpClientSecret,
            secret,
            created_at: now,
        }
    }
}
