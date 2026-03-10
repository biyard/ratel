use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct QuizAnswerResponse {
    pub quiz_id: SpaceQuizEntityType,
    pub space_id: SpacePartition,
    pub created_at: i64,
    pub updated_at: i64,
    pub answers: Vec<QuizCorrectAnswer>,
}

#[cfg(feature = "server")]
impl From<SpaceQuizAnswer> for QuizAnswerResponse {
    fn from(answer: SpaceQuizAnswer) -> Self {
        let quiz_id: SpaceQuizEntityType = match answer.sk {
            EntityType::SpaceQuizAnswer(id) => id.into(),
            _ => SpaceQuizEntityType::default(),
        };
        let space_id: SpacePartition = match answer.space_pk {
            Partition::Space(id) => id.into(),
            _ => SpacePartition::default(),
        };

        Self {
            quiz_id,
            space_id,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            answers: answer.answers,
        }
    }
}
