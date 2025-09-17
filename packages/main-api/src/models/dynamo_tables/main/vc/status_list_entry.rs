use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct StatusListEntry {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "CRED", name = "find_by_credential", index = "gsi1", pk)]
    pub credential_id: String,
    
    #[dynamo(prefix = "INDEX", name = "find_by_index", index = "gsi2", pk)]
    pub list_index: i64,
    #[dynamo(prefix = "STATUS", index = "gsi2", sk)]
    pub status: String,

    pub reason: Option<String>,
    pub changed_by: String,
}

impl StatusListEntry {
    pub fn new(
        list_id: String,
        credential_id: String,
        list_index: i64,
        status: String,
        changed_by: String,
        reason: Option<String>,
    ) -> Self {
        let pk = Partition::StatusList(list_id);
        let sk = EntityType::StatusListEntry(credential_id.clone());
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            credential_id,
            list_index,
            status,
            reason,
            changed_by,
        }
    }
}
