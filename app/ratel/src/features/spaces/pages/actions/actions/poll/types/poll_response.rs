use crate::features::spaces::pages::actions::actions::poll::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PollResponse {
    pub sk: EntityType,
    pub created_at: i64,
    pub updated_at: i64,
    pub started_at: i64,
    pub ended_at: i64,
    #[serde(alias = "topic")]
    pub title: String,
    pub description: String,
    pub response_editable: bool,
    pub user_response_count: i64,
    pub questions: Vec<Question>,
    pub my_response: Option<Vec<Answer>>,
    pub status: PollStatus,
    pub default: bool,
}

#[cfg(feature = "server")]
impl From<SpacePoll> for PollResponse {
    fn from(poll: SpacePoll) -> Self {
        Self {
            sk: poll.sk.clone(),
            started_at: poll.started_at,
            ended_at: poll.ended_at,
            title: poll.title.clone(),
            description: poll.description.clone(),
            response_editable: poll.response_editable,
            user_response_count: poll.user_response_count,
            created_at: poll.created_at,
            updated_at: poll.updated_at,
            questions: poll.questions.clone(),
            status: poll.status(),
            default: poll.is_default_poll(),
            my_response: None,
        }
    }
}
