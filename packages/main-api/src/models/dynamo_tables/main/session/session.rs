use bdk::prelude::*;

use crate::types::{EntityType, Partition};

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct Session {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub data: String,
    pub expired_at: i64,
}

impl Session {
    pub fn new(id: String, expired_at: i64, data: String) -> Self {
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk: Partition::Session(id),
            sk: EntityType::Session,
            created_at: now,
            updated_at: now,
            data,
            expired_at,
        }
    }
}
