use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct QuizResponse {
    pub quiz_id: SpaceQuizEntityType,
    pub created_at: i64,
    pub updated_at: i64,
    pub started_at: i64,
    pub ended_at: i64,
    pub retry_count: i64,
    pub pass_score: i64,
    pub title: String,
    pub description: String,
    pub user_response_count: i64,
    pub questions: Vec<Question>,
    #[serde(default)]
    pub my_response: Option<Vec<Answer>>,
    #[serde(default)]
    pub my_score: Option<i64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub attempt_count: i64,
}

#[cfg(feature = "server")]
impl From<SpaceQuiz> for QuizResponse {
    fn from(quiz: SpaceQuiz) -> Self {
        let quiz_id: SpaceQuizEntityType = match &quiz.sk {
            EntityType::SpaceQuiz(id) => id.clone().into(),
            _ => SpaceQuizEntityType::default(),
        };
        Self {
            quiz_id,
            created_at: quiz.created_at,
            updated_at: quiz.updated_at,
            started_at: quiz.started_at,
            ended_at: quiz.ended_at,
            retry_count: quiz.retry_count,
            pass_score: quiz.pass_score,
            title: quiz.title,
            description: quiz.description,
            user_response_count: quiz.user_response_count,
            questions: quiz.questions,
            my_response: None,
            my_score: None,
            passed: None,
            attempt_count: 0,
        }
    }
}
