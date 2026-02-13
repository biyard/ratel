use crate::types;
use crate::*;
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct Session {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub data: String,
    pub expired_at: i64,
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
pub use tower_sessions::Session as TowerSession;
