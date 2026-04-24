use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::actions::SpaceAction;
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct QuizResponse {
    pub quiz_id: SpaceQuizEntityType,
    pub created_at: i64,
    pub updated_at: i64,
    pub retry_count: i64,
    pub pass_score: i64,
    pub title: String,
    pub description: String,
    pub user_response_count: i64,
    pub questions: Vec<Question>,
    #[serde(default)]
    pub files: Vec<File>,
    #[serde(default)]
    pub my_response: Option<Vec<Answer>>,
    #[serde(default)]
    pub my_score: Option<i64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub attempt_count: i64,
    /// Only populated for participants once the quiz is effectively over
    /// (passed, retries exhausted, or ended_at elapsed). Hidden while the
    /// user can still attempt the quiz so the correct answers don't leak.
    #[serde(default)]
    pub correct_answers: Option<Vec<QuizCorrectAnswer>>,
    pub space_action: crate::features::spaces::pages::actions::models::SpaceAction,
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
            retry_count: quiz.retry_count,
            pass_score: quiz.pass_score,
            title: String::new(),
            description: String::new(),
            user_response_count: quiz.user_response_count,
            questions: quiz.questions,
            files: quiz.files,
            my_response: None,
            my_score: None,
            passed: None,
            attempt_count: 0,
            correct_answers: None,
            space_action: SpaceAction::default(),
        }
    }
}
