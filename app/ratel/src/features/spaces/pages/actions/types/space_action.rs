use super::SpaceActionType;
use crate::features::spaces::pages::actions::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
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

    pub user_participated: bool,
    pub credits: u64,
    pub prerequisite: bool,

    // Populated for Discussion actions only; mirrors `SpacePost.comments`.
    #[serde(default)]
    pub comment_count: Option<i64>,

    #[serde(default)]
    pub status: Option<SpaceActionStatus>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default = "default_deps_met")]
    pub dependencies_met: bool,
}

fn default_deps_met() -> bool {
    true
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
            user_participated: false,
            credits: action.credits,
            prerequisite: action.prerequisite,
            comment_count: None,
            status: action.status,
            depends_on: action.depends_on,
            dependencies_met: true,
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
            // Arena owns the discussion viewer. Legacy `Route::DiscussionActionPage`
            // has been removed — callers outside the arena (e.g. the admin
            // `/actions` list) redirect to the space index and let the user
            // open the discussion from the quest dashboard.
            SpaceActionType::TopicDiscussion => Route::SpaceIndexPage {
                space_id: space_id.clone(),
            },
            SpaceActionType::Follow => Route::FollowActionPage {
                space_id: space_id.clone(),
                follow_id: self.action_id.clone().into(),
            },
            SpaceActionType::Quiz => Route::QuizActionPage {
                space_id: space_id.clone(),
                quiz_id: self.action_id.clone().into(),
            },
            SpaceActionType::Meet => Route::MeetActionPage {
                space_id: space_id.clone(),
                meet_id: self.action_id.clone().into(),
            },
        }
    }
}
