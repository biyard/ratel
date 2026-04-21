use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::*;

use crate::common::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceAction {
    pub pk: CompositePartition<SpacePartition, String>,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "SA", name = "find_by_space", index = "gsi1", pk)]
    pub space_pk: Partition,

    pub title: String,
    pub description: String,
    pub space_action_type: SpaceActionType,
    pub prerequisite: bool,

    // Internal GSI sort key — always mirrors `created_at` so the
    // `find_by_space` GSI stays sorted by creation order. The value is
    // never exposed through API DTOs or UI.
    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub started_at: i64,

    pub credits: u64,
    pub total_points: u64,

    #[serde(default)]
    pub activity_score: i64,
    #[serde(default)]
    pub additional_score: i64,

    #[serde(default)]
    pub status: Option<SpaceActionStatus>,

    #[serde(default)]
    pub depends_on: Vec<String>,
}

#[cfg(feature = "server")]
impl SpaceAction {
    pub fn new(
        space_id: SpacePartition,
        action_id: String,
        space_action_type: SpaceActionType,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let space_pk: Partition = space_id.clone().into();

        Self {
            pk: CompositePartition(space_id, action_id),
            sk: EntityType::SpaceAction,
            space_pk,
            title: String::new(),
            description: String::new(),
            space_action_type,
            prerequisite: false,
            created_at: now,
            updated_at: now,
            started_at: now,
            credits: 0,
            total_points: 0,
            activity_score: 0,
            additional_score: 0,
            status: Some(SpaceActionStatus::Designing),
            depends_on: Vec::new(),
        }
    }
}
