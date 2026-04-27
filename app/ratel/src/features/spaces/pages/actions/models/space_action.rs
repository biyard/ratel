use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::*;

use crate::common::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceAction {
    pub pk: CompositePartition<SpacePartition, String>,
    pub sk: EntityType,

    // Internal GSI sort key; mirrors `created_at`. Not exposed in API/UI.
    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "SA", name = "find_by_space", index = "gsi1", pk)]
    pub space_pk: Partition,

    pub title: String,
    pub description: String,
    pub space_action_type: SpaceActionType,
    pub prerequisite: bool,

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
            credits: 0,
            total_points: 0,
            activity_score: 0,
            additional_score: 0,
            status: Some(SpaceActionStatus::Designing),
            depends_on: Vec::new(),
        }
    }
}

impl SpaceAction {
    /// Build the absolute participant-facing deep link for this action. Used
    /// by inbox + email notifications. In-app callers (Dioxus `Link`) can
    /// keep using `SpaceActionSummary::get_url` which returns a `Route`
    /// directly.
    pub fn get_cta_url(&self) -> String {
        let space_id = &self.pk.0;
        let action_id = self.pk.1.clone();
        let route = match self.space_action_type {
            SpaceActionType::Poll => Route::PollActionPage {
                space_id: space_id.clone(),
                poll_id: action_id.into(),
            },
            SpaceActionType::TopicDiscussion => Route::SpaceIndexPage {
                space_id: space_id.clone(),
            },
            SpaceActionType::Follow => Route::FollowActionPage {
                space_id: space_id.clone(),
                follow_id: action_id.into(),
            },
            SpaceActionType::Quiz => Route::QuizActionPage {
                space_id: space_id.clone(),
                quiz_id: action_id.into(),
            },
            SpaceActionType::Meet => Route::MeetActionPage {
                space_id: space_id.clone(),
                meet_id: action_id.into(),
            },
        };
        format!("https://ratel.foundation{}", route)
    }
}
