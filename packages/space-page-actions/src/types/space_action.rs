use super::SpaceActionType;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
// #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceAction {
    pub action_type: SpaceActionType,
    pub title: String,
    pub description: String,

    pub created_at: i64,
    pub updated_at: i64,

    pub total_score: Option<i64>,
    pub total_point: Option<i64>,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
}

impl From<SpacePoll> for SpaceAction {
    fn from(poll: SpacePoll) -> Self {
        Self {
            action_type: SpaceActionType::Poll,
            title: poll.topic,
            description: poll.description,
            created_at: poll.created_at,
            updated_at: poll.updated_at,
            total_score: Some(poll.total_score),
            total_point: Some(poll.total_point),
            started_at: Some(poll.started_at),
            ended_at: Some(poll.ended_at),
        }
    }
}
