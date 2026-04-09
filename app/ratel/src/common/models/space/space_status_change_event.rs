use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceStatusChangeEvent {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,

    pub space_pk: Partition,
    pub old_status: Option<SpaceStatus>,
    pub new_status: SpaceStatus,
}

#[cfg(feature = "server")]
impl SpaceStatusChangeEvent {
    pub fn new(
        space_pk: Partition,
        old_status: Option<SpaceStatus>,
        new_status: SpaceStatus,
    ) -> Self {
        let id = uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        Self {
            pk: Partition::SpaceStatusChangeEvent(id.clone()),
            sk: EntityType::SpaceStatusChangeEvent(id),
            created_at: crate::common::utils::time::get_now_timestamp_millis(),
            space_pk,
            old_status,
            new_status,
        }
    }
}
