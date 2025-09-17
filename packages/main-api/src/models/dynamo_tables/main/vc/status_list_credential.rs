use bdk::prelude::*;
use crate::types::*;
use base64::Engine;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct StatusListCredential {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "PURPOSE", name = "find_by_purpose", index = "gsi1", pk)]
    pub purpose: String,
    #[dynamo(prefix = "SIZE", name = "find_by_size", index = "gsi2", pk)]
    pub size: i64,
    #[dynamo(prefix = "VERSION", index = "gsi2", sk)]
    pub version: i64,

    pub status_list_jwt: String,
    pub bitstring_base64: String,
    pub next_index: i64,
}

impl StatusListCredential {
    pub fn new(
        list_id: String,
        purpose: String,
        size: i64,
        status_list_jwt: String,
        bitstring_compressed: Vec<u8>,
    ) -> Self {
        let pk = Partition::StatusList(list_id);
        let sk = EntityType::StatusListCredential;
        let now = chrono::Utc::now().timestamp_micros();
        let bitstring_base64 = base64::engine::general_purpose::STANDARD.encode(&bitstring_compressed);

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            purpose,
            size,
            version: 1,
            status_list_jwt,
            bitstring_base64,
            next_index: 0,
        }
    }
}
