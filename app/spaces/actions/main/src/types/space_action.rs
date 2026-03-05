use super::SpaceActionType;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
// #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceAction {
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
}

impl From<(SpacePoll, bool)> for SpaceAction {
    fn from((poll, user_participated): (SpacePoll, bool)) -> Self {
        let action_id = poll.sk.to_string();
        Self {
            action_id,
            action_type: SpaceActionType::Poll,
            title: poll.title,
            description: poll.description,
            created_at: poll.created_at,
            updated_at: poll.updated_at,
            total_score: Some(poll.total_score),
            total_point: Some(poll.total_point),
            started_at: Some(poll.started_at),
            ended_at: Some(poll.ended_at),
            user_participated,
        }
    }
}

#[cfg(feature = "server")]
impl From<(space_action_discussion::SpacePost, SpaceUserRole)> for SpaceAction {
    fn from((post, role): (space_action_discussion::SpacePost, SpaceUserRole)) -> Self {
        let action_id = post.sk.to_string();
        Self {
            user_participated: post.can_participate(&role).is_ok(),
            action_id,
            action_type: SpaceActionType::TopicDiscussion,
            title: post.title,
            description: String::new(),
            created_at: post.created_at,
            updated_at: post.updated_at,
            total_score: None,
            total_point: None,
            started_at: Some(post.started_at),
            ended_at: Some(post.ended_at),
        }
    }
}

use space_common::types::route::{
    space_action_discussion, space_action_poll, space_action_quiz, space_action_subscription,
};
impl SpaceAction {
    pub fn get_url(&self, space_id: &SpacePartition) -> String {
        match self.action_type {
            SpaceActionType::Poll => space_action_poll(space_id, &self.action_id.clone().into()),
            SpaceActionType::TopicDiscussion => {
                space_action_discussion(space_id, &self.action_id.clone().into())
            }
            SpaceActionType::Subscription => space_action_subscription(space_id),
            SpaceActionType::Quiz => space_action_quiz(space_id),
        }
    }
}
