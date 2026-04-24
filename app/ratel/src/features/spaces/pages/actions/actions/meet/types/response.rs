use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MeetResponse {
    pub pk: SpacePartition,
    pub sk: SpaceMeetEntityType,
    pub mode: MeetMode,
    pub start_time: i64,
    pub duration_min: i32,
    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub space_action: SpaceAction,
}

#[cfg(feature = "server")]
impl From<SpaceMeet> for MeetResponse {
    fn from(m: SpaceMeet) -> Self {
        let pk: SpacePartition = m.pk.into();
        let sk: SpaceMeetEntityType = m.sk.into();
        Self {
            pk,
            sk,
            mode: m.mode,
            start_time: m.start_time,
            duration_min: m.duration_min,
            created_at: m.created_at,
            updated_at: m.updated_at,
            space_action: SpaceAction::default(),
        }
    }
}
