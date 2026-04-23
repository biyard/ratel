use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Translate)]
pub enum MeetMode {
    #[default]
    #[translate(ko = "예약", en = "Scheduled")]
    Scheduled,
    #[translate(ko = "즉시 시작", en = "Instant")]
    Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[dynamo(prefix = "SM")]
pub struct SpaceMeet {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub mode: MeetMode,
    pub start_time: i64,
    pub duration_min: i32,
}

#[cfg(feature = "server")]
impl SpaceMeet {
    pub fn new(space_pk: SpacePartition) -> Result<Self> {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceMeet(uuid::Uuid::new_v4().to_string());
        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            mode: MeetMode::Scheduled,
            start_time: now,
            duration_min: 60,
        })
    }

    pub fn can_edit(role: &SpaceUserRole) -> Result<()> {
        if role.is_admin() {
            Ok(())
        } else {
            Err(crate::common::Error::NoPermission)
        }
    }
}
