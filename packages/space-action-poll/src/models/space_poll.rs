use common::utils::time::get_now_timestamp_millis;

use crate::*;

use super::Question;
use crate::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePoll {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub topic: String,       // Poll Title
    pub description: String, // Poll Description

    pub user_response_count: i64, // Participants count

    pub started_at: i64,
    pub ended_at: i64,

    pub response_editable: bool, // Whether users can edit their responses

    #[serde(default)]
    pub questions: Vec<Question>, // Questions in the survey

    #[serde(default)]
    pub total_score: i64,
    #[serde(default)]
    pub total_point: i64,
}

#[cfg(feature = "server")]
impl SpacePoll {
    pub fn new(space_pk: SpacePartition) -> crate::Result<Self> {
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpacePoll(uuid::Uuid::now_v7().to_string());

        let now = get_now_timestamp_millis();

        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_response_count: 0,
            response_editable: false,
            started_at: now,
            ended_at: now + 7 * 24 * 60 * 60 * 1000, // Default to 7 days later
            topic: String::new(),
            description: String::new(),
            questions: Vec::new(),
            total_point: 0,
            total_score: 0,
        })
    }
}
