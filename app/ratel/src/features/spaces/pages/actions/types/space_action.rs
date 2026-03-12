use super::SpaceActionType;
use crate::features::spaces::pages::actions::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
// #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceActionSummary {
    pub action_id: String,
    pub action_type: SpaceActionType,
    pub title: String,
    pub description: String,

    pub created_at: i64,
    pub updated_at: i64,

    pub total_score: Option<i64>,
    pub total_point: Option<i64>,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,

    pub user_participated: bool,
    pub credits: u64,
}

impl From<crate::features::spaces::pages::actions::models::SpaceAction> for SpaceActionSummary {
    fn from(action: crate::features::spaces::pages::actions::models::SpaceAction) -> Self {
        let action_id = action.pk.1;
        Self {
            action_id,
            action_type: action.space_action_type,
            title: action.title,
            description: action.description,
            created_at: action.created_at,
            updated_at: action.updated_at,
            total_score: None,
            total_point: None,
            started_at: Some(action.started_at),
            ended_at: Some(action.ended_at),
            user_participated: false,
            credits: action.credits,
        }
    }
}

use crate::features::spaces::space_common::types::route::{
    space_action_discussion, space_action_follow, space_action_poll, space_action_quiz,
};
impl SpaceActionSummary {
    pub fn get_url(&self, space_id: &SpacePartition) -> String {
        match self.action_type {
            SpaceActionType::Poll => space_action_poll(space_id, &self.action_id.clone().into()),
            SpaceActionType::TopicDiscussion => {
                space_action_discussion(space_id, &self.action_id.clone().into())
            }
            SpaceActionType::Follow => space_action_follow(space_id),
            SpaceActionType::Quiz => space_action_quiz(space_id, &self.action_id.clone().into()),
        }
    }
}
