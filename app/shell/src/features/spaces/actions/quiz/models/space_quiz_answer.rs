use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::actions::quiz::*;
use crate::features::spaces::actions::quiz::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceQuizAnswer {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub space_pk: Partition,

    #[serde(default)]
    pub answers: Vec<QuizCorrectAnswer>,
}

#[cfg(feature = "server")]
impl SpaceQuizAnswer {
    pub fn new(
        space_id: SpacePartition,
        quiz_id: SpaceQuizEntityType,
        answers: Vec<QuizCorrectAnswer>,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let space_pk: Partition = space_id.into();
        let pk = space_pk.clone();
        let sk = EntityType::SpaceQuizAnswer(quiz_id.to_string());

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            space_pk,
            answers,
        }
    }
}
