use super::SpaceActionType;
use crate::macros::DynamoEntity;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceAction {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    pub description: String,

    pub score: Option<u16>,
    pub point: Option<u16>,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
}

#[cfg(feature = "server")]
use common::utils::time::get_now_timestamp_millis;

#[cfg(feature = "server")]
impl SpaceAction {
    pub fn key(
        space_pk: SpacePartition,
        action_type: SpaceActionType,
        action_key: Option<String>, //
    ) -> (Partition, EntityType) {
        // let uid = if let Some(now) = now {
        //     let secs = (now / 1000) as u64;
        //     let nanos = ((now % 1000) * 1_000_000) as u32;
        //     let ts = uuid::Timestamp::from_unix(uuid::NoContext, secs, nanos);
        //     uuid::Uuid::new_v7(ts).to_string()
        // } else {
        //     uuid::Uuid::now_v7().to_string()
        // };
        let pk: Partition = space_pk.into();
        let sk: EntityType =
            EntityType::SpaceAction(action_type.to_string(), action_key.unwrap_or_default());
        (pk, sk)
    }

    pub fn new(
        space_pk: SpacePartition,
        action_type: SpaceActionType,
        action_key: Option<String>,

        title: String,
        description: String,
        score: Option<u16>,
        point: Option<u16>,
        started_at: Option<i64>,
        ended_at: Option<i64>,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let (pk, sk) = Self::key(space_pk, action_type, action_key);

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            title,
            description,
            score,
            point,
            started_at,
            ended_at,
            ..Default::default()
        }
    }
}
