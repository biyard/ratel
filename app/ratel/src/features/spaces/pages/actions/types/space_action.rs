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
    pub quiz_score: Option<i64>,
    pub quiz_total_score: Option<i64>,
    pub quiz_passed: Option<bool>,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,

    pub user_participated: bool,
    pub credits: u64,
    pub prerequisite: bool,
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
            quiz_score: None,
            quiz_total_score: None,
            quiz_passed: None,
            started_at: Some(action.started_at),
            ended_at: Some(action.ended_at),
            user_participated: false,
            credits: action.credits,
            prerequisite: action.prerequisite,
        }
    }
}

impl SpaceActionSummary {
    pub fn get_url(&self, space_id: &SpacePartition) -> Route {
        match self.action_type {
            SpaceActionType::Poll => Route::PollActionPage {
                space_id: space_id.clone(),
                poll_id: self.action_id.clone().into(),
            },
            SpaceActionType::TopicDiscussion => Route::DiscussionActionPage {
                space_id: space_id.clone(),
                discussion_id: self.action_id.clone().into(),
            },
            SpaceActionType::Follow => Route::FollowActionPage {
                space_id: space_id.clone(),
                follow_id: self.action_id.clone().into(),
            },
            SpaceActionType::Quiz => Route::QuizActionPage {
                space_id: space_id.clone(),
                quiz_id: self.action_id.clone().into(),
            },
        }
    }
}
