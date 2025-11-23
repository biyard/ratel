use crate::{features::migration::MigrationDataType, *};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct Migration {
    pub pk: MigrationDataType,
    pub sk: String,

    pub created_at: i64,
}

impl Migration {
    pub fn new(doc: MigrationDataType, version: i32) -> Self {
        let created_at = chrono::Utc::now().timestamp_millis();

        Self {
            pk: doc,
            sk: version.to_string(),
            created_at,
        }
    }
}
